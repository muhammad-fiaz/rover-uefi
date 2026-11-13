# Implementation Matrix

Detailed tracking of reference interfaces, specifications, and corresponding native Rust implementations in `rover-uefi`.

| Interface / Component | Specification | EDK II Reference | Rover Location | Test Suite |
|---|---|---|---|---|
| `EfiSystemTable` | UEFI 2.10 §4.3 | `MdePkg/Include/Uefi/UefiSpec.h` | `src/tables/system.rs` | `tests/tables.rs` |
| `EfiBootServices` | UEFI 2.10 §4.4 | `MdePkg/Include/Uefi/UefiSpec.h` | `src/tables/boot.rs` | `tests/tables.rs` |
| `EfiRuntimeServices` | UEFI 2.10 §4.5 | `MdePkg/Include/Uefi/UefiSpec.h` | `src/tables/runtime.rs` | `tests/tables.rs` |
| `EfiDxeServices` (GCD) | PI 1.8 Vol 2 §9 | `MdePkg/Include/Pi/PiDxeCis.h` | `src/protocols/firmware/dxe_services.rs` | `tests/pi_dxe_services.rs` |
| `EfiPeiServices` | PI 1.8 Vol 1 §4 | `MdePkg/Include/Pi/PiPeiCis.h` | `src/protocols/firmware/pei.rs` | `tests/pi_pei_services.rs` |
| `EfiMmSystemTable` | PI 1.8 Vol 4 §4 | `MdePkg/Include/Pi/PiMmCis.h` | `src/protocols/firmware/mm_system_table.rs` | `tests/pi_mm_system_table.rs` |
| DXE Architectural Protocols | PI 1.8 Vol 2 §11 | `MdePkg/Include/Protocol/*Arch*.h` | `src/protocols/firmware/arch_protocols.rs` | `tests/pi_arch_protocols.rs` |
| Core PEI PPIs | PI 1.8 Vol 1 §5 | `MdePkg/Include/Ppi/*.h` | `src/protocols/firmware/ppi/core_ppis.rs` | `tests/pi_ppi_tests.rs` |
| UEFI TLS & Config Protocol | UEFI 2.10 §29.8 | `MdePkg/Include/Protocol/Tls*.h` | `src/protocols/network/tls_protocol.rs` | `tests/network_tls_http_boot.rs` |
| HTTP Boot Callback | UEFI 2.10 §29.7 | `MdePkg/Include/Protocol/HttpBootCallback.h` | `src/protocols/network/http_boot_callback.rs` | `tests/network_tls_http_boot.rs` |
| Storage Security Command | UEFI 2.10 §13.23 | `MdePkg/Include/Protocol/StorageSecurityCommand.h` | `src/protocols/storage/storage_security.rs` | `tests/storage_security.rs` |
| Erase Block Protocol | UEFI 2.10 §13.24 | `MdePkg/Include/Protocol/EraseBlock.h` | `src/protocols/storage/erase_block.rs` | `tests/storage_erase_block.rs` |
| Disk Info Protocol | UEFI 2.10 §13.21 | `MdePkg/Include/Protocol/DiskInfo.h` | `src/protocols/storage/disk_info.rs` | `tests/storage_disk_info.rs` |
| Unicode Collation 1 & 2 | UEFI 2.10 §22.2 | `MdePkg/Include/Protocol/UnicodeCollation.h` | `src/protocols/console/unicode_collation.rs` | `tests/unicode_collation.rs` |
| PKCS#7 Verify Protocol | UEFI 2.10 §32.5 | `SecurityPkg/Include/Protocol/Pkcs7Verify.h` | `src/protocols/security/pkcs7.rs` | `tests/security_pkcs7_verify.rs` |
| CC Measurement Protocol | UEFI 2.10 §38.1 | `MdePkg/Include/Protocol/CcMeasurement.h` | `src/protocols/security/cc_measurement.rs` | `tests/security_cc_measurement.rs` |
| SMBIOS Protocol | UEFI 2.10 §33.1 | `MdePkg/Include/Protocol/Smbios.h` | `src/protocols/smbios/smbios_protocol.rs` | `tests/smbios_protocol.rs` |
| MP Services Protocol | PI 1.8 Vol 2 §13 | `MdePkg/Include/Protocol/MpService.h` | `src/protocols/core/mp_services.rs` | `tests/core_mp_services.rs` |
| CPU I/O 2 Protocol | PI 1.8 Vol 2 §12 | `MdePkg/Include/Protocol/CpuIo2.h` | `src/protocols/core/cpu_io2.rs` | `tests/core_cpu_io2.rs` |
| Driver Diagnostics 1 & 2 | UEFI 2.10 §11.5 | `MdePkg/Include/Protocol/DriverDiagnostics2.h` | `src/protocols/driver/diagnostics.rs` | `tests/driver_model_extensions.rs` |
| Driver Health Protocol | UEFI 2.10 §11.7 | `MdePkg/Include/Protocol/DriverHealth.h` | `src/protocols/driver/health.rs` | `tests/driver_model_extensions.rs` |
| Platform & Bus Overrides | UEFI 2.10 §11.8 | `MdePkg/Include/Protocol/PlatformDriverOverride.h` | `src/protocols/driver/override_protocol.rs` | `tests/driver_model_extensions.rs` |
| Universal Payload (UPL) | UPL Spec 0.8 | `MdeModulePkg/Include/Guid/UniversalPayload.h` | `src/protocols/firmware/universal_payload.rs` | `tests/universal_payload.rs` |
