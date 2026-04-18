use crate::security::{
    signatures::SignatureInfo,
    trust_score::TrustScore,
    sandbox_audit::SandboxAudit,
    network_monitor::NetworkActivity,
};

#[derive(Debug)]
pub struct SecurityReport {
    pub signature: SignatureInfo,
    pub trust: TrustScore,
    pub sandbox: SandboxAudit,
    pub network: NetworkActivity,
}

impl SecurityReport {
    pub fn new(
        signature: SignatureInfo,
        trust: TrustScore,
        sandbox: SandboxAudit,
        network: NetworkActivity,
    ) -> Self {
        Self {
            signature,
            trust,
            sandbox,
            network,
        }
    }

    pub fn print(&self) {
        println!("=== Security Report ===");
        println!("{}", self.signature.describe());
        println!("{}", self.trust.describe());

        for line in self.sandbox.describe() {
            println!("{}", line);
        }

        println!("{}", self.network.describe());
    }
}
