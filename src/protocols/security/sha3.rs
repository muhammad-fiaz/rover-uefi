//! # SHA-3, SHAKE, and the Keccak permutation
//!
//! Native implementation of the `Keccak-f[1600]` permutation and of the
//! sponge construction defined by FIPS 202 (SHA-3 Standard: Permutation
//! Based Hash and Extendable-Output Functions).
//!
//! The module provides the four SHA-3 digests (`SHA3-224`, `SHA3-256`,
//! `SHA3-384`, `SHA3-512`) and the two extendable-output functions
//! (`SHAKE128`, `SHAKE256`). Every operation works on caller-provided
//! buffers; the library performs no heap allocation.
//!
//! Each digest type exposes a `DIGEST_SIZE` constant, a streaming
//! hasher whose `finalize` method consumes the hasher, and a one-shot
//! helper function. The SHAKE hashers produce an arbitrarily long
//! output through a squeezer that can be asked for more bytes at any
//! time.
//!
//! The digest of the empty string and of `abc` are the published
//! FIPS 202 examples; the implementation is self-checked by the
//! integration tests against an independent implementation.

/// The number of 64-bit lanes in the `Keccak-f[1600]` state.
const LANE_COUNT: usize = 25;

/// The largest sponge rate, in bytes, supported by this module.
///
/// The largest rate in use is the `168` byte rate of `SHAKE128`.
pub const MAX_RATE: usize = 168;

/// The domain separation byte prefixed to SHA-3 messages.
pub const SHA3_DOMAIN: u8 = 0x06;

/// The domain separation byte prefixed to SHAKE messages.
pub const SHAKE_DOMAIN: u8 = 0x1f;

/// The `Keccak-f[1600]` round constants.
const ROUND_CONSTANTS: [u64; 24] = [
    0x0000_0000_0000_0001,
    0x0000_0000_0000_8082,
    0x8000_0000_0000_808a,
    0x8000_0000_8000_8000,
    0x0000_0000_0000_808b,
    0x0000_0000_8000_0001,
    0x8000_0000_8000_8081,
    0x8000_0000_0000_8009,
    0x0000_0000_0000_008a,
    0x0000_0000_0000_0088,
    0x0000_0000_8000_8009,
    0x0000_0000_8000_000a,
    0x0000_0000_8000_808b,
    0x8000_0000_0000_008b,
    0x8000_0000_0000_8089,
    0x8000_0000_0000_8003,
    0x8000_0000_0000_8002,
    0x8000_0000_0000_0080,
    0x0000_0000_0000_800a,
    0x8000_0000_8000_000a,
    0x8000_0000_8000_8081,
    0x8000_0000_0000_8080,
    0x0000_0000_8000_0001,
    0x8000_0000_8000_8008,
];

/// The rho rotation offsets in bits, indexed by column `x` and row `y`.
const ROTATION_OFFSETS: [[u32; 5]; 5] = [
    [0, 36, 3, 41, 18],
    [1, 44, 10, 45, 2],
    [62, 6, 43, 15, 61],
    [28, 55, 25, 21, 56],
    [27, 20, 39, 8, 14],
];

/// The `Keccak-f[1600]` permutation and its 5 by 5 lane state.
///
/// The state is exposed so that callers can build constructions that
/// are not covered by the SHA-3 and SHAKE helpers of this module. The
/// lane order follows the specification: lane `x + 5 * y` holds the
/// element at column `x` and row `y`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Keccak {
    lanes: [u64; LANE_COUNT],
}

