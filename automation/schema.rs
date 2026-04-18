#[derive(Debug, Clone)]
pub struct AutomationStep {
    pub action: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AutomationPipeline {
    pub name: String,
    pub steps: Vec<AutomationStep>,
}

impl AutomationPipeline {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            steps: vec![],
        }
    }

    pub fn add_step(&mut self, action: &str, args: Vec<&str>) {
        self.steps.push(AutomationStep {
            action: action.to_string(),
            args: args.into_iter().map(|s| s.to_string()).collect(),
        });
    }
}
