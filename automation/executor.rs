use crate::automation::schema::AutomationPipeline;
use crate::core::system;

pub fn execute_pipeline(pipeline: &AutomationPipeline) -> Result<(), String> {
    println!("[automation] running pipeline: {}", pipeline.name);

    for step in &pipeline.steps {
        println!(
            "[step] {} {:?}",
            step.action,
            step.args
        );

        match step.action.as_str() {
            "install" => {
                if let Some(pkg) = step.args.get(0) {
                    system::run_command("nuggies", &["install", pkg])?;
                }
            }

            "remove" => {
                if let Some(pkg) = step.args.get(0) {
                    system::run_command("nuggies", &["remove", pkg])?;
                }
            }

            "update" => {
                system::run_command("nuggies", &["update"])?;
            }

            _ => {
                println!("[warning] unknown action: {}", step.action);
            }
        }
    }

    Ok(())
}
