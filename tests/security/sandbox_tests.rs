use nuggies::security::sandbox_audit::SandboxAudit;

#[test]
fn sandbox_disabled_by_default() {
    let sandbox = SandboxAudit::new();

    let desc = sandbox.describe();

    assert!(desc.contains("disabled"));
}

#[test]
fn sandbox_permissions_listed() {
    let mut sandbox = SandboxAudit::new();
    sandbox.enabled = true;
    sandbox.filesystem_access = true;

    let desc = sandbox.describe();

    assert!(desc.iter().any(|l| l.contains("filesystem")));
}
