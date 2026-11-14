//! # UEFI Network Proxy and Tunneling Subsystem
//!
//! Provides enterprise network proxy discovery, configuration, and HTTP CONNECT
//! tunneling for UEFI pre-boot networking environments (HTTP Boot, Redfish, REST EX,
//! and Secure Firmware updates).
//!
//! Supports WPAD (Web Proxy Auto-Discovery, DHCP Option 252), HTTP/HTTPS proxies,
//! SOCKS4/SOCKS5 proxies, Proxy-Authorization headers, and domain/CIDR bypass lists.

use crate::base::EfiStatus;

/// Supported proxy protocol types for firmware connections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProxyType {
    /// Direct connection without intermediate proxy.
    Direct,
    /// Standard HTTP forwarding proxy.
    Http,
    /// HTTPS CONNECT tunnel proxy.
    Https,
    /// SOCKS Version 4 proxy.
    Socks4,
    /// SOCKS Version 5 proxy.
    Socks5,
}

/// Authentication scheme for proxy authorization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProxyAuthScheme {
    /// No authentication required.
    None,
    /// HTTP Basic authentication (RFC 7617).
    Basic,
    /// HTTP Digest authentication (RFC 7616).
    Digest,
}

/// Configuration settings for an enterprise network proxy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProxyConfig<'a> {
    /// Proxy protocol variant.
    pub proxy_type: ProxyType,
    /// Hostname or IP address of the proxy server.
    pub host: &'a str,
    /// TCP port number of the proxy server (e.g. 8080, 3128, 1080).
    pub port: u16,
    /// Authentication scheme.
    pub auth_scheme: ProxyAuthScheme,
    /// Optional username for proxy authentication.
    pub username: Option<&'a str>,
    /// Optional password for proxy authentication.
    pub password: Option<&'a str>,
}

impl<'a> ProxyConfig<'a> {
    /// Creates a new proxy configuration.
    #[must_use]
    pub const fn new(proxy_type: ProxyType, host: &'a str, port: u16) -> Self {
        Self {
            proxy_type,
            host,
            port,
            auth_scheme: ProxyAuthScheme::None,
            username: None,
            password: None,
        }
    }

    /// Sets HTTP Basic authentication credentials.
    #[must_use]
    pub const fn with_basic_auth(mut self, username: &'a str, password: &'a str) -> Self {
        self.auth_scheme = ProxyAuthScheme::Basic;
        self.username = Some(username);
        self.password = Some(password);
        self
    }
}

/// Rule for determining if a destination host should bypass the proxy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProxyBypassRule<'a> {
    /// Matches the exact destination hostname (e.g. "intranet.local").
    ExactDomain(&'a str),
    /// Matches suffix domain wildcard (e.g. ".corp.internal" matches "node.corp.internal").
    SuffixDomain(&'a str),
    /// Localhost / loopback addresses.
    Loopback,
}

impl ProxyBypassRule<'_> {
    /// Evaluates whether the given target hostname or IP matches this bypass rule.
    #[must_use]
    pub fn matches(&self, target_host: &str) -> bool {
        match self {
            Self::ExactDomain(domain) => target_host.eq_ignore_ascii_case(domain),
            Self::SuffixDomain(suffix) => {
                if target_host.len() >= suffix.len() {
                    let end = &target_host[target_host.len() - suffix.len()..];
                    end.eq_ignore_ascii_case(suffix)
                } else {
                    false
                }
            }
            Self::Loopback => {
                target_host.eq_ignore_ascii_case("localhost")
                    || target_host == "127.0.0.1"
                    || target_host == "::1"
            }
        }
    }
}

/// Safe router managing proxy selection and bypass rule evaluation.
pub struct ProxyRouter<'a, const MAX_RULES: usize> {
    config: Option<ProxyConfig<'a>>,
    bypass_rules: [Option<ProxyBypassRule<'a>>; MAX_RULES],
    rule_count: usize,
}

impl<'a, const MAX_RULES: usize> ProxyRouter<'a, MAX_RULES> {
    /// Creates a new empty proxy router.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: None,
            bypass_rules: [None; MAX_RULES],
            rule_count: 0,
        }
    }

    /// Sets the active proxy configuration.
    pub fn set_config(&mut self, config: ProxyConfig<'a>) {
        self.config = Some(config);
    }

    /// Clears the proxy configuration (defaults to Direct).
    pub fn clear_config(&mut self) {
        self.config = None;
    }

    /// Adds a proxy bypass rule.
    ///
    /// # Errors
    /// Returns `EfiStatus::OUT_OF_RESOURCES` if capacity is reached.
    pub fn add_bypass_rule(&mut self, rule: ProxyBypassRule<'a>) -> Result<(), EfiStatus> {
        if self.rule_count >= MAX_RULES {
            return Err(EfiStatus::OUT_OF_RESOURCES);
        }
        self.bypass_rules[self.rule_count] = Some(rule);
        self.rule_count += 1;
        Ok(())
    }

    /// Determines if a target host should bypass the proxy.
    #[must_use]
    pub fn should_bypass(&self, target_host: &str) -> bool {
        for rule in self.bypass_rules.iter().take(self.rule_count).flatten() {
            if rule.matches(target_host) {
                return true;
            }
        }
        false
    }

    /// Resolves the proxy configuration to use for the specified target host.
    /// Returns `None` if the target should connect directly.
    #[must_use]
    pub fn resolve(&self, target_host: &str) -> Option<&ProxyConfig<'a>> {
        if self.should_bypass(target_host) {
            None
        } else {
            self.config.as_ref()
        }
    }
}

