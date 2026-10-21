//! Tests for platform hand-off blocks, dependency expressions, and IFR.

use rover_uefi::protocols::firmware::depex::{
    DEPEX_AFTER, DEPEX_AND, DEPEX_BEFORE, DEPEX_END, DEPEX_FALSE, DEPEX_NOT, DEPEX_OR, DEPEX_PUSH,
    DEPEX_TRUE, DepexError, DepexIter, DepexToken, validate_dependency_expression,
};
use rover_uefi::protocols::firmware::hob::{
    HOB_TYPE_END_OF_HOB_LIST, HOB_TYPE_FIRMWARE_VOLUME, HOB_TYPE_FIRMWARE_VOLUME2,
    HOB_TYPE_GUID_EXTENSION, HOB_TYPE_HANDOFF, HOB_TYPE_MEMORY_ALLOCATION,
    HOB_TYPE_RESOURCE_DESCRIPTOR, HobError, ResourceType, hob_count, hob_iter, is_valid_hob_list,
    parse_firmware_volume, parse_firmware_volume2, parse_guid_hob, parse_handoff_info_table,
    parse_memory_allocation, parse_resource_descriptor,
};
use rover_uefi::protocols::firmware::ifr::{
    IFR_END, IFR_FORM, IFR_FORM_SET, IfrError, IfrIter, ifr_opcode_count, validate_ifr,
};

fn push_u16(buffer: &mut Vec<u8>, value: u16) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(buffer: &mut Vec<u8>, value: u32) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn push_u64(buffer: &mut Vec<u8>, value: u64) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn make_hob(hob_type: u16, payload: &[u8]) -> Vec<u8> {
    let mut padded = payload.to_vec();
    while !(padded.len() + 8).is_multiple_of(8) {
        padded.push(0);
    }
    let total = 8 + padded.len();
    let mut buffer = Vec::with_capacity(total);
    push_u16(&mut buffer, hob_type);
    push_u16(&mut buffer, u16::try_from(total).unwrap_or(u16::MAX));
    push_u32(&mut buffer, 0);
    buffer.extend_from_slice(&padded);
    buffer
}

fn guid(seed: u8) -> [u8; 16] {
    let mut value = [seed; 16];
    value[15] = 0x5A;
    value
}

#[test]
fn hob_list_iterates_typed_blocks() {
    let mut stream = Vec::new();

    let mut handoff = Vec::new();
    push_u32(&mut handoff, 9);
    push_u32(&mut handoff, 1);
    push_u64(&mut handoff, 0x1000_0000);
    push_u64(&mut handoff, 0x0000_0000);
    push_u64(&mut handoff, 0x0FFF_0000);
    push_u64(&mut handoff, 0x0001_0000);
    push_u64(&mut handoff, 0x0FFF_1234);
    stream.extend_from_slice(&make_hob(HOB_TYPE_HANDOFF, &handoff));

    let mut memory = Vec::new();
    push_u64(&mut memory, 0x2000_0000);
    push_u64(&mut memory, 0x0001_0000);
    push_u32(&mut memory, 7);
    push_u32(&mut memory, 0);
    stream.extend_from_slice(&make_hob(HOB_TYPE_MEMORY_ALLOCATION, &memory));

    let mut resource = Vec::new();
    resource.extend_from_slice(&guid(0xA1));
    push_u32(&mut resource, 7);
    push_u32(&mut resource, 0x0F);
    push_u64(&mut resource, 0x8000_0000);
    push_u64(&mut resource, 0x0100_0000);
    stream.extend_from_slice(&make_hob(HOB_TYPE_RESOURCE_DESCRIPTOR, &resource));

    let mut extension = Vec::new();
    extension.extend_from_slice(&guid(0xB2));
    extension.extend_from_slice(b"DATA");
    stream.extend_from_slice(&make_hob(HOB_TYPE_GUID_EXTENSION, &extension));

    let mut volume = Vec::new();
    push_u64(&mut volume, 0xFF00_0000);
    push_u64(&mut volume, 0x0010_0000);
    stream.extend_from_slice(&make_hob(HOB_TYPE_FIRMWARE_VOLUME, &volume));

    let mut volume_two = Vec::new();
    push_u64(&mut volume_two, 0xFD00_0000);
    push_u64(&mut volume_two, 0x0020_0000);
    volume_two.extend_from_slice(&guid(0xC3));
    volume_two.extend_from_slice(&guid(0xD4));
    stream.extend_from_slice(&make_hob(HOB_TYPE_FIRMWARE_VOLUME2, &volume_two));

    stream.extend_from_slice(&make_hob(HOB_TYPE_END_OF_HOB_LIST, &[]));

    assert!(is_valid_hob_list(&stream));
    assert_eq!(hob_count(&stream), Some(6));

    let mut iterator = hob_iter(&stream);
    let Some(Ok(handoff_hob)) = iterator.next() else {
        panic!("expected handoff HOB")
    };
    let Some(handoff) = parse_handoff_info_table(&handoff_hob) else {
        panic!("handoff table did not parse")
    };
    assert_eq!(handoff.version, 9);
    assert_eq!(handoff.boot_mode, 1);
    assert_eq!(handoff.memory_top, 0x1000_0000);
    assert_eq!(handoff.end_of_hob_list, 0x0FFF_1234);

    let Some(Ok(memory_hob)) = iterator.next() else {
        panic!("expected memory HOB")
    };
    let Some(memory) = parse_memory_allocation(&memory_hob) else {
        panic!("memory allocation did not parse")
    };
    assert_eq!(memory.base_address, 0x2000_0000);
    assert_eq!(memory.memory_type, 7);

    let Some(Ok(resource_hob)) = iterator.next() else {
        panic!("expected resource HOB")
    };
    let Some(resource) = parse_resource_descriptor(&resource_hob) else {
        panic!("resource descriptor did not parse")
    };
    assert_eq!(resource.resource_type, ResourceType::PciMemory64);
    assert_eq!(resource.physical_start, 0x8000_0000);
    assert_eq!(resource.resource_length, 0x0100_0000);

    let Some(Ok(extension_hob)) = iterator.next() else {
        panic!("expected GUID HOB")
    };
    let Some(extension) = parse_guid_hob(&extension_hob) else {
        panic!("GUID HOB did not parse")
    };
    assert_eq!(extension.data.get(..4), Some(&b"DATA"[..]));

    let Some(Ok(volume_hob)) = iterator.next() else {
        panic!("expected firmware volume HOB")
    };
    let Some(volume) = parse_firmware_volume(&volume_hob) else {
        panic!("firmware volume HOB did not parse")
    };
    assert_eq!(volume.base_address, 0xFF00_0000);

    let Some(Ok(second_volume_hob)) = iterator.next() else {
        panic!("expected firmware volume 2 HOB")
    };
    let Some(second_volume) = parse_firmware_volume2(&second_volume_hob) else {
        panic!("firmware volume 2 HOB did not parse")
    };
    assert_eq!(second_volume.length, 0x0020_0000);
}

