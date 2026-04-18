use crate::automation::schema::{AutomationPipeline, AutomationStep};

pub fn parse_script(name: &str, lines: Vec<&str>) -> AutomationPipeline {
    let mut pipeline = AutomationPipeline::new(name);

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        let action = parts[0];
        let args = parts[1..].to_vec();

        pipeline.steps.push(AutomationStep {
            action: action.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
        });
    }

    pipeline
}
