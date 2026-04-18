use crate::core::permissions::Permissions;

#[derive(Debug)]
pub struct SandboxInfo {
    pub enabled: bool,
    pub permissions: Permissions,
}

impl SandboxInfo {
    pub fn new() -> Self {
        Self {
            enabled: false,
            permissions: Permissions::new(),
        }
    }

    pub fn describe(&self) -> String {
        if !self.enabled {
            return "No sandbox".into();
        }

        format!(
            "Sandbox enabled with permissions: {:?}",
            self.permissions.describe()
        )
    }
}
