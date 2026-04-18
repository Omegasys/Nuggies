pub struct UpdatesView;

impl UpdatesView {
    pub fn render() {
        println!("⬆️ Updates View");

        println!("Available updates:");

        println!("  - firefox → 121.0");
        println!("  - vim → 9.1");
        println!("  - rust → latest stable");

        println!("\nActions:");
        println!("- update all");
        println!("- selective update");
        println!("- rollback view");
    }
}
