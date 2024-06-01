use anyhow::Result;
use path_absolutize::*;
use std::{env, path::Path, str::FromStr};
use tokio::fs;

use crate::{Arch, InstallationDetailItem, ToolInstallationManifest, ToolkitsManifest, OS};

pub async fn get_tookits_manifest(manifest_path: &str) -> Result<ToolkitsManifest> {
    let manifest = if manifest_path.starts_with("http") {
        let manifest: ToolkitsManifest = reqwest::get(manifest_path).await?.json().await?;
        manifest
    } else {
        let json = fs::read_to_string(Path::new(manifest_path).absolutize()?).await?;
        let manifest: ToolkitsManifest = serde_json::from_str(&json)?;
        manifest
    };

    Ok(manifest)
}
// filter the tools based on the current OS and Arch
pub fn filter_tool_installation_detail(
    tools_installation_manifest: &[ToolInstallationManifest],
) -> Result<Vec<InstallationDetailItem>> {
    let cur_arch: &str = env::consts::ARCH;
    let cur_os = env::consts::OS;

    let mut filtered_tools_installation_detail: Vec<InstallationDetailItem> = vec![];

    tools_installation_manifest.iter().for_each(|tool| {
        tool.installations.iter().for_each(|installation| {
            let tool_installation_detail = InstallationDetailItem {
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
                    filtered_tools_installation_detail.push(tool_installation_detail);
                }
            } else if is_os_matched {
                filtered_tools_installation_detail.push(tool_installation_detail);
            }
        })
    });

    Ok(filtered_tools_installation_detail)
}

#[cfg(test)]
mod test_get_tookits_manifest {
    use super::*;

    #[tokio::test]
    async fn test_with_local_file() -> Result<()> {
        let toolkits_manifest = get_tookits_manifest("./toolkits.manifest.json").await?;
        assert!(toolkits_manifest.tools.len() == 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_with_remote_file() -> Result<()> {
        let toolkits_manifest = get_tookits_manifest(
            "https://raw.githubusercontent.com/apptools-lab/AppToolkit/feat/cli/toolkits.manifest.json",
        )
        .await?;
        assert!(toolkits_manifest.tools.len() == 3);
        Ok(())
    }
}
