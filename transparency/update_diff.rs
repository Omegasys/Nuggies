#[derive(Debug)]
pub struct UpdateDiff {
    pub package: String,
    pub old_version: String,
    pub new_version: String,
}

impl UpdateDiff {
    pub fn new(package: &str, old: &str, new: &str) -> Self {
        Self {
            package: package.to_string(),
            old_version: old.to_string(),
            new_version: new.to_string(),
        }
    }

    pub fn describe(&self) {
        println!("[update diff] {}", self.package);
        println!("  - old: {}", self.old_version);
        println!("  - new: {}", self.new_version);
    }
}
