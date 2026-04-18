use std::process::Command;

pub fn fetch_build_logs(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);

            format!(
                "=== STDOUT ===\n{}\n\n=== STDERR ===\n{}",
                stdout, stderr
            )
        }
        Err(e) => format!("[error fetching logs] {}", e),
    }
}
