use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "install", about = "Install toolkits.")]
    Install(InstallOpts),
    // TODO: Implement the command
    // #[command(name = "list", about = "Show the toolkits Details.")]
    // List(ListOpts),
}

#[derive(Parser, Debug)]
pub struct InstallOpts {
    #[arg(
        long,
        default_value = "https://raw.githubusercontent.com/apptools-lab/AppToolkit/feat/cli/toolkits.manifest.json",
        help = "Path to the toolkits manifest file. You can pass a URL to a remote manifest file or a file path to a local manifest file."
    )]
    pub manifest: String,
}

#[derive(Parser, Debug)]
pub struct GitSSHOpts {
    #[arg(long)]
    user_name: String,
    #[arg(long)]
    user_email: String,
}
