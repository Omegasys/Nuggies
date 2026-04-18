pub trait BuildStrategy {
    fn detect(path: &str) -> bool;
    fn build(path: &str) -> Result<(), String>;
}
