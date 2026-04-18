#[derive(Debug, Clone)]
pub struct TrustScore {
    pub score: u8, // 0–100
}

impl TrustScore {
    pub fn new(score: u8) -> Self {
        Self {
            score: score.min(100),
        }
    }

    pub fn describe(&self) -> String {
        match self.score {
            80..=100 => "trust: high".into(),
            50..=79 => "trust: medium".into(),
            20..=49 => "trust: low".into(),
            _ => "trust: critical".into(),
        }
    }
}
