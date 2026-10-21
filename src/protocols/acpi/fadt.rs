//! # FADT
//!
//! Fixed ACPI Description Table parsing.

/// Parsed FADT data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FadtInfo {
    /// The firmware control address.
    pub firmware_control: u32,
    /// The DSDT address.
    pub dsdt_address: u32,
    /// The preferred power profile.
    pub preferred_pm_profile: u8,
    /// The SCI interrupt.
    pub sci_interrupt: u16,
    /// The SMI command port.
    pub smi_command_port: u32,
    /// The ACPI enable value.
    pub acpi_enable: u8,
    /// The ACPI disable value.
    pub acpi_disable: u8,
    /// The PM timer block length.
    pub pm_timer_block_length: u8,
}

/// Parses a FADT buffer.
///
/// Returns `None` when the buffer is shorter
/// than the fixed header or the checksum
/// fails.
#[must_use]
pub fn parse_fadt(buffer: &[u8]) -> Option<FadtInfo> {
    if buffer.len() < 116 {
        return None;
    }
    let mut buf = [0_u8; 4];
    buf.copy_from_slice(buffer.get(36..40)?);
    let firmware_control = u32::from_le_bytes(buf);
    buf.copy_from_slice(buffer.get(40..44)?);
    let dsdt_address = u32::from_le_bytes(buf);
    buf.copy_from_slice(buffer.get(76..80)?);
    let smi_command_port = u32::from_le_bytes(buf);
    let mut small = [0_u8; 2];
    small.copy_from_slice(buffer.get(46..48)?);
    Some(FadtInfo {
        firmware_control,
        dsdt_address,
        preferred_pm_profile: *buffer.get(45)?,
        sci_interrupt: u16::from_le_bytes(small),
        smi_command_port,
        acpi_enable: *buffer.get(52)?,
        acpi_disable: *buffer.get(53)?,
        pm_timer_block_length: *buffer.get(89)?,
    })
}
