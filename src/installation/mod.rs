mod linux;
mod macos;
mod windows;

use core::fmt;
use std::fmt::Display;
use std::path::Path;
use std::{env, str::FromStr};

use anyhow::Result;
use path_absolutize::*;
use tokio::fs;

use serde::{Deserialize, Serialize};

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

async fn get_toolkit_config(config_path: &str) -> Result<ToolsInstallationInfo> {
    let config = if config_path.starts_with("http") {
        let config: ToolsInstallationInfo = reqwest::get(config_path).await?.json().await?;
        config
    } else {
        let json = fs::read_to_string(Path::new(config_path).absolutize()?).await?;
        let config: ToolsInstallationInfo = serde_json::from_str(&json)?;
        config
    };

    Ok(config)
}

pub async fn install(config_path: &str) -> Result<()> {
    let tools_installation_info = get_toolkit_config(config_path).await?;
    let filter_tools_installation_info = filter_install_tools(&tools_installation_info)?;
    println!("filtered tools_installation_info: {:#?}", filter_tools_installation_info);
    match env::consts::OS {
        "macos" => {
            macos::install(filter_tools_installation_info).await?;
        }
        "linux" => {
            linux::install().await?;
        }
        "windows" => {
            windows::install().await?;
        }
        _ => return Err(anyhow::anyhow!("Unsupported OS {}", std::env::consts::OS)),
    };

    Ok(())
}

fn filter_install_tools(tools_installation_info: &ToolsInstallationInfo) -> Result<Vec<ToolInstallationInfo>> {
    let cur_arch: &str = env::consts::ARCH;
    let cur_os = env::consts::OS;

    let mut final_tools_installation_info: Vec<ToolInstallationInfo> = vec![];

    tools_installation_info.tools.iter().for_each(|tool| {
        tool.installations.iter().for_each(|installation| {
            let tool_installation_info = ToolInstallationInfo {
                name: tool.name.clone(),
                description: tool.description.clone(),
                os: installation.os,
                arch: installation.arch,
                id: installation.id.clone(),
                r#type: installation.r#type,
                source: installation.source.clone(),
                post_install: installation.post_install.clone(),
            };
            let is_os_matched =
                installation.os == OS::from_str(cur_os).expect("failed to convert `std::env::consts::OS` to OS enum");
            if let Some(arch) = &installation.arch {
                if is_os_matched
                    && *arch
                        == Arch::from_str(cur_arch).expect("failed to convert `std::env::consts::ARCH` to Arch enum")
                {
                    final_tools_installation_info.push(tool_installation_info);
                }
            } else if is_os_matched {
                final_tools_installation_info.push(tool_installation_info);
            }
        })
    });

    Ok(final_tools_installation_info)
}

#[cfg(test)]
mod test_get_toolkit_config_fn {
    use super::*;

    #[tokio::test]
    async fn test_get_config_with_local_file() -> Result<()> {
        let config = get_toolkit_config("./tools-installation-info.json").await?;
        assert!(config.tools.len() == 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_config_with_remote_file() -> Result<()> {
        let config = get_toolkit_config("https://gist.githubusercontent.com/luhc228/6980b3e72e66066c8d27ef7b3f66580b/raw/a47a3c0b68b5fedf62cd0f7a43c0bc2c224d4d60/toolkit.config.json").await?;
        assert!(config.tools.len() == 3);
        Ok(())
    }
}

#[cfg(target_os = "macos")]
#[cfg(test)]
mod test_install_fn_on_macos {
    use super::*;

    use crate::run_command_on_unix;

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

#[cfg(target_os = "windows")]
#[cfg(test)]
mod test_install_fn_on_windows {
    use super::*;

    #[tokio::test]
    async fn test_install_on_windows() -> Result<()> {
        super::install("./tools-installation-info.json").await?;
        Ok(())
    }
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod test_install_fn_on_linux {}
