pub fn explain_package(name: &str) -> String {
    format!(
        "Explanation for '{}':\n\
        - This is a placeholder explanation\n\
        - Real implementation could use local model or offline dataset\n\
        - No network dependency assumed",
        name
    )
}
