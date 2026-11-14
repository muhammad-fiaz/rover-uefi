//! Integration tests for UEFI Image Execution Information Table.

use rover_uefi::base::EfiGuid;
use rover_uefi::protocols::security::image_execution::*;

#[test]
fn test_image_execution_constants_and_guids() {
    assert_eq!(
        EFI_IMAGE_EXECUTION_INFO_TABLE_GUID,
        EfiGuid::new(
            0x58FB_BED1,
            0x9E24,
            0x4FA6,
            [0x9D, 0x0F, 0x6A, 0x0B, 0x17, 0x43, 0xA0, 0xC8]
        )
    );
    assert_eq!(
        EFI_IMAGE_SECURITY_DATABASE_GUID,
        EfiGuid::new(
            0xD719_B2CB,
            0x3D3A,
            0x4596,
            [0xA3, 0xBC, 0xDA, 0xD0, 0x0E, 0x67, 0x65, 0x6F]
        )
    );

    assert_eq!(EFI_IMAGE_EXECUTION_AUTHENTICATION, 0x0000_0007);
    assert_eq!(EFI_IMAGE_EXECUTION_AUTH_SIG_PASSED, 0x0000_0002);
    assert_eq!(EFI_IMAGE_EXECUTION_INITIALIZED, 0x0000_0008);
}

#[test]
fn test_image_execution_record_round_trip() {
    let name = [
        u16::from(b'B'),
        u16::from(b'o'),
        u16::from(b'o'),
        u16::from(b't'),
        u16::from(b'X'),
        u16::from(b'6'),
        u16::from(b'4'),
    ];
    // EndEntire device path node
    let device_path = [0x7Fu8, 0xFF, 0x04, 0x00];
    let signature = [0xAAu8, 0xBB, 0xCC, 0xDD];

    let original = ImageExecutionInfoView {
        action: EFI_IMAGE_EXECUTION_AUTH_SIG_PASSED | EFI_IMAGE_EXECUTION_INITIALIZED,
        name: &name,
        device_path: &device_path,
        signature: &signature,
    };

    assert!(original.is_sig_passed());
    assert!(!original.is_sig_failed());
    assert!(!original.is_policy_failed());
    assert!(original.is_initialized());

    let mut buffer = [0u8; 128];
    let ser_res = original.serialize(&mut buffer);
    assert!(ser_res.is_ok());

    if let Ok(size) = ser_res {
        assert_eq!(size, original.serialized_size());

        let parsed = ImageExecutionInfoView::parse(&buffer[..size]);
        assert!(parsed.is_ok());

        if let Ok(view) = parsed {
            assert_eq!(view.action, original.action);
            assert_eq!(view.name, original.name);
            assert_eq!(view.device_path, original.device_path);
            assert_eq!(view.signature, original.signature);
            assert!(view.is_sig_passed());
            assert!(view.is_initialized());
        }
    }
}

#[test]
fn test_image_execution_table_iteration() {
    let name1 = [u16::from(b'O'), u16::from(b'S'), u16::from(b'1')];
    let name2 = [u16::from(b'O'), u16::from(b'S'), u16::from(b'2')];
    let dp = [0x7Fu8, 0xFF, 0x04, 0x00];

    let item1 = ImageExecutionInfoView {
        action: EFI_IMAGE_EXECUTION_AUTH_SIG_PASSED | EFI_IMAGE_EXECUTION_INITIALIZED,
        name: &name1,
        device_path: &dp,
        signature: &[0x11, 0x22],
    };

    let item2 = ImageExecutionInfoView {
        action: EFI_IMAGE_EXECUTION_AUTH_SIG_FAILED,
        name: &name2,
        device_path: &dp,
        signature: &[],
    };

    let mut table_buf = [0u8; 256];
    let hdr_size = core::mem::size_of::<EfiImageExecutionInfoTableHeader>();

    // Write NumberOfImages = 2
    let num_images: usize = 2;
    table_buf[..hdr_size].copy_from_slice(&num_images.to_ne_bytes());

    let mut offset = hdr_size;
    let res1 = item1.serialize(&mut table_buf[offset..]);
    assert!(res1.is_ok());
    if let Ok(len1) = res1 {
        offset += len1;
    }

    let res2 = item2.serialize(&mut table_buf[offset..]);
    assert!(res2.is_ok());
    if let Ok(len2) = res2 {
        offset += len2;
    }

    let table_view = ImageExecutionInfoTableView::parse(&table_buf[..offset]);
    assert!(table_view.is_ok());

    if let Ok(tv) = table_view {
        assert_eq!(tv.number_of_images, 2);
        let items: Vec<ImageExecutionInfoView> = tv.iter().collect();
        assert_eq!(items.len(), 2);
        assert!(items[0].is_sig_passed());
        assert!(items[0].is_initialized());
        assert_eq!(items[0].name, &name1);

        assert!(items[1].is_sig_failed());
        assert!(!items[1].is_initialized());
        assert_eq!(items[1].name, &name2);
    }
}
