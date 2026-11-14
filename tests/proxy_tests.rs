//! Integration tests for Network Proxy and HTTP CONNECT tunneling subsystem.

use rover_uefi::base::EfiStatus;
use rover_uefi::protocols::network::proxy::{
    ProxyAuthScheme, ProxyBypassRule, ProxyConfig, ProxyRouter, ProxyType,
    build_http_connect_request, parse_wpad_option_252, validate_proxy_connect_response,
};

#[test]
fn test_proxy_config_and_auth() {
    let cfg = ProxyConfig::new(ProxyType::Http, "proxy.corp.internal", 8080);
    assert_eq!(cfg.proxy_type, ProxyType::Http);
    assert_eq!(cfg.host, "proxy.corp.internal");
    assert_eq!(cfg.port, 8080);
    assert_eq!(cfg.auth_scheme, ProxyAuthScheme::None);
    assert!(cfg.username.is_none());

    let authed_cfg = cfg.with_basic_auth("admin", "secret123");
    assert_eq!(authed_cfg.auth_scheme, ProxyAuthScheme::Basic);
    assert_eq!(authed_cfg.username, Some("admin"));
    assert_eq!(authed_cfg.password, Some("secret123"));
}

#[test]
fn test_proxy_router_and_bypass_rules() {
    let mut router = ProxyRouter::<8>::new();
    let cfg = ProxyConfig::new(ProxyType::Https, "10.0.0.1", 3128);
    router.set_config(cfg);

    assert!(router.add_bypass_rule(ProxyBypassRule::Loopback).is_ok());
    assert!(
        router
            .add_bypass_rule(ProxyBypassRule::ExactDomain("intranet.local"))
            .is_ok()
    );
    assert!(
        router
            .add_bypass_rule(ProxyBypassRule::SuffixDomain(".corp.internal"))
            .is_ok()
    );

    // Loopback bypass
    assert!(router.should_bypass("localhost"));
    assert!(router.should_bypass("127.0.0.1"));
    assert!(router.should_bypass("::1"));

    // Exact domain bypass
    assert!(router.should_bypass("intranet.local"));

    // Suffix domain bypass
    assert!(router.should_bypass("server1.corp.internal"));
    assert!(router.should_bypass("api.corp.internal"));

    // External target routed to proxy
    assert!(!router.should_bypass("github.com"));
    assert_eq!(router.resolve("github.com"), Some(&cfg));

    // Bypassed target resolves to None (Direct)
    assert!(router.resolve("localhost").is_none());
}

#[test]
fn test_wpad_option_252_parsing() {
    let wpad_data = b"http://wpad.corp.internal/wpad.dat\0";
    let url = parse_wpad_option_252(wpad_data);
    assert_eq!(url, Some("http://wpad.corp.internal/wpad.dat"));

    let empty = b"";
    assert!(parse_wpad_option_252(empty).is_none());
}

#[test]
fn test_http_connect_builder_and_response_validation() {
    let mut buf = [0u8; 256];
    let len = build_http_connect_request("firmware.update.org", 443, &mut buf);
    assert!(len.is_ok());

    if let Ok(written) = len {
        let request_str = core::str::from_utf8(&buf[..written]);
        assert!(request_str.is_ok());
        if let Ok(s) = request_str {
            assert!(s.starts_with("CONNECT firmware.update.org:443 HTTP/1.1\r\n"));
            assert!(s.contains("Host: firmware.update.org:443\r\n"));
        }
    }

    // Valid response 200
    let resp_ok = b"HTTP/1.1 200 Connection Established\r\n\r\n";
    assert_eq!(validate_proxy_connect_response(resp_ok), Ok(200));

    // Proxy Auth Required 407
    let resp_auth = b"HTTP/1.1 407 Proxy Authentication Required\r\n\r\n";
    assert_eq!(
        validate_proxy_connect_response(resp_auth),
        Err(EfiStatus::ACCESS_DENIED)
    );

    // Malformed response
    let resp_bad = b"GARBAGE";
    assert_eq!(
        validate_proxy_connect_response(resp_bad),
        Err(EfiStatus::DEVICE_ERROR)
    );
}
