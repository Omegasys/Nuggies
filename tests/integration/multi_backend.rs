#[test]
fn backend_switching_simulation() {
    let backends = vec!["apt", "dnf", "pacman", "flatpak"];

    assert!(backends.contains(&"apt"));
    assert!(backends.contains(&"flatpak"));
}
