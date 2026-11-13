//! # `FTPv4` Protocol Tests
//!
//! Unit tests for `FTPv4` protocol definitions and client wrapper.

use rover_uefi::base::{EfiEvent, EfiStatus};
use rover_uefi::protocols::network::ftp4::{
    EFI_FTP4_PROTOCOL_GUID, EFI_FTP4_SERVICE_BINDING_PROTOCOL_GUID, EfiFtp4CommandToken,
    EfiFtp4ConfigData, EfiFtp4ConnectionToken, EfiFtp4Protocol, FILE_STRUCT_FILE, FILE_STRUCT_PAGE,
    FILE_STRUCT_RECORD, Ftp4, REP_TYPE_ASCII, REP_TYPE_EBCDIC, REP_TYPE_IMAGE, REP_TYPE_LOCAL,
    TRANS_MODE_BLOCK, TRANS_MODE_COMPRESSED, TRANS_MODE_STREAM,
};
use rover_uefi::protocols::network::types::Ipv4Address;

#[test]
fn test_ftp4_guids() {
    assert_ne!(
        EFI_FTP4_SERVICE_BINDING_PROTOCOL_GUID,
        EFI_FTP4_PROTOCOL_GUID
    );
}

#[test]
fn test_ftp4_constants() {
    assert_eq!(REP_TYPE_IMAGE, 0x00);
    assert_eq!(REP_TYPE_EBCDIC, 0x01);
    assert_eq!(REP_TYPE_ASCII, 0x02);
    assert_eq!(REP_TYPE_LOCAL, 0x03);

    assert_eq!(FILE_STRUCT_FILE, 0x00);
    assert_eq!(FILE_STRUCT_RECORD, 0x01);
    assert_eq!(FILE_STRUCT_PAGE, 0x02);

    assert_eq!(TRANS_MODE_STREAM, 0x00);
    assert_eq!(TRANS_MODE_BLOCK, 0x01);
    assert_eq!(TRANS_MODE_COMPRESSED, 0x02);
}

static CONNECT_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static CLOSE_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static READ_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static WRITE_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static POLL_CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

unsafe extern "efiapi" fn mock_ftp4_get_mode_data(
    _this: *mut EfiFtp4Protocol,
    _mode_data: *mut EfiFtp4ConfigData,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_ftp4_connect(
    _this: *mut EfiFtp4Protocol,
    token: *mut EfiFtp4ConnectionToken,
) -> EfiStatus {
    CONNECT_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    if !token.is_null() {
        unsafe {
            (*token).status = EfiStatus::SUCCESS;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_ftp4_close(
    _this: *mut EfiFtp4Protocol,
    token: *mut EfiFtp4ConnectionToken,
) -> EfiStatus {
    CLOSE_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    if !token.is_null() {
        unsafe {
            (*token).status = EfiStatus::SUCCESS;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_ftp4_configure(
    _this: *mut EfiFtp4Protocol,
    _ftp_config_data: *const EfiFtp4ConfigData,
) -> EfiStatus {
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_ftp4_read_file(
    _this: *mut EfiFtp4Protocol,
    token: *mut EfiFtp4CommandToken,
) -> EfiStatus {
    READ_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    if !token.is_null() {
        unsafe {
            (*token).status = EfiStatus::SUCCESS;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_ftp4_write_file(
    _this: *mut EfiFtp4Protocol,
    token: *mut EfiFtp4CommandToken,
) -> EfiStatus {
    WRITE_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    if !token.is_null() {
        unsafe {
            (*token).status = EfiStatus::SUCCESS;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_ftp4_read_directory(
    _this: *mut EfiFtp4Protocol,
    token: *mut EfiFtp4CommandToken,
) -> EfiStatus {
    if !token.is_null() {
        unsafe {
            (*token).status = EfiStatus::SUCCESS;
        }
    }
    EfiStatus::SUCCESS
}

unsafe extern "efiapi" fn mock_ftp4_poll(_this: *mut EfiFtp4Protocol) -> EfiStatus {
    POLL_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
    EfiStatus::SUCCESS
}

#[test]
fn test_ftp4_client_wrapper() {
    let mut raw = EfiFtp4Protocol {
        get_mode_data: mock_ftp4_get_mode_data,
        connect: mock_ftp4_connect,
        close: mock_ftp4_close,
        configure: mock_ftp4_configure,
        read_file: mock_ftp4_read_file,
        write_file: mock_ftp4_write_file,
        read_directory: mock_ftp4_read_directory,
        poll: mock_ftp4_poll,
    };

    let mut client = Ftp4::new(&mut raw);
    assert!(!client.raw().is_null());

    let config = EfiFtp4ConfigData {
        username: core::ptr::null_mut(),
        password: core::ptr::null_mut(),
        active: false,
        use_default_setting: true,
        station_ip: Ipv4Address([0, 0, 0, 0]),
        subnet_mask: Ipv4Address([0, 0, 0, 0]),
        gateway_ip: Ipv4Address([0, 0, 0, 0]),
        server_ip: Ipv4Address([192, 168, 1, 1]),
        server_port: 21,
        alt_data_port: 0,
        rep_type: REP_TYPE_IMAGE,
        file_struct: FILE_STRUCT_FILE,
        trans_mode: TRANS_MODE_STREAM,
    };

    assert!(client.configure(Some(&config)).is_ok());
    assert!(client.configure(None).is_ok());

    let mut conn_token = EfiFtp4ConnectionToken {
        event: EfiEvent::NULL,
        status: EfiStatus::NOT_READY,
    };
    assert!(client.connect(&mut conn_token).is_ok());
    assert!(CONNECT_CALLED.load(std::sync::atomic::Ordering::SeqCst));
    assert_eq!(conn_token.status, EfiStatus::SUCCESS);

    let mut cmd_token = EfiFtp4CommandToken {
        event: EfiEvent::NULL,
        pathname: core::ptr::null_mut(),
        data_buffer_size: 1024,
        data_buffer: core::ptr::null_mut(),
        data_callback: None,
        context: core::ptr::null_mut(),
        status: EfiStatus::NOT_READY,
    };

    assert!(client.read_file(&mut cmd_token).is_ok());
    assert!(READ_CALLED.load(std::sync::atomic::Ordering::SeqCst));

    assert!(client.write_file(&mut cmd_token).is_ok());
    assert!(WRITE_CALLED.load(std::sync::atomic::Ordering::SeqCst));

    assert!(client.read_directory(&mut cmd_token).is_ok());

    assert!(client.close(&mut conn_token).is_ok());
    assert!(CLOSE_CALLED.load(std::sync::atomic::Ordering::SeqCst));

    assert!(client.poll().is_ok());
    assert!(POLL_CALLED.load(std::sync::atomic::Ordering::SeqCst));
}
