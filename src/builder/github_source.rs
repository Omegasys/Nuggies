use std::process::Command;

pub fn clone_repo(owner: &str, repo: &str, path: &str) -> Result<(), String> {
    let url = format!("https://github.com/{}/{}.git", owner, repo);

    let output = Command::new("git")
        .args(["clone", &url, path])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("git clone failed".into());
    }

    Ok(())
}

pub fn install_from_github(owner: &str, repo: &str) -> Result<(), String> {
    let path = format!("/tmp/{}_{}", owner, repo);

    clone_repo(owner, repo, &path)?;

    // VERY simple detection (expand later)
    if std::path::Path::new(&format!("{}/Cargo.toml", path)).exists() {
        println!("Detected Rust project → cargo build");
        Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(&path)
            .status()
            .map_err(|e| e.to_string())?;
    } else {
        println!("Unknown project type");
    }

    Ok(())
}
