pub fn run(package: &str, format: Option<String>, dry_run: bool) {
    println!("[nuggies] Install request");

    println!("Package: {}", package);
    println!("Format: {}", format.as_deref().unwrap_or("auto"));

    if dry_run {
        println!("[dry-run] Command preview:");
        println!("-> (not implemented)");
        return;
    }

    println!("Installing...");
    println!("-> (backend not implemented)");
}
