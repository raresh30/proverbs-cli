use clap::{Parser, Subcommand, Args};

mod proverbs;
mod commands;

#[derive(Parser)]
#[command(name = "proverbs", about = "A CLI tool for proverbs")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Search(SearchArgs),
    List,
}

#[derive(Args)]
struct SearchArgs {
    pattern: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proverbs = crate::proverbs::load_proverbs().expect("Couldn't load proverbs");

    let cli = Cli::parse();
    match cli.command {
        Command::Search(args) => commands::search(&proverbs, args.pattern),
        Command::List => commands::list(&proverbs),
    }

    Ok(())
}
