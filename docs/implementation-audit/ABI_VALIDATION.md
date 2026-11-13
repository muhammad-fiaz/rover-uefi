# ABI Validation & Binary Compatibility

All firmware-facing functions in `rover-uefi` enforce:
`unsafe extern "efiapi"`

## Validated Layout Invariants

1. **System & Service Tables**:
   - `EfiTableHeader`: 24 bytes, 64-bit aligned.
   - `EfiSystemTable`: Contains accurate offsets for `ConIn`, `ConOut`, `StdErr`, `RuntimeServices`, `BootServices`, and `ConfigurationTable`.
   - `EfiDxeServices`: Signature `0x4553584420494645` ("EFI DXE SERV"), 64-bit GCD memory/IO descriptors.
   - `EfiPeiServices`: Signature `0x5652455320494550` ("PEI SERV"), revision `0x00010050`.
   - `EfiMmSystemTable`: Signature `0x54534d4d` ("MMST"), SMM/MM dispatch and memory pools.

2. **Primitive & Handle Types**:
   - `EfiHandle`: Opaque pointer-sized (`*mut c_void`).
   - `EfiGuid`: 128-bit layout (u32, u16, u16, [u8; 8]).
   - `EfiStatus`: Native machine word (`usize`) with high bit error flag.
   - `EfiTime`: Packed 16-byte structure with timezone and daylight flags.

3. **Calling Convention Compatibility**:
   - Compiles across x86-64 MS ABI (`x86_64-unknown-uefi`), AArch64 standard PCS (`aarch64-unknown-uefi`), and IA-32 Cdecl/regparm.