impl Keccak {
    /// Creates the all-zero state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            lanes: [0; LANE_COUNT],
        }
    }

    /// Applies the `Keccak-f[1600]` permutation to the state in place.
    ///
    /// The permutation applies the `theta`, `rho`, `pi`, `chi`, and
    /// `iota` steps to the 24 rounds defined by the specification.
    pub fn permute(&mut self) {
        for round in &ROUND_CONSTANTS {
            // Theta: mix the columns.
            let mut columns = [0_u64; 5];
            for x in 0..5 {
                let mut parity = 0_u64;
                for y in 0..5 {
                    parity ^= self.lane(x, y);
                }
                if let Some(column) = columns.get_mut(x) {
                    *column = parity;
                }
            }
            for x in 0..5 {
                let previous = columns.get((x + 4) % 5).copied().unwrap_or(0);
                let next = columns.get((x + 1) % 5).copied().unwrap_or(0);
                let delta = previous ^ next.rotate_left(1);
                for y in 0..5 {
                    if let Some(lane) = self.lane_mut(x, y) {
                        *lane ^= delta;
                    }
                }
            }

            // Rho and pi: rotate the lanes and move them.
            let mut permuted = [0_u64; LANE_COUNT];
            for x in 0..5 {
                for y in 0..5 {
                    let source = self.lane(x, y);
                    let rotation = ROTATION_OFFSETS
                        .get(x)
                        .and_then(|row| row.get(y))
                        .copied()
                        .unwrap_or(0);
                    let target = y + 5 * ((2 * x + 3 * y) % 5);
                    if let Some(lane) = permuted.get_mut(target) {
                        *lane = source.rotate_left(rotation);
                    }
                }
            }

            // Chi: apply the non-linear step to each row.
            for x in 0..5 {
                for y in 0..5 {
                    let first = permuted.get(x + 5 * y).copied().unwrap_or(0);
                    let second = permuted.get((x + 1) % 5 + 5 * y).copied().unwrap_or(0);
                    let third = permuted.get((x + 2) % 5 + 5 * y).copied().unwrap_or(0);
                    let value = first ^ ((!second) & third);
                    if let Some(lane) = self.lane_mut(x, y) {
                        *lane = value;
                    }
                }
            }

            // Iota: break the symmetry with the round constant.
            if let Some(lane) = self.lanes.get_mut(0) {
                *lane ^= round;
            }
        }
    }

    /// Returns the raw state lanes.
    #[must_use]
    pub const fn lanes(&self) -> &[u64; LANE_COUNT] {
        &self.lanes
    }

    /// Absorbs `data` into the state in whole blocks of `rate` bytes.
    ///
    /// The state is permuted after every absorbed block. Bytes that do
    /// not fill a whole block are not absorbed. The number of absorbed
    /// bytes is returned.
    pub fn absorb(&mut self, rate: usize, data: &[u8]) -> usize {
        if rate == 0 || rate > MAX_RATE {
            return 0;
        }
        let mut consumed = 0_usize;
        while data.len().saturating_sub(consumed) >= rate {
            let block = data.get(consumed..consumed + rate).unwrap_or(&[]);
            self.xor_bytes(0, block);
            self.permute();
            consumed += rate;
        }
        consumed
    }

    /// Squeezes one block of at most `rate` bytes out of the state.
    ///
    /// The number of written bytes is returned.
    pub fn squeeze_block(&mut self, rate: usize, out: &mut [u8]) -> usize {
        if rate == 0 || rate > MAX_RATE {
            return 0;
        }
        let count = out.len().min(rate);
        if let Some(slice) = out.get_mut(..count) {
            self.read_bytes(0, slice);
        }
        count
    }

    /// Returns the lane at column `x` and row `y`, or zero when the
    /// coordinates are out of range.
    fn lane(&self, x: usize, y: usize) -> u64 {
        self.lanes.get(x + 5 * y).copied().unwrap_or(0)
    }

    /// Returns a mutable reference to the lane at column `x` and row
    /// `y`, or `None` when the coordinates are out of range.
    fn lane_mut(&mut self, x: usize, y: usize) -> Option<&mut u64> {
        self.lanes.get_mut(x + 5 * y)
    }

    /// Exclusive-ors `data` into the state starting at byte `offset`.
    fn xor_bytes(&mut self, offset: usize, data: &[u8]) {
        for (index, byte) in data.iter().enumerate() {
            let position = offset + index;
            let lane = position / 8;
            let shift = 8 * (position % 8);
            if let Some(value) = self.lanes.get_mut(lane) {
                *value ^= u64::from(*byte) << shift;
            }
        }
    }

    /// Reads the state bytes starting at `offset` into `out`.
    fn read_bytes(&self, offset: usize, out: &mut [u8]) {
        for (index, byte) in out.iter_mut().enumerate() {
            let position = offset + index;
            let lane = position / 8;
            let shift = 8 * (position % 8);
            let value = self.lanes.get(lane).copied().unwrap_or(0);
            *byte = u8::try_from((value >> shift) & 0xff).unwrap_or(0);
        }
    }
}

impl Default for Keccak {
    /// Creates the all-zero state.
    fn default() -> Self {
        Self::new()
    }
}

/// A streaming sponge hasher over the `Keccak-f[1600]` permutation.
///
/// The hasher absorbs a message and, once finalized, produces the
/// squeezed output. `finalize` consumes the hasher, so it cannot be
/// called twice.
#[derive(Clone, Debug)]
pub struct Sha3Hasher {
    keccak: Keccak,
    buffer: [u8; MAX_RATE],
    buffer_len: usize,
    rate: usize,
    domain: u8,
    total_len: u64,
}

