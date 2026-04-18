use crate::pkg::{manager::PackageManager, package::{Package, PackageFormat}};
use crate::core::system;

pub struct AptBackend;

impl AptBackend {
    pub fn new() -> Self {
        Self
    }
}

impl PackageManager for AptBackend {
    fn name(&self) -> &str {
        "apt"
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, String> {
        let output = system::run_command("apt", &["search", query])?;

        Ok(vec![Package {
            name: output.lines().next().unwrap_or("").to_string(),
            version: None,
            description: None,
            format: PackageFormat::Native,
            dependencies: vec![],
            size: None,
            source: Some("apt".into()),
            installed: false,
        }])
    }

    fn install(&self, package: &str) -> Result<(), String> {
        system::run_command("sudo", &["apt", "install", "-y", package])?;
        Ok(())
    }

    fn remove(&self, package: &str) -> Result<(), String> {
        system::run_command("sudo", &["apt", "remove", "-y", package])?;
        Ok(())
    }

    fn update(&self, _package: Option<&str>) -> Result<(), String> {
        system::run_command("sudo", &["apt", "update"])?;
        Ok(())
    }

    fn info(&self, package: &str) -> Result<Package, String> {
        let output = system::run_command("apt", &["show", package])?;

        Ok(Package {
            name: package.to_string(),
            version: None,
            description: Some(output),
            format: PackageFormat::Native,
            dependencies: vec![],
            size: None,
            source: Some("apt".into()),
            installed: true,
        })
    }
}
