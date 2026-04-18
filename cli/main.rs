use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "nuggies")]
#[command(about = "Transparent Linux package interface", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Search { query: String },
    Install {
        package: String,
        #[arg(long)]
        format: Option<String>,
        #[arg(long)]
        dry_run: bool,
    },
    Remove { package: String },
    Update { package: Option<String> },
    Rollback { package: String, version: String },
    Compare { package: String },
    Explain { package: String },
    Doctor,
    Config,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search { query } => commands::search::run(&query),
        Commands::Install { package, format, dry_run } => {
            commands::install::run(&package, format, dry_run)
        }
        Commands::Remove { package } => commands::remove::run(&package),
        Commands::Update { package } => commands::update::run(package),
        Commands::Rollback { package, version } => {
            commands::rollback::run(&package, &version)
        }
        Commands::Compare { package } => commands::compare::run(&package),
        Commands::Explain { package } => commands::explain::run(&package),
        Commands::Doctor => commands::doctor::run(),
        Commands::Config => commands::config::run(),
    }
}
