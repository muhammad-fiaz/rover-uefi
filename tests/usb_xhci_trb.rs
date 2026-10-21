//! Tests for the xHCI Transfer Request Block encodings.
use rover_uefi::protocols::usb::xhci::trb::{
    CompletionCode, ControlTransferType, NormalTrb, SetupStageTrb, TRB_SIZE, Trb, TrbFlags, TrbType,
};

#[test]
fn trb_flags_set_operations() {
    let flags = TrbFlags::CYCLE.union(TrbFlags::CHAIN);
    assert!(flags.contains(TrbFlags::CYCLE));
    assert!(flags.contains(TrbFlags::CHAIN));
    assert!(!flags.contains(TrbFlags::NO_SNOOP));
    assert!(!flags.is_empty());
    assert!(TrbFlags::from_bits_truncate(0).is_empty());
    assert_eq!(flags.difference(TrbFlags::CYCLE), TrbFlags::CHAIN);
    assert_eq!(TrbFlags::CYCLE.bits(), 0x0001);
}

#[test]
fn trb_type_classification() {
    assert!(TrbType::Normal.is_assigned());
    assert!(!TrbType::Normal.is_event());
    assert!(!TrbType::Normal.is_command());
    assert!(TrbType::TransferEvent.is_event());
    assert!(TrbType::CommandCompletionEvent.is_event());
    assert!(TrbType::EnableSlot.is_command());
    assert!(TrbType::AddressDevice.is_command());
    assert!(!TrbType::Reserved(0x3F).is_assigned());
}

#[test]
fn trb_type_raw_roundtrip() {
    let types = [
        TrbType::Normal,
        TrbType::SetupStage,
        TrbType::DataStage,
        TrbType::StatusStage,
        TrbType::Link,
        TrbType::NoOp,
        TrbType::EnableSlot,
        TrbType::TransferEvent,
        TrbType::CommandCompletionEvent,
        TrbType::PortStatusChangeEvent,
    ];
    for trb_type in types {
        assert_eq!(TrbType::from_raw(trb_type.to_raw()), trb_type);
    }
}

#[test]
fn completion_codes() {
    assert_eq!(CompletionCode::Success.to_raw(), 1);
    assert!(CompletionCode::Success.is_success());
    assert!(CompletionCode::from_raw(1).is_success());
    assert!(!CompletionCode::UsbTransactionError.is_success());
    assert!(!CompletionCode::from_raw(0).is_success());
}

#[test]
fn normal_trb_encode_decode() {
    let trb = Trb::Normal(NormalTrb::new(0x1234_5000, 512, 1));
    let mut bytes = [0_u8; TRB_SIZE];
    assert!(trb.encode(&mut bytes));
    assert_eq!(bytes.len(), TRB_SIZE);
    assert_eq!(&bytes[..8], &0x1234_5000_u64.to_le_bytes());

    let Some(decoded) = Trb::decode(&bytes) else {
        panic!("a complete TRB must decode");
    };
    assert_eq!(decoded.trb_type(), TrbType::Normal);
    let Trb::Normal(normal) = decoded else {
        panic!("the TRB must decode as a normal TRB");
    };
    assert_eq!(normal.buffer, 0x1234_5000);
    assert_eq!(normal.length, 512);
    assert_eq!(normal.interrupter, 1);
    assert!(normal.flags.contains(TrbFlags::CYCLE));
    assert!(normal.flags.contains(TrbFlags::INTERRUPT_ON_COMPLETION));
}

#[test]
fn setup_stage_carries_the_setup_packet() {
    let setup = [0x80_u8, 0x06, 0x00, 0x01, 0x00, 0x00, 0x40, 0x00];
    let trb = Trb::SetupStage(SetupStageTrb::new(setup, 0, ControlTransferType::InData));
    let mut bytes = [0_u8; TRB_SIZE];
    assert!(trb.encode(&mut bytes));
    assert_eq!(&bytes[..8], &setup);
    let Some(decoded) = Trb::decode(&bytes) else {
        panic!("the setup stage TRB must decode");
    };
    assert_eq!(decoded.trb_type(), TrbType::SetupStage);
    let Trb::SetupStage(stage) = decoded else {
        panic!("the TRB must decode as a setup stage");
    };
    assert_eq!(stage.setup, setup);
    assert!(stage.flags.contains(TrbFlags::IMMEDIATE_DATA));
}

#[test]
fn encode_rejects_short_buffers_and_decode_rejects_truncated_input() {
    let trb = Trb::Normal(NormalTrb::new(0, 16, 0));
    let mut short = [0_u8; TRB_SIZE - 1];
    assert!(!trb.encode(&mut short));
    assert!(Trb::decode(&[]).is_none());
    assert!(Trb::decode(&[0_u8; 15]).is_none());
}

#[test]
fn unknown_types_are_preserved() {
    // Build a TRB whose type field is a reserved encoding and decode it.
    let mut bytes = [0_u8; TRB_SIZE];
    bytes[12..16].copy_from_slice(&(0x3F_u32 << 10).to_le_bytes());
    let Some(decoded) = Trb::decode(&bytes) else {
        panic!("a reserved TRB must still decode");
    };
    assert_eq!(decoded.trb_type(), TrbType::Reserved(0x3F));
}
