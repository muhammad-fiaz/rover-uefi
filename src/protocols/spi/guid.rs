//! # SPI protocol identifiers
//!
//! Official SPI protocol `GUID` constants defined in UEFI PI 1.6/1.8 specifications.

use crate::base::EfiGuid;

/// The SPI Configuration protocol `GUID`.
pub const EFI_SPI_CONFIGURATION_GUID: EfiGuid = EfiGuid::new(
    0x85A6_D3E6,
    0xB65B,
    0x4AFC,
    [0xB3, 0x8F, 0xC6, 0xD5, 0x4A, 0xF6, 0xDD, 0xC8],
);

/// The SPI Host Controller protocol `GUID`.
pub const EFI_SPI_HC_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xC74E_5DB2,
    0xFA96,
    0x4AE2,
    [0xB3, 0x99, 0x15, 0x97, 0x7F, 0xE3, 0x00, 0x2D],
);

/// The SPI NOR Flash protocol `GUID`.
pub const EFI_SPI_NOR_FLASH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xB57E_C3FE,
    0xF833,
    0x4BA6,
    [0x85, 0x78, 0x2A, 0x7D, 0x6A, 0x87, 0x44, 0x4B],
);

/// The extended SPI NOR Flash protocol `GUID`.
pub const SPI_NOR_FLASH_PROTOCOL_EX_GUID: EfiGuid = EfiGuid::new(
    0x99DE_F701,
    0x7FF9,
    0x46B5,
    [0x84, 0x52, 0x56, 0x8E, 0xFE, 0x5B, 0xA9, 0x9B],
);