#[test]
fn hob_malformed_inputs_are_rejected() {
    assert_eq!(
        hob_iter(&[0x01, 0x00]).next(),
        Some(Err(HobError::TruncatedHeader))
    );
    let mut short = Vec::new();
    push_u16(&mut short, HOB_TYPE_HANDOFF);
    push_u16(&mut short, 4);
    push_u32(&mut short, 0);
    assert_eq!(hob_iter(&short).next(), Some(Err(HobError::InvalidLength)));

    let mut misaligned = Vec::new();
    push_u16(&mut misaligned, HOB_TYPE_HANDOFF);
    push_u16(&mut misaligned, 9);
    push_u32(&mut misaligned, 0);
    misaligned.push(0);
    assert_eq!(
        hob_iter(&misaligned).next(),
        Some(Err(HobError::MisalignedLength))
    );

    let mut out_of_bounds = Vec::new();
    push_u16(&mut out_of_bounds, HOB_TYPE_HANDOFF);
    push_u16(&mut out_of_bounds, 16);
    push_u32(&mut out_of_bounds, 0);
    assert_eq!(
        hob_iter(&out_of_bounds).next(),
        Some(Err(HobError::OutOfBounds))
    );

    let single = make_hob(HOB_TYPE_MEMORY_ALLOCATION, &[0_u8; 24]);
    assert!(!is_valid_hob_list(&single));
    assert_eq!(hob_count(&single), None);
    assert!(!is_valid_hob_list(&[]));
}

#[test]
fn depex_parses_and_validates_postfix_expression() {
    let mut expression = Vec::new();
    expression.push(DEPEX_PUSH);
    expression.extend_from_slice(&guid(1));
    expression.push(DEPEX_PUSH);
    expression.extend_from_slice(&guid(2));
    expression.push(DEPEX_AND);
    expression.push(DEPEX_NOT);
    expression.push(DEPEX_PUSH);
    expression.extend_from_slice(&guid(3));
    expression.push(DEPEX_OR);
    expression.push(DEPEX_BEFORE);
    expression.extend_from_slice(&guid(4));
    expression.push(DEPEX_AFTER);
    expression.extend_from_slice(&guid(5));
    expression.push(DEPEX_OR);
    expression.push(DEPEX_AND);
    expression.push(DEPEX_END);
    assert_eq!(validate_dependency_expression(&expression), Ok(()));

    let tokens: Vec<DepexToken> = DepexIter::new(&expression).filter_map(Result::ok).collect();
    assert_eq!(tokens.len(), 11);
    assert!(matches!(tokens.first(), Some(DepexToken::Push(_))));
    assert_eq!(tokens.last(), Some(&DepexToken::End));
}

