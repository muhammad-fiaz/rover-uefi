//! Integration tests for PSA Firmware Update protocol interfaces.

use rover_uefi::protocols::firmware::psa_fwu::*;

#[test]
fn test_psa_fwu_guids() {
    assert_eq!(
        PSA_FWU_UPDATE_AGENT_GUID.to_string(),
        "6823a838-1b06-470e-9774-0cce8bfb53fd"
    );
    assert_eq!(
        PSA_FWU_IMAGE_DIRECTORY_GUID.to_string(),
        "deee58d9-5147-4ad3-a290-77666e2341a5"
    );
    assert_eq!(
        PSA_FWU_METADATA_GUID.to_string(),
        "8a7a84a0-8387-40f6-ab41-a8b9a5a60d23"
    );
}

#[test]
fn test_psa_fwu_commands_and_status() {
    assert_eq!(PSA_MM_FWU_COMMAND_DISCOVER, 0);
    assert_eq!(PSA_MM_FWU_COMMAND_BEGIN_STAGING, 16);
    assert_eq!(PSA_MM_FWU_COMMAND_END_STAGING, 17);
    assert_eq!(PSA_MM_FWU_COMMAND_OPEN, 19);
    assert_eq!(PSA_MM_FWU_COMMAND_WRITE_STREAM, 20);
    assert_eq!(PSA_MM_FWU_COMMAND_COMMIT, 22);
    assert_eq!(PSA_MM_FWU_COMMAND_ACCEPT_IMAGE, 23);

    assert!(psa_fwu_is_success(PSA_MM_FWU_SUCCESS));
    assert!(!psa_fwu_is_success(PSA_MM_FWU_AUTH_FAIL));
    assert!(!psa_fwu_is_success(PSA_MM_FWU_DENIED));
}

#[test]
fn test_psa_version_parsing_and_structures() {
    let ver = (1u32 << 16) | 2u32;
    let (major, minor) = parse_psa_version(ver);
    assert_eq!(major, 1);
    assert_eq!(minor, 2);

    let discovery = PsaFwuDiscoveryResult {
        service_status: SERVICE_STATUS_OPERATIVE,
        version: ver,
        num_images: 3,
    };
    let num_images = discovery.num_images;
    let service_status = discovery.service_status;
    assert_eq!(num_images, 3);
    assert_eq!(service_status, 0);

    let stream = PsaFwuStreamHeader {
        stream_id: 42,
        block_size: 4096,
        stream_size: 1_048_576,
    };
    let stream_id = stream.stream_id;
    let block_size = stream.block_size;
    assert_eq!(stream_id, 42);
    assert_eq!(block_size, 4096);

    let prop = PsaFwuImageProperty {
        image_type_id: PSA_FWU_IMAGE_DIRECTORY_GUID,
        client_permissions: 0xFF,
        version: 0x0200_0000,
        lowest_accepted_version: 0x0100_0000,
    };
    let prop_ver = prop.version;
    assert_eq!(prop_ver, 0x0200_0000);
}
