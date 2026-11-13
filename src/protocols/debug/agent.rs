//! # Source-Level Debug Agent Transfer Protocol
//!
//! Provides the packet definitions, command codes, CRC16 calculation, and
//! serialization primitives for communication between target firmware debug agent
//! and host debugger.

use crate::base::EfiGuid;

/// Debug Agent HOB and Configuration Table GUID:
/// `{865a5a9b-b85d-474c-8455-65d1be844be2}`
pub const DEBUG_AGENT_GUID: EfiGuid = EfiGuid::new(
    0x865a_5a9b,
    0xb85d,
    0x474c,
    [0x84, 0x55, 0x65, 0xd1, 0xbe, 0x84, 0x4b, 0xe2],
);

/// Starting symbol for Attach packet.
pub const DEBUG_STARTING_SYMBOL_ATTACH: u8 = 0xFA;
/// Starting symbol for Normal debug packet.
pub const DEBUG_STARTING_SYMBOL_NORMAL: u8 = 0xFE;
/// Starting symbol for Compressed debug packet.
pub const DEBUG_STARTING_SYMBOL_COMPRESS: u8 = 0xFC;

/// Upper limit for total debug packet size in bytes.
pub const DEBUG_DATA_UPPER_LIMIT: usize = 0xFF;

/// Debug command flags.
pub const DEBUG_COMMAND_REQUEST: u8 = 0 << 7;
/// Debug command response flag.
pub const DEBUG_COMMAND_RESPONSE: u8 = 1 << 7;

// HOST initiated request commands
/// Command: Reset target system.
pub const DEBUG_COMMAND_RESET: u8 = DEBUG_COMMAND_REQUEST;
/// Command: Continue execution (Go).
pub const DEBUG_COMMAND_GO: u8 = DEBUG_COMMAND_REQUEST | 0x01;
/// Command: Query reason for target break.
pub const DEBUG_COMMAND_BREAK_CAUSE: u8 = DEBUG_COMMAND_REQUEST | 0x02;
/// Command: Set hardware breakpoint register.
pub const DEBUG_COMMAND_SET_HW_BREAKPOINT: u8 = DEBUG_COMMAND_REQUEST | 0x03;
/// Command: Clear hardware breakpoint register.
pub const DEBUG_COMMAND_CLEAR_HW_BREAKPOINT: u8 = DEBUG_COMMAND_REQUEST | 0x04;
/// Command: Single step execution.
pub const DEBUG_COMMAND_SINGLE_STEPPING: u8 = DEBUG_COMMAND_REQUEST | 0x05;
/// Command: Set software breakpoint instruction (`INT3`).
pub const DEBUG_COMMAND_SET_SW_BREAKPOINT: u8 = DEBUG_COMMAND_REQUEST | 0x06;
/// Command: Read target memory range.
pub const DEBUG_COMMAND_READ_MEMORY: u8 = DEBUG_COMMAND_REQUEST | 0x07;
/// Command: Write target memory range.
pub const DEBUG_COMMAND_WRITE_MEMORY: u8 = DEBUG_COMMAND_REQUEST | 0x08;
/// Command: Read I/O space port.
pub const DEBUG_COMMAND_READ_IO: u8 = DEBUG_COMMAND_REQUEST | 0x09;
/// Command: Write I/O space port.
pub const DEBUG_COMMAND_WRITE_IO: u8 = DEBUG_COMMAND_REQUEST | 0x0A;
/// Command: Read processor register.
pub const DEBUG_COMMAND_READ_REGISTER: u8 = DEBUG_COMMAND_REQUEST | 0x0B;
/// Command: Write processor register.
pub const DEBUG_COMMAND_WRITE_REGISTER: u8 = DEBUG_COMMAND_REQUEST | 0x0C;
/// Command: Read all general purpose registers.
pub const DEBUG_COMMAND_READ_ALL_REGISTERS: u8 = DEBUG_COMMAND_REQUEST | 0x0D;
/// Command: Query target architecture mode.
pub const DEBUG_COMMAND_ARCH_MODE: u8 = DEBUG_COMMAND_REQUEST | 0x0E;
/// Command: Read Model Specific Register (MSR).
pub const DEBUG_COMMAND_READ_MSR: u8 = DEBUG_COMMAND_REQUEST | 0x0F;
/// Command: Write Model Specific Register (MSR).
pub const DEBUG_COMMAND_WRITE_MSR: u8 = DEBUG_COMMAND_REQUEST | 0x10;
/// Command: Query or modify debug settings.
pub const DEBUG_COMMAND_SET_DEBUG_SETTING: u8 = DEBUG_COMMAND_REQUEST | 0x11;
/// Command: Get debug protocol revision.
pub const DEBUG_COMMAND_GET_REVISION: u8 = DEBUG_COMMAND_REQUEST | 0x12;
/// Command: Get exception state.
pub const DEBUG_COMMAND_GET_EXCEPTION: u8 = DEBUG_COMMAND_REQUEST | 0x13;
/// Command: Set multi-core processor viewpoint.
pub const DEBUG_COMMAND_SET_VIEWPOINT: u8 = DEBUG_COMMAND_REQUEST | 0x14;
/// Command: Get multi-core processor viewpoint.
pub const DEBUG_COMMAND_GET_VIEWPOINT: u8 = DEBUG_COMMAND_REQUEST | 0x15;
/// Command: Detach debugger from target.
pub const DEBUG_COMMAND_DETACH: u8 = DEBUG_COMMAND_REQUEST | 0x16;
/// Command: Execute CPUID instruction on target.
pub const DEBUG_COMMAND_CPUID: u8 = DEBUG_COMMAND_REQUEST | 0x17;
/// Command: Search memory signature.
pub const DEBUG_COMMAND_SEARCH_SIGNATURE: u8 = DEBUG_COMMAND_REQUEST | 0x18;
/// Command: Halt target CPU.
pub const DEBUG_COMMAND_HALT: u8 = DEBUG_COMMAND_REQUEST | 0x19;

