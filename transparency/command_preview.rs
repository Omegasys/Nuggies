pub fn preview_install_command(pkg_manager: &str, package: &str) -> String {
    format!(
        "[preview] {} install {}",
        pkg_manager, package
    )
}

pub fn preview_remove_command(pkg_manager: &str, package: &str) -> String {
    format!(
        "[preview] {} remove {}",
        pkg_manager, package
    )
}

pub fn preview_update_command(pkg_manager: &str, package: Option<&str>) -> String {
    match package {
        Some(p) => format!("[preview] {} update {}", pkg_manager, p),
        None => format!("[preview] {} update (all packages)", pkg_manager),
    }
}
