//! X.509 signature verification tests using an independently generated certificate.

use rover_uefi::protocols::security::rsa::parse_rsa_public_key;
use rover_uefi::protocols::security::x509::{
    ChainError, MAX_CHAIN_LENGTH, parse_certificate, verify_chain,
};

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let clean: String = hex.chars().filter(char::is_ascii_hexdigit).collect();
    clean
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            let Ok(text) = std::str::from_utf8(pair) else {
                panic!("invalid hex literal")
            };
            let Ok(value) = u8::from_str_radix(text, 16) else {
                panic!("invalid hex digit")
            };
            value
        })
        .collect()
}

const CERT_DER: &str = "308203233082020BA003020102021419508A58388A384B042D2120F7A1FBDE5547E4BA300D06092A864886F70D01010B05003021311F301D06035504030C16526F7665722054657374204365727469666963617465301E170D3236313130373032323431395A170D3336313130343032323431395A3021311F301D06035504030C16526F766572205465737420436572746966696361746530820122300D06092A864886F70D01010105000382010F003082010A0282010100D8CA8590CBFE1BA8930F8AD9E5C1308BE66C11FC210462FB3668B91D34B5D3CD12C6311279039F984FFBBC6ECD27FBB7F830D316E6D3B4773E656444D6D85D7983C1858F086924EEB1B33115DD00FF16C5241CB4085FBE1FE2C6767171DDC91928EC6A502C453C24A2DA6BEC22AB35BEB0A9AB14730F68CE97E613B26A3313AB078D757A362D8A29D71872264A1E9D791C08CD80A02AB78C1AD7CC76687C3831EC2CBA0C04318A4C540BE6C85FCC96559B859EDD1B420678811F6EF2C2F7983C2085F6D44FCE388813364869FFF11F5BFAEBE41930BF36D4FC0C5C86B412E82135D912FA05C3542DBD0C1A6AD8F28C7C4EB22AADEFB4E0AB1CF3FBB87B08A2730203010001A3533051301D0603551D0E041604147FD62E8CE995C1E1675071D68378D4AD848DA82F301F0603551D230418301680147FD62E8CE995C1E1675071D68378D4AD848DA82F300F0603551D130101FF040530030101FF300D06092A864886F70D01010B050003820101004980E360BDBED7C3C5FEBC029B44293C66AC49E8C9F9A274BB1D72C6D5FB2942603DF2A4CEB62E89C5966C45551A95B72D972C29BFA1F70D22050DA46B48606EA5684F17CF13BE8F0F2004FE075D5E582C8F6FCC0313AB917B8F13A0B209DBF9C2584893CBA1861478BA75E62D4CBC08853B4FDEF1B3D0672CD50FF1FDDD518AF255FFA6EF8CF7D33A0582AA703AA22870ED796D42A63C161B05ED1422869FA7FB0BF617BD7FF4CDA37AB2B777C55BF48104DAF54130F82CF6DFDEA4B65C8F82CA78E7077CDD6865D7BEBC59B8122138BD84D7A90F3AB1F7DC17CA701017AE51049776AE953ED73DD108F57A2CA55152E8DC1D7E66946DEAC665BD113436378C";

#[test]
fn x509_self_signed_signature_verification() {
    let der = hex_to_bytes(CERT_DER);
    let Ok(certificate) = parse_certificate(&der) else {
        panic!("certificate did not parse")
    };
    assert_eq!(certificate.version, Some(2));
    assert!(certificate.verify_signature(&certificate));
    assert!(certificate.is_valid_at(1_900_000_000));
    assert!(!certificate.is_valid_at(0));
    assert!(!certificate.is_valid_at(2_200_000_000));
    let Some(spki) = certificate.subject_public_key_info() else {
        panic!("missing subject public key information")
    };
    assert!(parse_rsa_public_key(spki).is_ok());
}

