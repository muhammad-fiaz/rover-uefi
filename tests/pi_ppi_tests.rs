//! Integration tests for PEI Program and Interface (PPI) definitions.

use rover_uefi::protocols::firmware::ppi::*;

#[test]
fn test_core_ppi_guids() {
    assert_ne!(EFI_PEI_PERMANENT_MEMORY_INSTALLED_PPI_GUID.data1, 0);
    assert_ne!(EFI_PEI_MASTER_BOOT_MODE_PEIM_PPI_GUID.data1, 0);
    assert_ne!(EFI_PEI_BOOT_IN_RECOVERY_MODE_PPI_GUID.data1, 0);
    assert_ne!(EFI_DXE_IPL_PPI_GUID.data1, 0);
    assert_ne!(EFI_PEI_CPU_IO_PPI_INSTALLED_GUID.data1, 0);
    assert_ne!(EFI_PEI_FIRMWARE_VOLUME_INFO_PPI_GUID.data1, 0);
    assert_ne!(EFI_PEI_FIRMWARE_VOLUME_INFO2_PPI_GUID.data1, 0);
    assert_ne!(EFI_PEI_RECOVERY_BLOCK_IO_PPI_GUID.data1, 0);
    assert_ne!(EFI_PEI_GRAPHICS_PPI_GUID.data1, 0);
    assert_ne!(EFI_PEI_MP_SERVICES_PPI_GUID.data1, 0);
}

#[test]
fn test_core_ppi_struct_sizes() {
    assert_eq!(
        core::mem::size_of::<EfiDxeIplPpi>(),
        core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<EfiPeiGraphicsPpi>(),
        2 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<EfiPeiMpServicesPpi>(),
        7 * core::mem::size_of::<usize>()
    );
    assert_eq!(
        core::mem::size_of::<EfiPeiCpuIoPpi>(),
        18 * core::mem::size_of::<usize>()
    );
}

#[test]
fn test_cpu_io_width_enum_values() {
    assert_eq!(EfiPeiCpuIoPpiWidth::Uint8 as u32, 0);
    assert_eq!(EfiPeiCpuIoPpiWidth::Uint64 as u32, 3);
    assert_eq!(EfiPeiCpuIoPpiWidth::FifoUint8 as u32, 4);
    assert_eq!(EfiPeiCpuIoPpiWidth::FillUint64 as u32, 11);
}
