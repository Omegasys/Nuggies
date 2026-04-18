use std::process::Command;

pub fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn detect_package_managers() -> Vec<String> {
    let mut managers = Vec::new();

    if command_exists("apt") {
        managers.push("apt".into());
    }
    if command_exists("dnf") {
        managers.push("dnf".into());
    }
    if command_exists("pacman") {
        managers.push("pacman".into());
    }
    if command_exists("flatpak") {
        managers.push("flatpak".into());
    }
    if command_exists("snap") {
        managers.push("snap".into());
    }

    managers
}

pub fn run_command(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
