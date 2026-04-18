use crate::pkg::{manager::PackageManager, package::{Package, PackageFormat}};
use crate::core::system;

pub struct SnapBackend;

impl SnapBackend {
    pub fn new() -> Self {
        Self
    }
}

impl PackageManager for SnapBackend {
    fn name(&self) -> &str {
        "snap"
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, String> {
        let output = system::run_command("snap", &["find", query])?;

        Ok(vec![Package {
            name: output.lines().next().unwrap_or("").to_string(),
            version: None,
            description: None,
            format: PackageFormat::Snap,
            dependencies: vec![],
            size: None,
            source: Some("snap store".into()),
            installed: false,
        }])
    }

    fn install(&self, package: &str) -> Result<(), String> {
        system::run_command("snap", &["install", package])?;
        Ok(())
    }

    fn remove(&self, package: &str) -> Result<(), String> {
        system::run_command("snap", &["remove", package])?;
        Ok(())
    }

    fn update(&self, _package: Option<&str>) -> Result<(), String> {
        system::run_command("snap", &["refresh"])?;
        Ok(())
    }

    fn info(&self, package: &str) -> Result<Package, String> {
        let output = system::run_command("snap", &["info", package])?;

        Ok(Package {
            name: package.to_string(),
            version: None,
            description: Some(output),
            format: PackageFormat::Snap,
            dependencies: vec![],
            size: None,
            source: Some("snap".into()),
            installed: true,
        })
    }
}
