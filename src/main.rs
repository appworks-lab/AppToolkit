use clap::Parser;
use toolkit::{install, Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => match command {
            Commands::Install(opts) => {
                install(&opts.manifest).await?;
            }
        },
        None => {
            eprintln!("No command provided");
        }
    }

    Ok(())
}
