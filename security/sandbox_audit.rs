#[derive(Debug, Clone)]
pub struct SandboxAudit {
    pub enabled: bool,
    pub filesystem_access: bool,
    pub network_access: bool,
    pub device_access: bool,
}

impl SandboxAudit {
    pub fn new() -> Self {
        Self {
            enabled: false,
            filesystem_access: false,
            network_access: false,
            device_access: false,
        }
    }

    pub fn describe(&self) -> Vec<String> {
        let mut out = vec![];

        if !self.enabled {
            out.push("sandbox: disabled".into());
            return out;
        }

        out.push("sandbox: enabled".into());

        if self.filesystem_access {
            out.push("  - filesystem access".into());
        }
        if self.network_access {
            out.push("  - network access".into());
        }
        if self.device_access {
            out.push("  - device access".into());
        }

        out
    }
}
