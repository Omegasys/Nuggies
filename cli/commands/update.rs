pub fn run(package: Option<String>) {
    match package {
        Some(pkg) => println!("[nuggies] Updating package: {}", pkg),
        None => println!("[nuggies] Updating all packages"),
    }

    println!("-> (backend not implemented)");
}
