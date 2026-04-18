#[derive(Debug, Clone)]
pub struct Permissions {
    pub network: bool,
    pub filesystem: bool,
    pub devices: bool,
}

impl Permissions {
    pub fn new() -> Self {
        Self {
            network: false,
            filesystem: false,
            devices: false,
        }
    }

    pub fn describe(&self) -> Vec<String> {
        let mut perms = Vec::new();

        if self.network {
            perms.push("network".into());
        }
        if self.filesystem {
            perms.push("filesystem".into());
        }
        if self.devices {
            perms.push("devices".into());
        }

        if perms.is_empty() {
            perms.push("none".into());
        }

        perms
    }
}
