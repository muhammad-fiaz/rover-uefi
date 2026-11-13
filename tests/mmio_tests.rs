//! Unit and integration tests for MMIO read, write, modify, and hardware memory barriers.

use rover_uefi::arch::common::mmio::{
    MmioReg, and_u8, and_u16, and_u32, and_u64, and_usize, compiler_barrier, memory_barrier,
    modify_u8, modify_u16, modify_u32, modify_u64, modify_usize, or_u8, or_u16, or_u32, or_u64,
    or_usize, read_memory_barrier, read_u8, read_u16, read_u32, read_u64, read_usize,
    write_memory_barrier, write_u8, write_u16, write_u32, write_u64, write_usize,
};

#[test]
fn test_mmio_read_write_u8() {
    let mut storage: u8 = 0x55;
    let addr = &raw mut storage as usize;

    unsafe {
        assert_eq!(read_u8(addr), 0x55);
        write_u8(addr, 0xAA);
        assert_eq!(read_u8(addr), 0xAA);
    }
}

#[test]
fn test_mmio_read_write_u16() {
    let mut storage: u16 = 0x1234;
    let addr = &raw mut storage as usize;

    unsafe {
        assert_eq!(read_u16(addr), 0x1234);
        write_u16(addr, 0xABCD);
        assert_eq!(read_u16(addr), 0xABCD);
    }
}

#[test]
fn test_mmio_read_write_u32() {
    let mut storage: u32 = 0x1234_5678;
    let addr = &raw mut storage as usize;

    unsafe {
        assert_eq!(read_u32(addr), 0x1234_5678);
        write_u32(addr, 0xDEAD_BEEF);
        assert_eq!(read_u32(addr), 0xDEAD_BEEF);
    }
}

#[test]
fn test_mmio_read_write_u64() {
    let mut storage: u64 = 0x1122_3344_5566_7788;
    let addr = &raw mut storage as usize;

    unsafe {
        assert_eq!(read_u64(addr), 0x1122_3344_5566_7788);
        write_u64(addr, 0xAABB_CCDD_EEFF_0011);
        assert_eq!(read_u64(addr), 0xAABB_CCDD_EEFF_0011);
    }
}

#[test]
fn test_mmio_read_write_usize() {
    let mut storage: usize = 0xCAFE;
    let addr = &raw mut storage as usize;

    unsafe {
        assert_eq!(read_usize(addr), 0xCAFE);
        write_usize(addr, 0xBABE);
        assert_eq!(read_usize(addr), 0xBABE);
    }
}

#[test]
fn test_mmio_modify_u8() {
    let mut storage: u8 = 0b1010_1010;
    let addr = &raw mut storage as usize;

    unsafe {
        // clear lower 4 bits, set bit 0 and bit 1: (0b1010_1010 & 0b1111_0000) | 0b0000_0011 = 0b1010_0011
        let prev = modify_u8(addr, 0x0F, 0x03);
        assert_eq!(prev, 0b1010_1010);
        assert_eq!(read_u8(addr), 0b1010_0011);

        assert_eq!(and_u8(addr, 0b1111_0000), 0b1010_0011);
        assert_eq!(read_u8(addr), 0b1010_0000);

        assert_eq!(or_u8(addr, 0b0000_1111), 0b1010_0000);
        assert_eq!(read_u8(addr), 0b1010_1111);
    }
}

#[test]
fn test_mmio_modify_u16() {
    let mut storage: u16 = 0xFF00;
    let addr = &raw mut storage as usize;

    unsafe {
        let prev = modify_u16(addr, 0xF000, 0x00FF);
        assert_eq!(prev, 0xFF00);
        assert_eq!(read_u16(addr), 0x0FFF);

        let _ = and_u16(addr, 0x00FF);
        assert_eq!(read_u16(addr), 0x00FF);

        let _ = or_u16(addr, 0xAA00);
        assert_eq!(read_u16(addr), 0xAAFF);
    }
}

#[test]
fn test_mmio_modify_u32() {
    let mut storage: u32 = 0xFFFF_0000;
    let addr = &raw mut storage as usize;

    unsafe {
        let prev = modify_u32(addr, 0xFF00_0000, 0x0000_00FF);
        assert_eq!(prev, 0xFFFF_0000);
        assert_eq!(read_u32(addr), 0x00FF_00FF);

        let _ = and_u32(addr, 0x00FF_0000);
        assert_eq!(read_u32(addr), 0x00FF_0000);

        let _ = or_u32(addr, 0x1234_5678);
        assert_eq!(read_u32(addr), 0x12FF_5678);
    }
}

#[test]
fn test_mmio_modify_u64() {
    let mut storage: u64 = 0x1111_2222_3333_4444;
    let addr = &raw mut storage as usize;

    unsafe {
        let prev = modify_u64(addr, 0x0000_0000_FFFF_FFFF, 0xAAAA_BBBB);
        assert_eq!(prev, 0x1111_2222_3333_4444);
        assert_eq!(read_u64(addr), 0x1111_2222_0000_0000 | 0xAAAA_BBBB);

        let _ = and_u64(addr, 0xFFFF_0000_0000_0000);
        assert_eq!(read_u64(addr), 0x1111_0000_0000_0000);

        let _ = or_u64(addr, 0x0000_FFFF_0000_0000);
        assert_eq!(read_u64(addr), 0x1111_FFFF_0000_0000);
    }
}

#[test]
fn test_mmio_modify_usize() {
    let mut storage: usize = 0xF0F0;
    let addr = &raw mut storage as usize;

    unsafe {
        let _ = modify_usize(addr, 0xF000, 0x000F);
        assert_eq!(read_usize(addr), 0x00FF);

        let _ = and_usize(addr, 0x000F);
        assert_eq!(read_usize(addr), 0x000F);

        let _ = or_usize(addr, 0x1110);
        assert_eq!(read_usize(addr), 0x111F);
    }
}

#[test]
fn test_mmio_reg_typed_access() {
    let mut buffer: [u32; 4] = [0x10, 0x20, 0x30, 0x40];
    let base_addr = buffer.as_mut_ptr() as usize;

    let reg0 = MmioReg::<u32>::new(base_addr);
    let reg1 = reg0.offset(4);
    let reg2 = reg0.offset(8);
    let reg3 = reg0.offset(12);

    assert_eq!(reg0.address(), base_addr);
    assert_eq!(reg1.address(), base_addr + 4);

    unsafe {
        assert_eq!(reg0.read(), 0x10);
        assert_eq!(reg1.read(), 0x20);
        assert_eq!(reg2.read(), 0x30);
        assert_eq!(reg3.read(), 0x40);

        reg0.write(0x100);
        assert_eq!(reg0.read(), 0x100);

        let _ = reg1.modify(0x0F, 0x0A);
        assert_eq!(reg1.read(), 0x2A);

        let _ = reg2.and(0x00);
        assert_eq!(reg2.read(), 0x00);

        let _ = reg3.or(0x0F);
        assert_eq!(reg3.read(), 0x4F);

        // Bitfield test: mask 0x00FF_0000, shift 16
        reg0.write(0x1234_5678);
        assert_eq!(reg0.read_field(0x00FF_0000, 16), 0x34);
        reg0.write_field(0x00FF_0000, 16, 0xAB);
        assert_eq!(reg0.read(), 0x12AB_5678);
    }
}

#[test]
fn test_memory_barriers_execute_without_panic() {
    compiler_barrier();
    memory_barrier();
    read_memory_barrier();
    write_memory_barrier();
}
