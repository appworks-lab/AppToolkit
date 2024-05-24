mod linux;
mod macos;
mod tools_installation_info;
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
use tools_installation_info::{filter_tools_installation_info, get_tools_installation_info};

#[derive(Debug, Deserialize, Serialize)]
pub struct ToolsInstallationInfo {
    pub tools: Vec<Tool>,
    #[serde(rename = "postInstall", default)]
    pub install_type: InstallationType,
}

impl Display for ToolsInstallationInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ToolsInstallationInfo: {:?}", self.tools)
    }
}

// TODO: support sequential it in the future
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum InstallationType {
    #[default]
    Parallel,
    Sequential,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub installations: Vec<Installation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Installation {
    os: OS,
    arch: Option<Arch>,
    id: String,
    r#type: Type,
    source: String,
    #[serde(rename = "postInstall", default)]
    post_install: Option<String>,
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
#[derive(Debug)]
pub struct ToolInstallationInfo {
    pub name: String,
    pub description: String,
    pub os: OS,
    pub arch: Option<Arch>,
    pub id: String,
    pub r#type: Type,
    pub source: String,
    pub post_install: Option<String>,
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

pub async fn install(config_path: &str) -> Result<()> {
    let tools_installation_info = get_tools_installation_info(config_path).await?;
    let tools_installation_info = filter_tools_installation_info(&tools_installation_info)?;

    match env::consts::OS {
        "macos" => {
            #[cfg(target_os = "macos")]
            macos::macos_installation::install(tools_installation_info).await?;
        }
        "linux" => {
            linux::install().await?;
        }
        "windows" => {
            #[cfg(target_os = "windows")]
            windows::windows_installation::install(tools_installation_info).await?;
        }
        _ => return Err(anyhow::anyhow!("Unsupported OS {}", std::env::consts::OS)),
    };

    Ok(())
}

#[cfg(target_os = "macos")]
#[cfg(test)]
mod test_install_fn_on_macos {
    use super::*;
    use crate::run_command_on_unix;
    use std::path::Path;

    #[tokio::test]
    async fn test_install_on_mac() -> Result<()> {
        install("./tools-installation-info.json").await?;
        check_path_existence("/Applications/Google Chrome.app")?;
        check_path_existence("/Applications/Visual Studio Code.app")?;
        check_script_existence("which fnm")?;
        Ok(())
    }

    fn check_script_existence(command: &str) -> Result<()> {
        let output = run_command_on_unix(command)?;
        assert_eq!(output.status.code(), Some(0), "Command {:?} not found", command);
        Ok(())
    }
    fn check_path_existence(path: &str) -> Result<()> {
        let path = Path::new(path);
        assert!(path.exists(), "Path {:?} does not exist", path);
        Ok(())
    }
}
