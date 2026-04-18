use crate::automation::schema::AutomationPipeline;

pub struct AutomationView;

impl AutomationView {
    pub fn render(pipeline: Option<&AutomationPipeline>) {
        println!("⚙️ Automation View");

        match pipeline {
            Some(p) => {
                println!("Pipeline: {}", p.name);

                for step in &p.steps {
                    println!("  - {} {:?}", step.action, step.args);
                }
            }
            None => {
                println!("No active automation pipeline");
                println!("Use nuggies.yaml or automation scripts");
            }
        }
    }
}
