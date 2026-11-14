//! Integration tests for Capsule Result Variables and Reports.

use rover_uefi::base::{EfiGuid, EfiStatus, EfiTime};
use rover_uefi::protocols::firmware::capsule_report::*;

#[test]
fn test_capsule_report_constants_and_guids() {
    assert_eq!(
        EFI_CAPSULE_REPORT_GUID,
        EfiGuid::new(
            0x39b6_8c46,
            0xf7fb,
            0x441b,
            [0xb6, 0xec, 0x16, 0xb0, 0xf6, 0x98, 0x21, 0xf3]
        )
    );
    assert_eq!(CAPSULE_RESULT_VARIABLE_FMP_VERSION, 0x0001);
    assert_eq!(CAPSULE_RESULT_VARIABLE_JSON_VERSION, 0x0001);
    assert_eq!(REDFISH_DEFINED_JSON_SCHEMA, 0xF000_0000);
}

#[test]
#[allow(clippy::cast_possible_truncation)]
fn test_capsule_report_fmp_payload_parse() {
    let header_size = core::mem::size_of::<EfiCapsuleResultVariableHeader>();
    let fmp_size = core::mem::size_of::<EfiCapsuleResultVariableFmp>();
    let total_size = (header_size + fmp_size) as u32;

    let test_guid = EfiGuid::new(
        0x1234_5678,
        0xABCD,
        0xEF01,
        [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
    );
    let type_guid = EfiGuid::new(
        0x8765_4321,
        0xDCBA,
        0x10FE,
        [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
    );

    let header = EfiCapsuleResultVariableHeader::new(
        total_size,
        test_guid,
        EfiTime::default(),
        EfiStatus::SUCCESS,
    );

    let fmp = EfiCapsuleResultVariableFmp {
        version: CAPSULE_RESULT_VARIABLE_FMP_VERSION,
        payload_index: 2,
        update_image_index: 1,
        update_image_type_id: type_guid,
    };

    let mut buffer = std::vec![0u8; total_size as usize];
    unsafe {
        core::ptr::copy_nonoverlapping(
            (&raw const header).cast::<u8>(),
            buffer.as_mut_ptr(),
            header_size,
        );
        core::ptr::copy_nonoverlapping(
            (&raw const fmp).cast::<u8>(),
            buffer.as_mut_ptr().add(header_size),
            fmp_size,
        );
    }

    let report = CapsuleResultReport::parse(&buffer);
    assert!(report.is_ok());
    if let Ok(rep) = report {
        assert_eq!(rep.header.variable_total_size, total_size);
        assert_eq!(rep.header.capsule_guid, test_guid);
        assert_eq!(rep.header.capsule_status, EfiStatus::SUCCESS);

        let parsed_fmp = rep.parse_fmp();
        assert!(parsed_fmp.is_ok());
        if let Ok(f) = parsed_fmp {
            assert_eq!(f.version, CAPSULE_RESULT_VARIABLE_FMP_VERSION);
            assert_eq!(f.payload_index, 2);
            assert_eq!(f.update_image_index, 1);
            assert_eq!(f.update_image_type_id, type_guid);
        }
    }
}

#[test]
#[allow(clippy::cast_possible_truncation)]
fn test_capsule_report_json_payload_parse() {
    let header_size = core::mem::size_of::<EfiCapsuleResultVariableHeader>();
    let json_header_size = core::mem::size_of::<EfiCapsuleResultVariableJson>();
    let json_body = br#"{"Status":"Success","ErrorCode":0}"#;
    let total_size = (header_size + json_header_size + json_body.len()) as u32;

    let test_guid = EfiGuid::new(
        0xAAAA_BBBB,
        0xCCCC,
        0xDDDD,
        [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
    );

    let header = EfiCapsuleResultVariableHeader::new(
        total_size,
        test_guid,
        EfiTime::default(),
        EfiStatus::SUCCESS,
    );

    let json_hdr = EfiCapsuleResultVariableJson {
        version: CAPSULE_RESULT_VARIABLE_JSON_VERSION,
        capsule_id: REDFISH_DEFINED_JSON_SCHEMA,
        resp_length: json_body.len() as u32,
    };

    let mut buffer = std::vec![0u8; total_size as usize];
    unsafe {
        core::ptr::copy_nonoverlapping(
            (&raw const header).cast::<u8>(),
            buffer.as_mut_ptr(),
            header_size,
        );
        core::ptr::copy_nonoverlapping(
            (&raw const json_hdr).cast::<u8>(),
            buffer.as_mut_ptr().add(header_size),
            json_header_size,
        );
        core::ptr::copy_nonoverlapping(
            json_body.as_ptr(),
            buffer.as_mut_ptr().add(header_size + json_header_size),
            json_body.len(),
        );
    }

    let report = CapsuleResultReport::parse(&buffer);
    assert!(report.is_ok());
    if let Ok(rep) = report {
        let parsed_json = rep.parse_json();
        assert!(parsed_json.is_ok());
        if let Ok((hdr, payload_slice)) = parsed_json {
            assert_eq!(hdr.version, CAPSULE_RESULT_VARIABLE_JSON_VERSION);
            assert_eq!(hdr.capsule_id, REDFISH_DEFINED_JSON_SCHEMA);
            assert_eq!(payload_slice, json_body);
        }
    }

    // Check bounds failures
    let short_buffer = [0u8; 4];
    assert_eq!(
        CapsuleResultReport::parse(&short_buffer),
        Err(EfiStatus::BUFFER_TOO_SMALL)
    );
}
