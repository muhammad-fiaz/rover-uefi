//! Unit tests for OP-TEE TEE Call Protocol.

use rover_uefi::arch::arm64::optee::*;

#[test]
fn test_optee_uid_and_guid() {
    assert!(is_optee_uid(&OPTEE_OS_UID));
    let fake_uid = [0, 0, 0, 0];
    assert!(!is_optee_uid(&fake_uid));

    assert_eq!(
        OPTEE_OS_GUID.to_string(),
        "384fb3e0-e7f8-11e3-af63-0002a5d5c51b"
    );
}

#[test]
fn test_optee_msg_header_and_params() {
    let mut header = OpteeMsgHeader::new(1, 2, 3, 2);
    assert_eq!(header.command, 1);
    assert_eq!(header.function, 2);
    assert_eq!(header.session, 3);
    assert_eq!(header.num_params, 2);
    assert!(header.is_success());

    header.ret = OPTEE_ERROR_COMMUNICATION;
    assert!(!header.is_success());

    let val_param = OpteeMsgParam {
        attribute: OPTEE_MSG_ATTR_TYPE_VALUE_INPUT,
        data: OpteeMsgParamData {
            value: OpteeMsgParamValue {
                a: 100,
                b: 200,
                c: 300,
            },
        },
    };
    assert_eq!(val_param.attribute, 1);
    let val_data = unsafe { val_param.data.value };
    assert_eq!(val_data.a, 100);
    assert_eq!(val_data.b, 200);

    let mem_param = OpteeMsgParam {
        attribute: OPTEE_MSG_ATTR_TYPE_MEMORY_INOUT,
        data: OpteeMsgParamData {
            memory: OpteeMsgParamMemory {
                buffer_address: 0x8000_0000,
                size: 4096,
                shared_memory_reference: 42,
            },
        },
    };
    let mem_data = unsafe { mem_param.data.memory };
    assert_eq!(mem_data.buffer_address, 0x8000_0000);
    assert_eq!(mem_data.size, 4096);
    assert_eq!(mem_data.shared_memory_reference, 42);
}