// TARGET initiated request commands
/// Target notify: Initial break occurred.
pub const DEBUG_COMMAND_INIT_BREAK: u8 = DEBUG_COMMAND_REQUEST | 0x3F;
/// Target notify: Breakpoint hit.
pub const DEBUG_COMMAND_BREAK_POINT: u8 = DEBUG_COMMAND_REQUEST | 0x3E;
/// Target notify: Target memory initialized and ready.
pub const DEBUG_COMMAND_MEMORY_READY: u8 = DEBUG_COMMAND_REQUEST | 0x3D;
/// Target notify: Print debug message to host.
pub const DEBUG_COMMAND_PRINT_MESSAGE: u8 = DEBUG_COMMAND_REQUEST | 0x3C;
/// Target notify: Break triggered by attach request.
pub const DEBUG_COMMAND_ATTACH_BREAK: u8 = DEBUG_COMMAND_REQUEST | 0x3B;

// Response status commands
/// Response: Command completed successfully.
pub const DEBUG_COMMAND_OK: u8 = DEBUG_COMMAND_RESPONSE;
/// Response: CRC error or sequence error, request resend.
pub const DEBUG_COMMAND_RESEND: u8 = DEBUG_COMMAND_RESPONSE | 0x01;
/// Response: Command execution aborted.
pub const DEBUG_COMMAND_ABORT: u8 = DEBUG_COMMAND_RESPONSE | 0x02;
/// Response: Multi-packet transfer in progress.
pub const DEBUG_COMMAND_IN_PROGRESS: u8 = DEBUG_COMMAND_RESPONSE | 0x03;
/// Response: Continue sending multi-packet data.
pub const DEBUG_COMMAND_CONTINUE: u8 = DEBUG_COMMAND_RESPONSE | 0x04;
/// Response: Deferred halt acknowledged.
pub const DEBUG_COMMAND_HALT_DEFERRED: u8 = DEBUG_COMMAND_RESPONSE | 0x05;
/// Response: Deferred halt processed.
pub const DEBUG_COMMAND_HALT_PROCESSED: u8 = DEBUG_COMMAND_RESPONSE | 0x06;
/// Response: Command execution timed out.
pub const DEBUG_COMMAND_TIMEOUT: u8 = DEBUG_COMMAND_RESPONSE | 0x07;
/// Response: Requested command is unsupported by target.
pub const DEBUG_COMMAND_NOT_SUPPORTED: u8 = DEBUG_COMMAND_RESPONSE | 0x0F;

/// Debug Packet Header preceding packet payload.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DebugPacketHeader {
    /// Starting symbol (`DEBUG_STARTING_SYMBOL_*`).
    pub start_symbol: u8,
    /// Packet command code.
    pub command: u8,
    /// Total packet length in bytes including header and payload.
    pub length: u8,
    /// Packet sequence counter for resend detection.
    pub sequence_no: u8,
    /// 16-bit CRC checksum over command, length, `sequence_no`, and payload.
    pub crc: u16,
}

