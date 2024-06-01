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
    #[command(name = "git-ssh", about = "Generate Git SSH secret key")]
    GitSSH(GitSSHOpts),
}

#[derive(Parser, Debug)]
pub struct InstallOpts {
    #[arg(
        long,
        default_value = "./toolkits.manifest.json",
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
