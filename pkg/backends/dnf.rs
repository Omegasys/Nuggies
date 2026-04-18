use crate::pkg::{manager::PackageManager, package::{Package, PackageFormat}};
use crate::core::system;

pub struct DnfBackend;

impl DnfBackend {
    pub fn new() -> Self {
        Self
    }
}

impl PackageManager for DnfBackend {
    fn name(&self) -> &str {
        "dnf"
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, String> {
        let output = system::run_command("dnf", &["search", query])?;

        Ok(vec![Package {
            name: output.lines().next().unwrap_or("").to_string(),
            version: None,
            description: None,
            format: PackageFormat::Native,
            dependencies: vec![],
            size: None,
            source: Some("dnf".into()),
            installed: false,
        }])
    }

    fn install(&self, package: &str) -> Result<(), String> {
        system::run_command("sudo", &["dnf", "install", "-y", package])?;
        Ok(())
    }

    fn remove(&self, package: &str) -> Result<(), String> {
        system::run_command("sudo", &["dnf", "remove", "-y", package])?;
        Ok(())
    }

    fn update(&self, _package: Option<&str>) -> Result<(), String> {
        system::run_command("sudo", &["dnf", "update", "-y"])?;
        Ok(())
    }

    fn info(&self, package: &str) -> Result<Package, String> {
        let output = system::run_command("dnf", &["info", package])?;

        Ok(Package {
            name: package.to_string(),
            version: None,
            description: Some(output),
            format: PackageFormat::Native,
            dependencies: vec![],
            size: None,
            source: Some("dnf".into()),
            installed: true,
        })
    }
}
