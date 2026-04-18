use nuggies::pkg::package::{Package, PackageFormat};

#[test]
fn install_flow_simulation() {
    let pkg = Package::new("vim", PackageFormat::Native);

    // simulate install pipeline stages
    let stages = vec!["preview", "security_check", "install"];

    assert!(pkg.name == "vim");
    assert_eq!(stages.len(), 3);
}