impl Sha3Hasher {
    /// Creates a hasher for the given rate and domain separation byte.
    ///
    /// Returns `None` when the rate is zero or larger than
    /// [`MAX_RATE`].
    #[must_use]
    pub fn new(rate: usize, domain: u8) -> Option<Self> {
        if rate == 0 || rate > MAX_RATE {
            return None;
        }
        Some(Self::with_valid_rate(rate, domain))
    }

    /// Creates a hasher for a rate that is known to be valid.
    fn with_valid_rate(rate: usize, domain: u8) -> Self {
        Self {
            keccak: Keccak::new(),
            buffer: [0; MAX_RATE],
            buffer_len: 0,
            rate,
            domain,
            total_len: 0,
        }
    }

    /// Returns the sponge rate in bytes.
    #[must_use]
    pub const fn rate(&self) -> usize {
        self.rate
    }

    /// Returns the domain separation byte.
    #[must_use]
    pub const fn domain(&self) -> u8 {
        self.domain
    }

    /// Returns the number of bytes absorbed so far.
    #[must_use]
    pub const fn total_len(&self) -> u64 {
        self.total_len
    }

    /// Absorbs `data` into the sponge.
    pub fn update(&mut self, mut data: &[u8]) {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
        while !data.is_empty() {
            let space = self.rate - self.buffer_len;
            let take = space.min(data.len());
            if let (Some(slot), Some(source)) = (
                self.buffer.get_mut(self.buffer_len..self.buffer_len + take),
                data.get(..take),
            ) {
                slot.copy_from_slice(source);
            }
            self.buffer_len += take;
            if let Some(rest) = data.get(take..) {
                data = rest;
            } else {
                data = &[];
            }
            if self.buffer_len == self.rate {
                self.absorb_block();
            }
        }
    }

    /// Finalizes the sponge and returns a squeezer for the output.
    ///
    /// The hasher is consumed, so it cannot be finalized twice.
    #[must_use]
    pub fn finalize(mut self) -> ShakeSqueezer {
        self.pad();
        ShakeSqueezer {
            keccak: self.keccak,
            rate: self.rate,
            position: 0,
        }
    }

    /// Finalizes the sponge and squeezes the output into `out`.
    ///
    /// The number of written bytes is returned. The hasher is
    /// consumed, so it cannot be finalized twice.
    #[must_use]
    pub fn finalize_into(mut self, out: &mut [u8]) -> usize {
        self.pad();
        let mut squeezer = ShakeSqueezer {
            keccak: self.keccak,
            rate: self.rate,
            position: 0,
        };
        squeezer.squeeze_into(out);
        out.len()
    }

    /// Absorbs the buffered block and permutes the state.
    fn absorb_block(&mut self) {
        let block = self.buffer.get(..self.rate).unwrap_or(&[]);
        self.keccak.xor_bytes(0, block);
        self.keccak.permute();
        self.buffer_len = 0;
    }

    /// Applies the domain separation and the `pad10*1` padding.
    fn pad(&mut self) {
        if self.buffer_len == self.rate {
            self.absorb_block();
        }
        if let Some(slot) = self.buffer.get_mut(self.buffer_len) {
            *slot = self.domain;
        }
        self.buffer_len += 1;
        if let Some(rest) = self.buffer.get_mut(self.buffer_len..self.rate) {
            rest.fill(0);
        }
        if let Some(last) = self.buffer.get_mut(self.rate - 1) {
            *last |= 0x80;
        }
        self.buffer_len = self.rate;
        self.absorb_block();
    }
}

/// An extendable-output squeezer.
///
/// A squeezer is produced by finalizing a SHAKE sponge. It yields an
/// arbitrarily long output: every call to [`ShakeSqueezer::squeeze_into`]
/// continues where the previous call stopped.
#[derive(Clone, Debug)]
pub struct ShakeSqueezer {
    keccak: Keccak,
    rate: usize,
    position: usize,
}

impl ShakeSqueezer {
    /// Squeezes output bytes into `out`.
    ///
    /// The output is unbounded, so every requested byte is produced.
    pub fn squeeze_into(&mut self, out: &mut [u8]) {
        let mut written = 0_usize;
        while written < out.len() {
            if self.position == self.rate {
                self.keccak.permute();
                self.position = 0;
            }
            let available = self.rate - self.position;
            let take = available.min(out.len() - written);
            if let Some(slice) = out.get_mut(written..written + take) {
                self.keccak.read_bytes(self.position, slice);
            }
            written += take;
            self.position += take;
        }
    }

