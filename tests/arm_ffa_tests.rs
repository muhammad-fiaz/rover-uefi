//! Integration tests for ARM FF-A services and ABI definitions.

use rover_uefi::arch::arm64::ffa::{
    ARM_FFA_DESTINATION_ENDPOINT_ID, ARM_FFA_MAJOR_VERSION, ARM_FFA_MEM_PERM_CODE_X,
    ARM_FFA_MEM_PERM_CODE_XN, ARM_FFA_MEM_PERM_DATA_NO_ACCESS, ARM_FFA_MEM_PERM_DATA_RO,
    ARM_FFA_MEM_PERM_DATA_RW, ARM_FFA_MINOR_VERSION, ARM_FID_FFA_ERROR, ARM_FID_FFA_ID_GET,
    ARM_FID_FFA_MEM_PERM_GET_64, ARM_FID_FFA_MEM_PERM_SET_64, ARM_FID_FFA_MSG_SEND_DIRECT_REQ_64,
    ARM_FID_FFA_MSG_SEND_DIRECT_RESP_64, ARM_FID_FFA_SUCCESS_32, ARM_FID_FFA_SUCCESS_64,
    ARM_FID_FFA_VERSION, FfaStatus, ffa_dest_partition_id, ffa_is_compatible,
    ffa_is_secure_partition, ffa_make_mem_perm, ffa_make_version, ffa_pack_partition_ids,
    ffa_source_partition_id, ffa_version_major, ffa_version_minor,
};

#[test]
fn test_ffa_function_ids() {
    assert_eq!(ARM_FID_FFA_ERROR, 0x8400_0060);
    assert_eq!(ARM_FID_FFA_SUCCESS_32, 0x8400_0061);
    assert_eq!(ARM_FID_FFA_SUCCESS_64, 0xC400_0061);
    assert_eq!(ARM_FID_FFA_VERSION, 0x8400_0063);
    assert_eq!(ARM_FID_FFA_ID_GET, 0x8400_0069);
    assert_eq!(ARM_FID_FFA_MSG_SEND_DIRECT_REQ_64, 0xC400_006F);
    assert_eq!(ARM_FID_FFA_MSG_SEND_DIRECT_RESP_64, 0xC400_0070);
    assert_eq!(ARM_FID_FFA_MEM_PERM_GET_64, 0xC400_0088);
    assert_eq!(ARM_FID_FFA_MEM_PERM_SET_64, 0xC400_0089);
    assert_eq!(ARM_FFA_DESTINATION_ENDPOINT_ID, 3);
}

#[test]
fn test_ffa_version_encoding() {
    assert_eq!(ARM_FFA_MAJOR_VERSION, 1);
    assert_eq!(ARM_FFA_MINOR_VERSION, 2);
    let ver = ffa_make_version(ARM_FFA_MAJOR_VERSION, ARM_FFA_MINOR_VERSION);
    assert_eq!(ffa_version_major(ver), 1);
    assert_eq!(ffa_version_minor(ver), 2);
    assert!(ffa_is_compatible(ver, 1, 0));
    assert!(ffa_is_compatible(ver, 1, 2));
    assert!(!ffa_is_compatible(ver, 1, 3));
    assert!(!ffa_is_compatible(ver, 2, 0));
}

#[test]
fn test_ffa_partition_id_packing() {
    let packed = ffa_pack_partition_ids(0x8001, 0x0003);
    assert_eq!(ffa_source_partition_id(packed), 0x8001);
    assert_eq!(ffa_dest_partition_id(packed), 0x0003);
    assert!(ffa_is_secure_partition(0x8001));
    assert!(!ffa_is_secure_partition(0x0003));
}

#[test]
fn test_ffa_mem_perm_packing() {
    let perm_ro_xn = ffa_make_mem_perm(ARM_FFA_MEM_PERM_DATA_RO, ARM_FFA_MEM_PERM_CODE_XN);
    assert_eq!(perm_ro_xn & 0x3, ARM_FFA_MEM_PERM_DATA_RO);
    assert_eq!((perm_ro_xn >> 2) & 0x1, ARM_FFA_MEM_PERM_CODE_XN);

    let perm_rw_x = ffa_make_mem_perm(ARM_FFA_MEM_PERM_DATA_RW, ARM_FFA_MEM_PERM_CODE_X);
    assert_eq!(perm_rw_x & 0x3, ARM_FFA_MEM_PERM_DATA_RW);
    assert_eq!((perm_rw_x >> 2) & 0x1, ARM_FFA_MEM_PERM_CODE_X);

    let perm_no_access =
        ffa_make_mem_perm(ARM_FFA_MEM_PERM_DATA_NO_ACCESS, ARM_FFA_MEM_PERM_CODE_XN);
    assert_eq!(perm_no_access & 0x3, ARM_FFA_MEM_PERM_DATA_NO_ACCESS);
}

#[test]
fn test_ffa_status_mapping() {
    assert_eq!(FfaStatus::from_raw(0), FfaStatus::Success);
    assert_eq!(FfaStatus::from_raw(-1), FfaStatus::NotSupported);
    assert_eq!(FfaStatus::from_raw(-2), FfaStatus::InvalidParameters);
    assert_eq!(FfaStatus::from_raw(-3), FfaStatus::NoMemory);
    assert_eq!(FfaStatus::from_raw(-4), FfaStatus::Busy);
    assert_eq!(FfaStatus::from_raw(-5), FfaStatus::Interrupted);
    assert_eq!(FfaStatus::from_raw(-6), FfaStatus::Denied);
    assert_eq!(FfaStatus::from_raw(-7), FfaStatus::Retry);
    assert_eq!(FfaStatus::from_raw(-8), FfaStatus::Aborted);
    assert_eq!(FfaStatus::from_raw(-9), FfaStatus::NoData);
    assert_eq!(FfaStatus::from_raw(-10), FfaStatus::NotReady);
    assert_eq!(FfaStatus::from_raw(99), FfaStatus::Unknown(99));

    assert_eq!(FfaStatus::Success.to_raw(), 0);
    assert_eq!(FfaStatus::NotSupported.to_raw(), -1);
    assert_eq!(FfaStatus::Denied.to_raw(), -6);
    assert_eq!(FfaStatus::Unknown(42).to_raw(), 42);
}