#[test]
fn x509_tampered_tbs_fails_signature() {
    let mut der = hex_to_bytes(CERT_DER);
    der[20] ^= 0x01;
    let Ok(certificate) = parse_certificate(&der) else {
        panic!("tampered certificate did not parse")
    };
    assert!(!certificate.verify_signature(&certificate));
}

/// The reference time inside the validity interval of `CERT_DER`.
const REFERENCE_TIME: i64 = 1_900_000_000;

#[test]
fn chain_accepts_a_self_signed_anchor() {
    let der = hex_to_bytes(CERT_DER);
    let Ok(certificate) = parse_certificate(&der) else {
        panic!("certificate did not parse")
    };
    let result = verify_chain(
        &certificate,
        &[],
        std::slice::from_ref(&certificate),
        REFERENCE_TIME,
    );
    assert_eq!(result, Ok(1));
}

#[test]
fn chain_requires_a_trust_anchor() {
    let der = hex_to_bytes(CERT_DER);
    let Ok(certificate) = parse_certificate(&der) else {
        panic!("certificate did not parse")
    };
    assert_eq!(
        verify_chain(&certificate, &[], &[], REFERENCE_TIME),
        Err(ChainError::NoTrustAnchor)
    );
}

#[test]
fn chain_rejects_a_certificate_outside_its_validity() {
    let der = hex_to_bytes(CERT_DER);
    let Ok(certificate) = parse_certificate(&der) else {
        panic!("certificate did not parse")
    };
    assert_eq!(
        verify_chain(&certificate, &[], std::slice::from_ref(&certificate), 0),
        Err(ChainError::Expired)
    );
    assert_eq!(
        verify_chain(
            &certificate,
            &[],
            std::slice::from_ref(&certificate),
            2_200_000_000
        ),
        Err(ChainError::Expired)
    );
}

#[test]
fn chain_rejects_a_tampered_signature() {
    let der = hex_to_bytes(CERT_DER);
    let Ok(certificate) = parse_certificate(&der) else {
        panic!("certificate did not parse")
    };
    let mut tampered_der = hex_to_bytes(CERT_DER);
    tampered_der[20] ^= 0x01;
    let Ok(tampered) = parse_certificate(&tampered_der) else {
        panic!("tampered certificate did not parse")
    };
    assert_eq!(
        verify_chain(&tampered, &[], &[certificate], REFERENCE_TIME),
        Err(ChainError::SignatureInvalid)
    );
}

#[test]
fn chain_reports_an_unknown_issuer() {
    let der = hex_to_bytes(CERT_DER);
    let Ok(certificate) = parse_certificate(&der) else {
        panic!("certificate did not parse")
    };
    let mut leaf = certificate.clone();
    leaf.issuer = b"\x30\x00not the same name";
    assert_eq!(
        verify_chain(&leaf, &[], &[certificate], REFERENCE_TIME),
        Err(ChainError::IssuerNotFound)
    );
}

#[test]
fn chain_rejects_an_oversized_candidate_set() {
    let der = hex_to_bytes(CERT_DER);
    let Ok(certificate) = parse_certificate(&der) else {
        panic!("certificate did not parse")
    };
    let candidates = vec![certificate.clone(); MAX_CHAIN_LENGTH + 1];
    assert_eq!(
        verify_chain(
            &certificate,
            &candidates,
            std::slice::from_ref(&certificate),
            REFERENCE_TIME
        ),
        Err(ChainError::ChainTooLong)
    );
}

#[test]
fn chain_accepts_the_maximum_candidate_set() {
    let der = hex_to_bytes(CERT_DER);
    let Ok(certificate) = parse_certificate(&der) else {
        panic!("certificate did not parse")
    };
    // The self-signed certificate anchors directly, so the unrelated
    // candidates are never visited.
    let candidates = vec![certificate.clone(); MAX_CHAIN_LENGTH];
    assert_eq!(
        verify_chain(
            &certificate,
            &candidates,
            std::slice::from_ref(&certificate),
            REFERENCE_TIME
        ),
        Ok(1)
    );
}
