//! Known-answer tests for the SHA-3 and SHAKE functions.
use std::fmt::Write as _;

use rover_uefi::protocols::security::sha3::{
    sha3_224, sha3_256, sha3_384, sha3_512, shake128_into, shake256_into,
};

fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

#[test]
fn sha3_known_answers() {
    assert_eq!(
        hex(&sha3_224(b"")),
        "6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7"
    );
    assert_eq!(
        hex(&sha3_256(b"")),
        "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
    );
    assert_eq!(
        hex(&sha3_256(b"abc")),
        "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"
    );
    assert_eq!(
        hex(&sha3_384(b"abc")),
        "ec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25"
    );
    assert_eq!(
        hex(&sha3_512(b"abc")),
        "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0"
    );
}

#[test]
fn shake_known_answers() {
    let mut out = [0_u8; 32];
    assert!(shake128_into(b"", &mut out));
    assert_eq!(
        hex(&out),
        "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26"
    );
    assert!(shake256_into(b"", &mut out));
    assert_eq!(
        hex(&out),
        "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762f"
    );
}

#[test]
fn shake_output_is_a_prefix_of_a_longer_squeeze() {
    let mut long = [0_u8; 64];
    assert!(shake128_into(b"abc", &mut long));
    let mut short = [0_u8; 16];
    assert!(shake128_into(b"abc", &mut short));
    assert_eq!(short, long[..16]);
}
