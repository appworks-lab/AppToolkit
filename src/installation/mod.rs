mod linux;
mod macos;
mod toolkit_manifest;
mod windows;

use crate::{ERROR_EMOJI, SUCCESS_EMOJI};
use anyhow::Result;
use backtrace::Backtrace;
use console::style;
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fmt::{self, Display},
    str::FromStr,
};
use toolkit_manifest::{filter_tool_installation_detail, get_tookits_manifest};

#[derive(Debug, Deserialize, Serialize)]
pub struct ToolkitsManifest {
    pub author: String,
    pub version: String,
    pub description: String,
    pub tools: Vec<ToolInstallationManifest>,
}

impl Display for ToolkitsManifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            " author: {}, version: {}, description: {}, tools: {:?}",
            self.author, self.version, self.description, self.tools
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ToolInstallationManifest {
    pub name: String,
    pub description: String,
    pub installations: Vec<RawInstallationDetailItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawInstallationDetailItem {
    os: OS,
    arch: Option<Arch>,
    id: String,
    r#type: Type,
    source: String,
    #[serde(rename = "postInstall", default)]
    post_install: Option<String>,
}
#[derive(Debug)]
pub struct InstallationDetailItem {
    pub name: String,
    pub description: String,
    pub os: OS,
    pub arch: Option<Arch>,
    pub id: String,
    pub r#type: Type,
    pub source: String,
    pub post_install: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OS {
    MacOS,
    Linux,
    Windows,
}
impl FromStr for OS {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "macos" => Ok(OS::MacOS),
            "linux" => Ok(OS::Linux),
            "windows" => Ok(OS::Windows),
            _ => Err(anyhow::anyhow!("Unsupported OS {}", s)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
    Aarch64,
    X86_64,
}
impl FromStr for Arch {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "aarch64" => Ok(Arch::Aarch64),
            "x86_64" => Ok(Arch::X86_64),
            _ => Err(anyhow::anyhow!("Unsupported Arch {}", s)),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Dmg,
    Shell,
    Zip,
    Exe,
    Deb,
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Dmg => write!(f, "dmg"),
            Type::Shell => write!(f, "shell"),
            Type::Zip => write!(f, "zip"),
            Type::Exe => write!(f, "exe"),
            Type::Deb => write!(f, "deb"),
        }
    }
}

pub enum InstallStatus {
    AlreadyInstalled,
    Installed,
}

fn handle_installation_finish_message(
    pb: &ProgressBar,
    tool_name: &str,
    result: Result<InstallStatus>,
    installation_results: &mut Vec<String>,
) {
    if let Err(err) = result {
        let bt = Backtrace::new();
        pb.finish_with_message("waiting...");
        eprintln!("Error: {:?}\n Backtrace: {:?}", err, bt);
        installation_results.push(format!(
            "{} {}: Failed to install. Reason: {}",
            ERROR_EMOJI,
            style(tool_name).bold(),
            err
        ));
    } else {
        pb.finish_with_message("waiting...");
        match result.expect("installation result has error") {
            InstallStatus::AlreadyInstalled => {
                installation_results.push(format!(
                    "{} {}: Already installed",
                    SUCCESS_EMOJI,
                    style(tool_name).bold()
                ));
            }
            InstallStatus::Installed => {
                installation_results.push(format!(
                    "{} {}: Installed Successfully",
                    SUCCESS_EMOJI,
                    style(tool_name).bold()
                ));
            }
        }
    }
}

pub async fn install(manifest_path: &str) -> Result<()> {
    let toolkits_manifest = get_tookits_manifest(manifest_path).await?;
    let tools_installation_detail = filter_tool_installation_detail(&toolkits_manifest.tools)?;

    println!(
        "Using Toolkits Manifest:\n  Path:    {}\n  Version: {}\n  Author:  {}\n",
        manifest_path, toolkits_manifest.version, toolkits_manifest.author
    );

    match env::consts::OS {
        "macos" => {
            #[cfg(target_os = "macos")]
            macos::macos_installation::install(tools_installation_detail).await?;
        }
        "linux" => {
            linux::install().await?;
        }
        "windows" => {
            #[cfg(target_os = "windows")]
            windows::windows_installation::install(tools_installation_detail).await?;
        }
        _ => return Err(anyhow::anyhow!("Unsupported OS {}", std::env::consts::OS)),
    };

    Ok(())
}
