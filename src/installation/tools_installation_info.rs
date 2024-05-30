use anyhow::Result;
use path_absolutize::*;
use std::{env, path::Path, str::FromStr};
use tokio::fs;

use crate::{Arch, ToolInstallationInfo, ToolsInstallationInfo, OS};

pub async fn get_tools_installation_info(config_path: &str) -> Result<ToolsInstallationInfo> {
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

#[cfg(test)]
mod test_get_tools_installation_info {
    use super::*;

    #[tokio::test]
    async fn test_get_config_with_local_file() -> Result<()> {
        let config = get_tools_installation_info("./tools-installation-info.json").await?;
        assert!(config.tools.len() == 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_config_with_remote_file() -> Result<()> {
        let config = get_tools_installation_info("https://gist.githubusercontent.com/luhc228/6980b3e72e66066c8d27ef7b3f66580b/raw/a47a3c0b68b5fedf62cd0f7a43c0bc2c224d4d60/toolkit.config.json").await?;
        assert!(config.tools.len() == 3);
        Ok(())
    }
}

pub fn filter_tools_installation_info(
    tools_installation_info: &ToolsInstallationInfo,
) -> Result<Vec<ToolInstallationInfo>> {
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