#[test]
fn depex_accepts_constants_and_zero_padding() {
    assert_eq!(
        validate_dependency_expression(&[DEPEX_TRUE, DEPEX_END]),
        Ok(())
    );
    assert_eq!(
        validate_dependency_expression(&[DEPEX_FALSE, DEPEX_END, 0, 0, 0]),
        Ok(())
    );
    let mut truth = vec![DEPEX_TRUE, DEPEX_TRUE, DEPEX_AND, DEPEX_END];
    truth.push(0);
    assert_eq!(validate_dependency_expression(&truth), Ok(()));
}

#[test]
fn depex_rejects_malformed_expressions() {
    assert_eq!(
        validate_dependency_expression(&[DEPEX_PUSH, 0, 0]),
        Err(DepexError::TruncatedGuid)
    );
    assert_eq!(
        validate_dependency_expression(&[DEPEX_TRUE, DEPEX_AND, DEPEX_END]),
        Err(DepexError::StackUnderflow)
    );
    assert_eq!(
        validate_dependency_expression(&[DEPEX_NOT, DEPEX_END]),
        Err(DepexError::StackUnderflow)
    );
    assert_eq!(
        validate_dependency_expression(&[DEPEX_TRUE, DEPEX_TRUE, DEPEX_END]),
        Err(DepexError::UnconsumedOperands)
    );
    assert_eq!(
        validate_dependency_expression(&[DEPEX_TRUE]),
        Err(DepexError::MissingEnd)
    );
    assert_eq!(
        validate_dependency_expression(&[0x7F]),
        Err(DepexError::UnknownOpcode(0x7F))
    );
    let mut trailing = vec![DEPEX_TRUE, DEPEX_END, 0xFF];
    assert_eq!(
        validate_dependency_expression(&trailing),
        Err(DepexError::TrailingData)
    );
    trailing[2] = 0;
    assert_eq!(validate_dependency_expression(&trailing), Ok(()));
    assert_eq!(
        validate_dependency_expression(&[DEPEX_END]),
        Err(DepexError::IncompleteExpression)
    );
}

fn ifr_op(opcode: u8, payload: &[u8], scope: bool) -> Vec<u8> {
    let mut buffer = Vec::new();
    let length = u8::try_from(payload.len() + 2).unwrap_or(0x7F);
    buffer.push(opcode);
    buffer.push(length | if scope { 0x80 } else { 0 });
    buffer.extend_from_slice(payload);
    buffer
}

#[test]
fn ifr_validates_nested_scopes() {
    let mut stream = Vec::new();
    stream.extend_from_slice(&ifr_op(IFR_FORM_SET, &[0; 20], true));
    stream.extend_from_slice(&ifr_op(IFR_FORM, &[0; 2], true));
    stream.extend_from_slice(&ifr_op(0x80, &[1, 2, 3], false));
    stream.extend_from_slice(&ifr_op(IFR_END, &[], false));
    stream.extend_from_slice(&ifr_op(IFR_END, &[], false));
    assert_eq!(validate_ifr(&stream), Ok(()));
    assert_eq!(ifr_opcode_count(&stream), Some(5));

    let mut iterator = IfrIter::new(&stream);
    let Some(Ok(first)) = iterator.next() else {
        panic!("expected form-set opcode")
    };
    assert_eq!(first.opcode, IFR_FORM_SET);
    assert!(first.scope);
    assert_eq!(first.length, 22);
    assert_eq!(first.payload.len(), 20);
    assert_eq!(iterator.position(), 22);
}

#[test]
fn ifr_rejects_malformed_streams() {
    assert_eq!(validate_ifr(&[0x01]), Err(IfrError::TruncatedHeader));
    assert_eq!(validate_ifr(&[]), Err(IfrError::TruncatedHeader));
    assert_eq!(
        validate_ifr(&[IFR_FORM, 0x01, 0x00]),
        Err(IfrError::InvalidLength)
    );
    assert_eq!(
        validate_ifr(&[IFR_FORM, 0x10, 0x00]),
        Err(IfrError::OutOfBounds)
    );
    assert_eq!(
        validate_ifr(&ifr_op(IFR_FORM, &[], true)),
        Err(IfrError::MissingEnd)
    );
    assert_eq!(
        validate_ifr(&ifr_op(IFR_END, &[], false)),
        Err(IfrError::UnexpectedEnd)
    );
    assert_eq!(
        validate_ifr(&ifr_op(IFR_END, &[], true)),
        Err(IfrError::UnexpectedEnd)
    );
}
