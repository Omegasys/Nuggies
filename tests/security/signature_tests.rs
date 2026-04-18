use nuggies::security::signatures::{SignatureInfo, SignatureStatus};

#[test]
fn valid_signature_describes_correctly() {
    let sig = SignatureInfo::new(SignatureStatus::Valid, Some("nuggies-signing-key".into()));

    let desc = sig.describe();

    assert!(desc.contains("valid"));
    assert!(desc.contains("nuggies-signing-key"));
}

#[test]
fn missing_signature_is_detected() {
    let sig = SignatureInfo::new(SignatureStatus::Missing, None);

    let desc = sig.describe();

    assert!(desc.contains("missing"));
}
