//! # HTTP Callback Protocol
//!
//! Provides notification and observability hooks for HTTP, TCP, TLS, and DNS
//! transactions across the network boot stack.

use crate::base::{EfiGuid, EfiStatus};

/// HTTP Callback Protocol GUID: `{611114f1-a37b-4468-a436-5bdda16aa240}`.
pub const HTTP_CALLBACK_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x6111_14F1,
    0xA37B,
    0x4468,
    [0xA4, 0x36, 0x5B, 0xDD, 0xA1, 0x6A, 0xA2, 0x40],
);

/// HTTP transaction events reported via [`HttpCallbackProtocol`].
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpCallbackEvent {
    /// DNS host address resolution event.
    Dns = 0,
    /// Non-blocking TCP connection initialization event.
    ConnectTcp = 1,
    /// TLS session handshake completion event.
    TlsConnectSession = 2,
    /// HTTP session initialization event.
    InitSession = 3,
    /// TLS session configuration application event.
    TlsConfigured = 4,
}

/// Function pointer type for HTTP callback invocation.
pub type HttpCallbackFn = unsafe extern "efiapi" fn(
    this: *mut HttpCallbackProtocol,
    event: HttpCallbackEvent,
    event_status: EfiStatus,
);

/// Raw HTTP Callback Protocol interface.
#[repr(C)]
pub struct HttpCallbackProtocol {
    /// Callback function invoked when an HTTP stack event occurs.
    pub callback: HttpCallbackFn,
}

/// Safe client wrapper around [`HttpCallbackProtocol`].
pub struct HttpCallback<'a> {
    raw: &'a mut HttpCallbackProtocol,
}

impl HttpCallback<'_> {
    /// Wraps a raw [`HttpCallbackProtocol`] pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `raw` points to an initialized, valid protocol instance.
    #[must_use]
    pub unsafe fn from_raw(raw: *mut HttpCallbackProtocol) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self {
                raw: unsafe { &mut *raw },
            })
        }
    }

    /// Invokes the underlying HTTP callback for a specified event and status.
    pub fn notify(&mut self, event: HttpCallbackEvent, event_status: EfiStatus) {
        unsafe {
            (self.raw.callback)(self.raw, event, event_status);
        }
    }
}
