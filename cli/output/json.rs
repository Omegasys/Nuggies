pub fn print_json(key: &str, value: &str) {
    println!(
        "{{\n  \"{}\": \"{}\"\n}}",
        escape(key),
        escape(value)
    );
}

pub fn print_json_map(map: &[(&str, &str)]) {
    println!("{{");

    for (i, (k, v)) in map.iter().enumerate() {
        let comma = if i == map.len() - 1 { "" } else { "," };
        println!(
            "  \"{}\": \"{}\"{}",
            escape(k),
            escape(v),
            comma
        );
    }

    println!("}}");
}

fn escape(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}
