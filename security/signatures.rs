#[derive(Debug, Clone)]
pub enum SignatureStatus {
    Valid,
    Invalid,
    Unknown,
    Missing,
}

#[derive(Debug, Clone)]
pub struct SignatureInfo {
    pub status: SignatureStatus,
    pub signer: Option<String>,
}

impl SignatureInfo {
    pub fn new(status: SignatureStatus, signer: Option<String>) -> Self {
        Self { status, signer }
    }

    pub fn describe(&self) -> String {
        match &self.status {
            SignatureStatus::Valid => {
                format!("signature: valid ({})", self.signer.as_deref().unwrap_or("unknown"))
            }
            SignatureStatus::Invalid => "signature: INVALID".into(),
            SignatureStatus::Unknown => "signature: unknown".into(),
            SignatureStatus::Missing => "signature: missing".into(),
        }
    }
}
