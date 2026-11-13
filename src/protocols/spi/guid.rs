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

/// The SPI SMM Host Controller protocol `GUID`.
pub const EFI_SPI_SMM_HC_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xE9F0_2217,
    0x2093,
    0x4470,
    [0x8A, 0x54, 0x5C, 0x2C, 0xFF, 0xE7, 0x3E, 0xCB],
);

/// The SPI SMM Configuration protocol `GUID`.
pub const EFI_SPI_SMM_CONFIGURATION_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x995C_6ECA,
    0x171B,
    0x45FD,
    [0xA3, 0xAA, 0xFD, 0x4C, 0x9C, 0x9D, 0xEF, 0x59],
);

/// The SPI SMM NOR Flash protocol `GUID`.
pub const EFI_SPI_SMM_NOR_FLASH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xAAB1_8F19,
    0xFE14,
    0x4666,
    [0x86, 0x04, 0x87, 0xFF, 0x6D, 0x66, 0x2C, 0x9A],
);

/// The extended SPI SMM NOR Flash protocol `GUID`.
pub const SPI_SMM_NOR_FLASH_PROTOCOL_EX_GUID: EfiGuid = EfiGuid::new(
    0x6743_5DED,
    0xCAA9,
    0x4BA5,
    [0xB9, 0x3E, 0x9C, 0xE4, 0x28, 0xD4, 0x94, 0xA2],
);