    /// Squeezes a fixed-size array of output bytes.
    #[must_use]
    pub fn squeeze_array<const SIZE: usize>(&mut self) -> [u8; SIZE] {
        let mut out = [0_u8; SIZE];
        self.squeeze_into(&mut out);
        out
    }

    /// Returns the sponge rate in bytes.
    #[must_use]
    pub const fn rate(&self) -> usize {
        self.rate
    }
}

/// A streaming SHA3-224 hasher.
#[derive(Clone, Debug)]
pub struct Sha3_224 {
    inner: Sha3Hasher,
}

impl Sha3_224 {
    /// The length of the SHA3-224 digest in bytes.
    pub const DIGEST_SIZE: usize = 28;

    /// The sponge rate in bytes.
    pub const RATE: usize = 144;

    /// Creates a hasher in the initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Sha3Hasher::with_valid_rate(Self::RATE, SHA3_DOMAIN),
        }
    }

    /// Absorbs `data`.
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Finalizes and returns the digest. Consumes the hasher.
    #[must_use]
    pub fn finalize(self) -> [u8; Self::DIGEST_SIZE] {
        let mut out = [0_u8; Self::DIGEST_SIZE];
        let written = self.inner.finalize_into(&mut out);
        if written == Self::DIGEST_SIZE {
            out
        } else {
            [0_u8; Self::DIGEST_SIZE]
        }
    }
}

impl Default for Sha3_224 {
    /// Creates a hasher in the initial state.
    fn default() -> Self {
        Self::new()
    }
}

/// A streaming SHA3-256 hasher.
#[derive(Clone, Debug)]
pub struct Sha3_256 {
    inner: Sha3Hasher,
}

impl Sha3_256 {
    /// The length of the SHA3-256 digest in bytes.
    pub const DIGEST_SIZE: usize = 32;

    /// The sponge rate in bytes.
    pub const RATE: usize = 136;

    /// Creates a hasher in the initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Sha3Hasher::with_valid_rate(Self::RATE, SHA3_DOMAIN),
        }
    }

    /// Absorbs `data`.
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Finalizes and returns the digest. Consumes the hasher.
    #[must_use]
    pub fn finalize(self) -> [u8; Self::DIGEST_SIZE] {
        let mut out = [0_u8; Self::DIGEST_SIZE];
        let written = self.inner.finalize_into(&mut out);
        if written == Self::DIGEST_SIZE {
            out
        } else {
            [0_u8; Self::DIGEST_SIZE]
        }
    }
}

impl Default for Sha3_256 {
    /// Creates a hasher in the initial state.
    fn default() -> Self {
        Self::new()
    }
}

/// A streaming SHA3-384 hasher.
#[derive(Clone, Debug)]
pub struct Sha3_384 {
    inner: Sha3Hasher,
}

impl Sha3_384 {
    /// The length of the SHA3-384 digest in bytes.
    pub const DIGEST_SIZE: usize = 48;

    /// The sponge rate in bytes.
    pub const RATE: usize = 104;

    /// Creates a hasher in the initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Sha3Hasher::with_valid_rate(Self::RATE, SHA3_DOMAIN),
        }
    }

    /// Absorbs `data`.
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Finalizes and returns the digest. Consumes the hasher.
    #[must_use]
    pub fn finalize(self) -> [u8; Self::DIGEST_SIZE] {
        let mut out = [0_u8; Self::DIGEST_SIZE];
        let written = self.inner.finalize_into(&mut out);
        if written == Self::DIGEST_SIZE {
            out
        } else {
            [0_u8; Self::DIGEST_SIZE]
        }
    }
}

impl Default for Sha3_384 {
    /// Creates a hasher in the initial state.
    fn default() -> Self {
        Self::new()
    }
}

/// A streaming SHA3-512 hasher.
#[derive(Clone, Debug)]
pub struct Sha3_512 {
    inner: Sha3Hasher,
}

impl Sha3_512 {
    /// The length of the SHA3-512 digest in bytes.
    pub const DIGEST_SIZE: usize = 64;

    /// The sponge rate in bytes.
    pub const RATE: usize = 72;

    /// Creates a hasher in the initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Sha3Hasher::with_valid_rate(Self::RATE, SHA3_DOMAIN),
        }
    }

    /// Absorbs `data`.
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Finalizes and returns the digest. Consumes the hasher.
    #[must_use]
    pub fn finalize(self) -> [u8; Self::DIGEST_SIZE] {
        let mut out = [0_u8; Self::DIGEST_SIZE];
        let written = self.inner.finalize_into(&mut out);
        if written == Self::DIGEST_SIZE {
            out
        } else {
            [0_u8; Self::DIGEST_SIZE]
        }
    }
}

