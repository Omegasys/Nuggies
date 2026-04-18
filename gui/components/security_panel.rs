pub struct SecurityPanel;

impl SecurityPanel {
    pub fn render(package: &str) {
        println!("🔐 Security Panel");

        println!("Package: {}", package);
        println!("---");

        println!("• signature: unknown (placeholder)");
        println!("• trust score: medium (placeholder)");
        println!("• sandbox profile: default");
        println!("• network access: not evaluated yet");
    }
}
