#[derive(Debug)]
pub struct PermissionDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

impl PermissionDiff {
    pub fn new() -> Self {
        Self {
            added: vec![],
            removed: vec![],
        }
    }

    pub fn describe(&self) {
        println!("[permissions diff]");

        if !self.added.is_empty() {
            println!("  + Added:");
            for p in &self.added {
                println!("    - {}", p);
            }
        }

        if !self.removed.is_empty() {
            println!("  - Removed:");
            for p in &self.removed {
                println!("    - {}", p);
            }
        }

        if self.added.is_empty() && self.removed.is_empty() {
            println!("  (no permission changes)");
        }
    }
}
