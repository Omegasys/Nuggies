use crate::pkg::{manager::PackageManager, package::{Package, PackageFormat}};
use crate::core::system;

pub struct PacmanBackend;

impl PacmanBackend {
    pub fn new() -> Self {
        Self
    }
}

impl PackageManager for PacmanBackend {
    fn name(&self) -> &str {
        "pacman"
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, String> {
        let output = system::run_command("pacman", &["-Ss", query])?;

        Ok(vec![Package {
            name: output.lines().next().unwrap_or("").to_string(),
            version: None,
            description: None,
            format: PackageFormat::Native,
            dependencies: vec![],
            size: None,
            source: Some("pacman".into()),
            installed: false,
        }])
    }

    fn install(&self, package: &str) -> Result<(), String> {
        system::run_command("sudo", &["pacman", "-S", "--noconfirm", package])?;
        Ok(())
    }

    fn remove(&self, package: &str) -> Result<(), String> {
        system::run_command("sudo", &["pacman", "-R", "--noconfirm", package])?;
        Ok(())
    }

    fn update(&self, _package: Option<&str>) -> Result<(), String> {
        system::run_command("sudo", &["pacman", "-Syu", "--noconfirm"])?;
        Ok(())
    }

    fn info(&self, package: &str) -> Result<Package, String> {
        let output = system::run_command("pacman", &["-Qi", package])?;

        Ok(Package {
            name: package.to_string(),
            version: None,
            description: Some(output),
            format: PackageFormat::Native,
            dependencies: vec![],
            size: None,
            source: Some("pacman".into()),
            installed: true,
        })
    }
}