/// Target break cause response payload.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DebugDataResponseBreakCause {
    /// Cause code.
    pub cause: u8,
    /// Stop linear address.
    pub stop_address: u64,
}

/// Computes the 16-bit CCITT CRC checksum used by debug agent packets.
#[must_use]
pub fn calculate_debug_crc16(data: &[u8], mut crc: u16) -> u16 {
    for &byte in data {
        crc ^= u16::from(byte) << 8;
        for _ in 0..8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

/// Errors occurring during debug packet serialization or deserialization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebugPacketError {
    /// Buffer is smaller than minimum packet header size (6 bytes).
    BufferTooSmall,
    /// Packet start symbol is unrecognized.
    InvalidStartSymbol,
    /// Packet length is invalid or does not match buffer length.
    InvalidLength,
    /// Packet CRC16 checksum does not match computed value.
    CrcMismatch,
}

/// Encodes a debug packet into `dest` buffer.
///
/// Returns the total bytes written into `dest`.
///
/// # Errors
/// Returns [`DebugPacketError`] if `dest` is too small to contain header and payload.
pub fn encode_debug_packet(
    start_symbol: u8,
    command: u8,
    sequence_no: u8,
    payload: &[u8],
    dest: &mut [u8],
) -> Result<usize, DebugPacketError> {
    let header_size = core::mem::size_of::<DebugPacketHeader>();
    let total_len = header_size + payload.len();
    if total_len > DEBUG_DATA_UPPER_LIMIT || dest.len() < total_len {
        return Err(DebugPacketError::BufferTooSmall);
    }

    #[allow(clippy::cast_possible_truncation)]
    let len_u8 = total_len as u8;

    // CRC is computed over command, length, sequence_no, and payload
    let mut crc = 0_u16;
    let meta = [command, len_u8, sequence_no];
    crc = calculate_debug_crc16(&meta, crc);
    crc = calculate_debug_crc16(payload, crc);

    dest[0] = start_symbol;
    dest[1] = command;
    dest[2] = len_u8;
    dest[3] = sequence_no;
    dest[4..6].copy_from_slice(&crc.to_le_bytes());
    dest[6..total_len].copy_from_slice(payload);

    Ok(total_len)
}

/// Decodes and validates a debug packet from `src` buffer.
///
/// Returns the parsed [`DebugPacketHeader`] and a slice to the verified payload.
///
/// # Errors
/// Returns [`DebugPacketError`] on short buffers, unknown start symbols, length mismatches,
/// or CRC corruption.
pub fn decode_debug_packet(
    src: &[u8],
) -> Result<(DebugPacketHeader, &[u8]), DebugPacketError> {
    let header_size = core::mem::size_of::<DebugPacketHeader>();
    if src.len() < header_size {
        return Err(DebugPacketError::BufferTooSmall);
    }

    let start_symbol = src[0];
    if start_symbol != DEBUG_STARTING_SYMBOL_NORMAL
        && start_symbol != DEBUG_STARTING_SYMBOL_ATTACH
        && start_symbol != DEBUG_STARTING_SYMBOL_COMPRESS
    {
        return Err(DebugPacketError::InvalidStartSymbol);
    }

    let command = src[1];
    let length = src[2] as usize;
    if length < header_size || length > src.len() {
        return Err(DebugPacketError::InvalidLength);
    }

    let sequence_no = src[3];
    let mut crc_raw = [0_u8; 2];
    crc_raw.copy_from_slice(&src[4..6]);
    let crc = u16::from_le_bytes(crc_raw);

    let payload = &src[6..length];

    // Compute expected CRC
    #[allow(clippy::cast_possible_truncation)]
    let meta = [command, length as u8, sequence_no];
    let mut expected_crc = 0_u16;
    expected_crc = calculate_debug_crc16(&meta, expected_crc);
    expected_crc = calculate_debug_crc16(payload, expected_crc);

    if crc != expected_crc {
        return Err(DebugPacketError::CrcMismatch);
    }

    #[allow(clippy::cast_possible_truncation)]
    Ok((
        DebugPacketHeader {
            start_symbol,
            command,
            length: length as u8,
            sequence_no,
            crc,
        },
        payload,
    ))
}
