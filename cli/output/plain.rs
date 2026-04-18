pub fn print_line(message: &str) {
    println!("{}", message);
}

pub fn print_kv(key: &str, value: &str) {
    println!("{}: {}", key, value);
}

pub fn print_list(items: &[String]) {
    for item in items {
        println!("{}", item);
    }
}