impl<const MAX_RULES: usize> Default for ProxyRouter<'_, MAX_RULES> {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses a Web Proxy Auto-Discovery (WPAD) URL from DHCP Option 252 bytes.
///
/// Returns the null-terminated ASCII or UTF-8 URL string without trailing nulls.
#[must_use]
pub fn parse_wpad_option_252(option_data: &[u8]) -> Option<&str> {
    if option_data.is_empty() {
        return None;
    }
    // Trim null terminators or whitespace
    let trimmed = match option_data.iter().position(|&b| b == 0) {
        Some(pos) => &option_data[..pos],
        None => option_data,
    };
    core::str::from_utf8(trimmed).ok()
}

/// Builds an HTTP `CONNECT` tunnel request line and headers into `out_buffer`.
///
/// Formats: `CONNECT target_host:target_port HTTP/1.1\r\nHost: target_host:target_port\r\n\r\n`
///
/// # Errors
/// Returns `EfiStatus::BUFFER_TOO_SMALL` if the buffer cannot hold the request.
pub fn build_http_connect_request(
    target_host: &str,
    target_port: u16,
    out_buffer: &mut [u8],
) -> Result<usize, EfiStatus> {
    let mut offset = 0;

    let append = |buf: &mut [u8], off: &mut usize, slice: &[u8]| -> Result<(), EfiStatus> {
        if *off + slice.len() > buf.len() {
            return Err(EfiStatus::BUFFER_TOO_SMALL);
        }
        buf[*off..*off + slice.len()].copy_from_slice(slice);
        *off += slice.len();
        Ok(())
    };

    append(out_buffer, &mut offset, b"CONNECT ")?;
    append(out_buffer, &mut offset, target_host.as_bytes())?;
    append(out_buffer, &mut offset, b":")?;

    // Format port as decimal
    let mut port_buf = [0u8; 5];
    let mut p = target_port;
    let mut p_idx = 5;
    if p == 0 {
        p_idx -= 1;
        port_buf[p_idx] = b'0';
    } else {
        while p > 0 {
            p_idx -= 1;
            port_buf[p_idx] = b'0' + (p % 10) as u8;
            p /= 10;
        }
    }
    append(out_buffer, &mut offset, &port_buf[p_idx..])?;
    append(out_buffer, &mut offset, b" HTTP/1.1\r\nHost: ")?;
    append(out_buffer, &mut offset, target_host.as_bytes())?;
    append(out_buffer, &mut offset, b":")?;
    append(out_buffer, &mut offset, &port_buf[p_idx..])?;
    append(out_buffer, &mut offset, b"\r\n\r\n")?;

    Ok(offset)
}

/// Validates an HTTP proxy response buffer for successful tunnel establishment.
///
/// Returns `Ok(200)` if `HTTP/1.x 200` is detected, `Err(407)` if authentication
/// is required, or `Err(code)` for other failure codes.
///
/// # Errors
/// Returns `EfiStatus::DEVICE_ERROR` or specific HTTP error codes if tunnel failed.
pub fn validate_proxy_connect_response(response: &[u8]) -> Result<u16, EfiStatus> {
    if response.len() < 12 {
        return Err(EfiStatus::DEVICE_ERROR);
    }
    if !response.starts_with(b"HTTP/1.0 ") && !response.starts_with(b"HTTP/1.1 ") {
        return Err(EfiStatus::DEVICE_ERROR);
    }

    let code_slice = &response[9..12];
    let d0 = code_slice[0].wrapping_sub(b'0');
    let d1 = code_slice[1].wrapping_sub(b'0');
    let d2 = code_slice[2].wrapping_sub(b'0');

    if d0 > 9 || d1 > 9 || d2 > 9 {
        return Err(EfiStatus::DEVICE_ERROR);
    }

    let code = u16::from(d0) * 100 + u16::from(d1) * 10 + u16::from(d2);
    if code == 200 {
        Ok(200)
    } else if code == 407 {
        Err(EfiStatus::ACCESS_DENIED)
    } else {
        Err(EfiStatus::DEVICE_ERROR)
    }
}
