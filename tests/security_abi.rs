//! Security ABI tests.

use core::mem::{offset_of, size_of};

use rover_uefi::protocols::security::hash::hash2::EfiHash2Protocol;
use rover_uefi::protocols::security::hash::protocol::EfiHashProtocol;
use rover_uefi::protocols::security::rng::protocol::EfiRngProtocol;
use rover_uefi::protocols::security::security_arch::protocol::EfiSecurityArchProtocol;
use rover_uefi::protocols::security::security_arch::security2::EfiSecurity2ArchProtocol;
use rover_uefi::protocols::security::tcg2::protocol::EfiTcg2Protocol;

fn pointer_size() -> usize {
    size_of::<usize>()
}

#[test]
fn rng_size() {
    assert_eq!(size_of::<EfiRngProtocol>(), 2 * pointer_size());
    assert_eq!(offset_of!(EfiRngProtocol, get_info), 0);
    assert_eq!(offset_of!(EfiRngProtocol, get_rng), pointer_size());
    assert_eq!(
        size_of::<
            Option<
                unsafe extern "efiapi" fn(
                    *mut EfiRngProtocol,
                    *mut rover_uefi::abi::types::UIntN,
                    *mut rover_uefi::base::EfiGuid,
                ) -> rover_uefi::base::EfiStatus,
            >,
        >(),
        pointer_size()
    );
}

#[test]
fn hash_sizes() {
    assert_eq!(size_of::<EfiHashProtocol>(), 2 * pointer_size());
    assert_eq!(offset_of!(EfiHashProtocol, get_hash_size), 0);
    assert_eq!(offset_of!(EfiHashProtocol, hash), pointer_size());
}

#[test]
fn hash2_size_is_five_pointers() {
    assert_eq!(size_of::<EfiHash2Protocol>(), 5 * pointer_size());
    assert_eq!(offset_of!(EfiHash2Protocol, get_hash_size), 0);
    assert_eq!(offset_of!(EfiHash2Protocol, hash), pointer_size());
    assert_eq!(offset_of!(EfiHash2Protocol, hash_init), 2 * pointer_size());
    assert_eq!(
        offset_of!(EfiHash2Protocol, hash_update),
        3 * pointer_size()
    );
    assert_eq!(offset_of!(EfiHash2Protocol, hash_final), 4 * pointer_size());
}

#[test]
fn security_sizes() {
    assert_eq!(size_of::<EfiSecurityArchProtocol>(), pointer_size());
    assert_eq!(
        offset_of!(EfiSecurityArchProtocol, file_authentication_state),
        0
    );
    assert_eq!(size_of::<EfiSecurity2ArchProtocol>(), pointer_size());
    assert_eq!(offset_of!(EfiSecurity2ArchProtocol, file_authentication), 0);
}

#[test]
fn tcg2_size_is_seven_pointers() {
    assert_eq!(size_of::<EfiTcg2Protocol>(), 7 * pointer_size());
    assert_eq!(offset_of!(EfiTcg2Protocol, get_capability), 0);
    assert_eq!(offset_of!(EfiTcg2Protocol, get_event_log), pointer_size());
    assert_eq!(
        offset_of!(EfiTcg2Protocol, hash_log_extend_event),
        2 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiTcg2Protocol, submit_command),
        3 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiTcg2Protocol, get_active_pcr_banks),
        4 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiTcg2Protocol, set_active_pcr_banks),
        5 * pointer_size()
    );
    assert_eq!(
        offset_of!(EfiTcg2Protocol, get_result_of_set_active_pcr_banks),
        6 * pointer_size()
    );
}
