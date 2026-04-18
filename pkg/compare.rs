use crate::pkg::package::{Package, PackageFormat};

#[derive(Debug)]
pub struct Comparison {
    pub name: String,
    pub formats: Vec<Package>,
}

pub fn compare_packages(name: &str, packages: Vec<Package>) -> Comparison {
    Comparison {
        name: name.to_string(),
        formats: packages,
    }
}

pub fn print_comparison(comp: &Comparison) {
    println!("Comparison for: {}\n", comp.name);

    for pkg in &comp.formats {
        println!("Format: {:?}", pkg.format);
        println!("  Version: {:?}", pkg.version);
        println!("  Installed: {}", pkg.installed);
        println!("  Source: {:?}", pkg.source);
        println!("  Size: {:?}", pkg.size);
        println!();
    }
}
