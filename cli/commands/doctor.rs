use std::process::Command;

pub fn run() {
    println!("[nuggies doctor] Running diagnostics...");

    check("cargo");
    check("git");
    check("flatpak");
    check("snap");
    check("apt");
    check("dnf");
    check("pacman");
}

fn check(cmd: &str) {
    let result = Command::new("which").arg(cmd).output();

    match result {
        Ok(output) if output.status.success() => {
            println!("[ok] {}", cmd);
        }
        _ => {
            println!("[missing] {}", cmd);
        }
    }
}
