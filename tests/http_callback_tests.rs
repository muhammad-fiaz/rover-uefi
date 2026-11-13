//! Integration tests for HTTP Callback Protocol.
#![allow(clippy::expect_used)]

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::network::http_callback::{
    HTTP_CALLBACK_PROTOCOL_GUID, HttpCallback, HttpCallbackEvent, HttpCallbackProtocol,
};

static mut LAST_EVENT: Option<HttpCallbackEvent> = None;
static mut LAST_STATUS: Option<EfiStatus> = None;

unsafe extern "efiapi" fn mock_http_callback(
    _this: *mut HttpCallbackProtocol,
    event: HttpCallbackEvent,
    event_status: EfiStatus,
) {
    unsafe {
        LAST_EVENT = Some(event);
        LAST_STATUS = Some(event_status);
    }
}

#[test]
fn test_http_callback_guid_and_events() {
    assert_eq!(
        HTTP_CALLBACK_PROTOCOL_GUID.to_string(),
        "611114f1-a37b-4468-a436-5bdda16aa240"
    );

    assert_eq!(HttpCallbackEvent::Dns as u32, 0);
    assert_eq!(HttpCallbackEvent::ConnectTcp as u32, 1);
    assert_eq!(HttpCallbackEvent::TlsConnectSession as u32, 2);
    assert_eq!(HttpCallbackEvent::InitSession as u32, 3);
    assert_eq!(HttpCallbackEvent::TlsConfigured as u32, 4);
}

#[test]
fn test_http_callback_wrapper() {
    let mut raw_proto = HttpCallbackProtocol {
        callback: mock_http_callback,
    };

    let mut client = unsafe { HttpCallback::from_raw(&raw mut raw_proto) }.expect("valid proto");

    client.notify(HttpCallbackEvent::Dns, EfiStatus::SUCCESS);
    assert_eq!(unsafe { LAST_EVENT }, Some(HttpCallbackEvent::Dns));
    assert_eq!(unsafe { LAST_STATUS }, Some(EfiStatus::SUCCESS));

    client.notify(
        HttpCallbackEvent::TlsConnectSession,
        EfiStatus::DEVICE_ERROR,
    );
    assert_eq!(
        unsafe { LAST_EVENT },
        Some(HttpCallbackEvent::TlsConnectSession)
    );
    assert_eq!(unsafe { LAST_STATUS }, Some(EfiStatus::DEVICE_ERROR));
}
