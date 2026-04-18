pub struct DependencyGraph;

impl DependencyGraph {
    pub fn render(package: &str) {
        println!("🌲 Dependency Graph for {}", package);

        println!("  ├─ dependency-a");
        println!("  ├─ dependency-b");
        println!("  │   └─ sub-dependency");
        println!("  └─ dependency-c");
    }
}
