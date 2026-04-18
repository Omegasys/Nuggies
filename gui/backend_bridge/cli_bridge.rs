use std::process::Command;

pub struct CliBridge;

impl CliBridge {
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

    pub fn install_package(pkg: &str) -> Result<String, String> {
        Self::run_command("echo", &["install", pkg])
    }
}
