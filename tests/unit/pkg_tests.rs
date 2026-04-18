use nuggies::pkg::package::{Package, PackageFormat};
use nuggies::pkg::version::Version;

#[test]
fn package_creation_works() {
    let pkg = Package::new("firefox", PackageFormat::Native);

    assert_eq!(pkg.name, "firefox");
    assert!(matches!(pkg.format, PackageFormat::Native));
}

#[test]
fn version_ordering() {
    let v1 = Version::new("1.0.0");
    let v2 = Version::new("2.0.0");

    assert!(v1 < v2);
}
