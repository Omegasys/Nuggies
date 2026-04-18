use crate::pkg::{manager::PackageManager, package::{Package, PackageFormat}};
use crate::core::system;

pub struct FlatpakBackend;

impl FlatpakBackend {
    pub fn new() -> Self {
        Self
    }
}

impl PackageManager for FlatpakBackend {
    fn name(&self) -> &str {
        "flatpak"
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, String> {
        let output = system::run_command("flatpak", &["search", query])?;

        Ok(vec![Package {
            name: output.lines().next().unwrap_or("").to_string(),
            version: None,
            description: None,
            format: PackageFormat::Flatpak,
            dependencies: vec![],
            size: None,
            source: Some("flatpak remote".into()),
            installed: false,
        }])
    }

    fn install(&self, package: &str) -> Result<(), String> {
        system::run_command("flatpak", &["install", "-y", package])?;
        Ok(())
    }

    fn remove(&self, package: &str) -> Result<(), String> {
        system::run_command("flatpak", &["uninstall", "-y", package])?;
        Ok(())
    }

    fn update(&self, _package: Option<&str>) -> Result<(), String> {
        system::run_command("flatpak", &["update", "-y"])?;
        Ok(())
    }

    fn info(&self, package: &str) -> Result<Package, String> {
        let output = system::run_command("flatpak", &["info", package])?;

        Ok(Package {
            name: package.to_string(),
            version: None,
            description: Some(output),
            format: PackageFormat::Flatpak,
            dependencies: vec![],
            size: None,
            source: Some("flatpak".into()),
            installed: true,
        })
    }
}
