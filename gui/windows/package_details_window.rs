use crate::gui::state::GuiState;

pub struct PackageDetailsWindow;

impl PackageDetailsWindow {
    pub fn render(state: &GuiState) {
        println!("=== Package Details ===");

        if let Some(pkg) = &state.selected_package {
            println!("Package: {}", pkg);
            println!("\nTransparency:");
            println!("- install command preview");
            println!("- dependencies graph");
            println!("- build source info");

            println!("\nSecurity:");
            println!("- signature status");
            println!("- trust score");
            println!("- sandbox profile");
        } else {
            println!("No package selected.");
        }
    }
}
