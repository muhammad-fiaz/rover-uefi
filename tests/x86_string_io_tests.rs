//! Unit tests for x86 port string I/O primitives and strongly typed port wrappers.

use rover_uefi::arch::x86::io::{
    Port8, Port16, Port32, ins_u8, ins_u16, ins_u32, outs_u8, outs_u16, outs_u32,
};

#[test]
fn test_port8_instantiation_and_properties() {
    let p = Port8::new(0x3F8);
    assert_eq!(p.port(), 0x3F8);

    let mut empty_buf = [];
    unsafe {
        p.read_buffer(&mut empty_buf);
        p.write_buffer(&empty_buf);
    }
}

#[test]
fn test_port16_instantiation_and_properties() {
    let p = Port16::new(0x1F0);
    assert_eq!(p.port(), 0x1F0);

    let mut empty_buf = [];
    unsafe {
        p.read_buffer(&mut empty_buf);
        p.write_buffer(&empty_buf);
    }
}

#[test]
fn test_port32_instantiation_and_properties() {
    let p = Port32::new(0xCF8);
    assert_eq!(p.port(), 0xCF8);

    let mut empty_buf = [];
    unsafe {
        p.read_buffer(&mut empty_buf);
        p.write_buffer(&empty_buf);
    }
}

#[test]
fn test_empty_string_io_no_op() {
    unsafe {
        ins_u8(0x80, &mut []);
        outs_u8(0x80, &[]);
        ins_u16(0x80, &mut []);
        outs_u16(0x80, &[]);
        ins_u32(0x80, &mut []);
        outs_u32(0x80, &[]);
    }
}
