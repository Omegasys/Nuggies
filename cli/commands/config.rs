use std::env;

pub fn run() {
    println!("[nuggies] Current configuration:");

    print_env("NUGGIES_DEBUG");
    print_env("NUGGIES_LOG_LEVEL");
    print_env("NUGGIES_DEFAULT_FORMAT");
    print_env("NUGGIES_ENABLE_SNAP");
    print_env("NUGGIES_OFFLINE_MODE");
}

fn print_env(key: &str) {
    match env::var(key) {
        Ok(val) => println!("{} = {}", key, val),
        Err(_) => println!("{} = (not set)", key),
    }
}
