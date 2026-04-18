#[derive(Debug, Clone)]
pub struct Rating {
    pub package: String,
    pub score: f32, // 0.0 - 5.0
    pub votes: u32,
}

impl Rating {
    pub fn new(package: &str, score: f32, votes: u32) -> Self {
        Self {
            package: package.to_string(),
            score: score.clamp(0.0, 5.0),
            votes,
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "{} rating: {:.1}/5 ({} votes)",
            self.package, self.score, self.votes
        )
    }
}