impl Default for Sha3_512 {
    /// Creates a hasher in the initial state.
    fn default() -> Self {
        Self::new()
    }
}

/// A streaming SHAKE128 extendable-output hasher.
#[derive(Clone, Debug)]
pub struct Shake128 {
    inner: Sha3Hasher,
}

impl Shake128 {
    /// The sponge rate in bytes.
    pub const RATE: usize = 168;

    /// Creates a hasher in the initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Sha3Hasher::with_valid_rate(Self::RATE, SHAKE_DOMAIN),
        }
    }

    /// Absorbs `data`.
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Finalizes the sponge and returns the output squeezer.
    ///
    /// The hasher is consumed, so it cannot be finalized twice.
    #[must_use]
    pub fn finalize(self) -> ShakeSqueezer {
        self.inner.finalize()
    }

    /// Finalizes the sponge and squeezes `out.len()` bytes into `out`.
    pub fn squeeze_into(self, out: &mut [u8]) {
        let mut squeezer = self.finalize();
        squeezer.squeeze_into(out);
    }
}

impl Default for Shake128 {
    /// Creates a hasher in the initial state.
    fn default() -> Self {
        Self::new()
    }
}

/// A streaming SHAKE256 extendable-output hasher.
#[derive(Clone, Debug)]
pub struct Shake256 {
    inner: Sha3Hasher,
}

impl Shake256 {
    /// The sponge rate in bytes.
    pub const RATE: usize = 136;

    /// Creates a hasher in the initial state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Sha3Hasher::with_valid_rate(Self::RATE, SHAKE_DOMAIN),
        }
    }

    /// Absorbs `data`.
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Finalizes the sponge and returns the output squeezer.
    ///
    /// The hasher is consumed, so it cannot be finalized twice.
    #[must_use]
    pub fn finalize(self) -> ShakeSqueezer {
        self.inner.finalize()
    }

    /// Finalizes the sponge and squeezes `out.len()` bytes into `out`.
    pub fn squeeze_into(self, out: &mut [u8]) {
        let mut squeezer = self.finalize();
        squeezer.squeeze_into(out);
    }
}

impl Default for Shake256 {
    /// Creates a hasher in the initial state.
    fn default() -> Self {
        Self::new()
    }
}

/// Computes the SHA3-224 digest of `data` in one call.
#[must_use]
pub fn sha3_224(data: &[u8]) -> [u8; Sha3_224::DIGEST_SIZE] {
    let mut hasher = Sha3_224::new();
    hasher.update(data);
    hasher.finalize()
}

/// Computes the SHA3-256 digest of `data` in one call.
#[must_use]
pub fn sha3_256(data: &[u8]) -> [u8; Sha3_256::DIGEST_SIZE] {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize()
}

/// Computes the SHA3-384 digest of `data` in one call.
#[must_use]
pub fn sha3_384(data: &[u8]) -> [u8; Sha3_384::DIGEST_SIZE] {
    let mut hasher = Sha3_384::new();
    hasher.update(data);
    hasher.finalize()
}

/// Computes the SHA3-512 digest of `data` in one call.
#[must_use]
pub fn sha3_512(data: &[u8]) -> [u8; Sha3_512::DIGEST_SIZE] {
    let mut hasher = Sha3_512::new();
    hasher.update(data);
    hasher.finalize()
}

/// Computes SHAKE128 of `data` into `out`.
///
/// Returns `true` when `out` was filled completely.
#[must_use]
pub fn shake128_into(data: &[u8], out: &mut [u8]) -> bool {
    let mut hasher = Shake128::new();
    hasher.update(data);
    let mut squeezer = hasher.finalize();
    squeezer.squeeze_into(out);
    true
}

/// Computes SHAKE256 of `data` into `out`.
///
/// Returns `true` when `out` was filled completely.
#[must_use]
pub fn shake256_into(data: &[u8], out: &mut [u8]) -> bool {
    let mut hasher = Shake256::new();
    hasher.update(data);
    let mut squeezer = hasher.finalize();
    squeezer.squeeze_into(out);
    true
}

/// Compares two digests without an early exit.
#[must_use]
pub fn digest_eq<const SIZE: usize>(left: &[u8; SIZE], right: &[u8; SIZE]) -> bool {
    let mut difference = 0_u8;
    for (first, second) in left.iter().zip(right.iter()) {
        difference |= first ^ second;
    }
    difference == 0
}
