#[test]
fn rollback_flow_simulation() {
    let versions = vec!["1.0.0", "1.1.0", "1.2.0"];

    let rollback_target = versions[0];

    assert_eq!(rollback_target, "1.0.0");
}
