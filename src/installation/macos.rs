use crate::{
    download_file, extract_zip, is_cmd_exists, run_command_on_unix, run_command_pipe_on_unix, ToolInstallationInfo,
    Type, ERROR_EMOJI, SPINNER_STYLE, SUCCESS_EMOJI,
};
use anyhow::Result;
use backtrace::Backtrace;
use console::style;
use indicatif::{MultiProgress, ProgressBar};
use std::clone::Clone;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::{path::Path, time::Duration};
use tokio::fs;
use walkdir::WalkDir;

pub async fn install(tools_installation_info: Vec<ToolInstallationInfo>) -> Result<()> {
    let multi_progress = MultiProgress::new();
    let tools_count = tools_installation_info.len();

    let installation_results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::with_capacity(tools_count)));

    let handles = tools_installation_info
        .into_iter()
        .enumerate()
        .map(|(index, tool_installation_info)| {
            // Change closure from FnMut to Fn
            let pb = multi_progress.add(ProgressBar::new(100));
            pb.set_style(SPINNER_STYLE.clone());
            pb.set_prefix(format!("[{}/{}]", index + 1, tools_count));
            pb.enable_steady_tick(Duration::from_millis(120));
            let installation_results = Arc::clone(&installation_results);
            tokio::spawn(async move {
                let mut installation_result: Option<Result<InstallStatus, anyhow::Error>> = None;
                match tool_installation_info.r#type {
                    Type::Zip => {
                        let zip_installation_result = install_tool_by_zip(
                            &tool_installation_info.id,
                            &tool_installation_info.source,
                            tool_installation_info.post_install.as_deref(),
                            |msg| pb.set_message(format!("{}: {}", style(&tool_installation_info.name).bold(), msg)),
                        )
                        .await;
                        installation_result = Some(zip_installation_result);
                    }
                    Type::Dmg => {
                        let dmg_installation_result = install_tool_by_dmg(
                            &tool_installation_info.id,
                            &tool_installation_info.source,
                            tool_installation_info.post_install.as_deref(),
                            |msg| pb.set_message(format!("{}: {}", style(&tool_installation_info.name).bold(), msg)),
                        )
                        .await;
                        installation_result = Some(dmg_installation_result);
                    }
                    Type::Shell => {
                        let shell_installation_result = install_tool_by_shell(
                            &tool_installation_info.id,
                            &tool_installation_info.source,
                            tool_installation_info.post_install.as_deref(),
                            |msg| pb.set_message(format!("{}: {}", style(&tool_installation_info.name).bold(), msg)),
                        )
                        .await;
                        installation_result = Some(shell_installation_result);
                    }
                    _ => {
                        pb.finish_with_message(format!(
                            "Unsupported installation type: {}",
                            style(&tool_installation_info.r#type).bold()
                        ));
                    }
                }
                if let Some(installation_result) = installation_result {
                    handle_installation_finish_message(
                        &pb,
                        &tool_installation_info.name,
                        installation_result,
                        &mut installation_results.lock().unwrap(),
                    );
                }
            })
        });
    futures::future::join_all(handles).await;
    // clear the progress bar
    multi_progress.clear().expect("Failed to clear progress bar");
    // print the installation results
    let installation_results = installation_results.lock().unwrap();
    for result in installation_results.iter() {
        println!("{}", result);
    }

    Ok(())
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
        match result.expect("result has error") {
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

enum InstallStatus {
    AlreadyInstalled,
    Installed,
}

async fn install_tool_by_zip(
    id: &str,
    source: &str,
    post_install: Option<&str>,
    message_callback: impl Fn(&str),
) -> Result<InstallStatus> {
    if is_app_installed(id) {
        Ok(InstallStatus::AlreadyInstalled)
    } else {
        message_callback("Downloading...");
        let zip_path = download_file(source).await?;
        message_callback("Extracting zip to `/Applications` directory...");
        extract_zip(&zip_path, "/Applications")?;
        if let Some(post_install) = post_install {
            run_command_on_unix(post_install)?;
        }
        if fs::try_exists(&zip_path).await? {
            fs::remove_file(&zip_path).await?;
        }

        Ok(InstallStatus::Installed)
    }
}

async fn install_tool_by_dmg(
    id: &str,
    source: &str,
    post_install: Option<&str>,
    message_callback: impl Fn(&str) + Clone,
) -> Result<InstallStatus> {
    if is_app_installed(id) {
        Ok(InstallStatus::AlreadyInstalled)
    } else {
        message_callback("Downloading...");
        let dmg_path = download_file(source).await?;

        install_dmg(id, &dmg_path, message_callback)?;

        if let Some(post_install) = post_install {
            run_command_on_unix(post_install)?;
        }

        fs::remove_file(dmg_path).await?;

        Ok(InstallStatus::Installed)
    }
}

async fn install_tool_by_shell(
    id: &str,
    source: &str,
    post_install: Option<&str>,
    message_callback: impl Fn(&str),
) -> Result<InstallStatus> {
    if is_cmd_exists(id)? {
        Ok(InstallStatus::AlreadyInstalled)
    } else {
        if let Some(post_install) = post_install {
            run_command_on_unix(post_install)?;
        }
        run_command_pipe_on_unix(source, message_callback)?;
        Ok(InstallStatus::Installed)
    }
}

fn install_dmg(id: &str, dmg_path: &Path, message_callback: impl Fn(&str) + Clone) -> Result<()> {
    // 1. mount the dmg
    message_callback("Mounting...");
    run_command_pipe_on_unix(
        &format!("hdiutil attach {}", dmg_path.to_str().unwrap()),
        message_callback.clone(),
    )?;
    message_callback("Mounted successfully!");

    let volumes_app_path = find_app(id).expect("failed to find the app in /Volumes");

    // 2. copy the app to /Applications
    message_callback("Copying to `/Applications` directory...");
    let command = format!(r#"cp -R "{}" /Applications"#, volumes_app_path.to_string_lossy());
    run_command_pipe_on_unix(&command, message_callback.clone())?;
    message_callback("Copied successfully!");

    // 3. unmount the dmg
    message_callback("Unmounting...");
    let volumes_app_parent_path = volumes_app_path
        .parent()
        .expect("failed to get volumes app's parent path")
        .to_string_lossy();
    let command = format!(r#"hdiutil detach "{}""#, volumes_app_parent_path);
    run_command_pipe_on_unix(&command, message_callback.clone())?;
    message_callback("Unmounted successfully!");

    Ok(())
}

fn find_app(app_file_name: &str) -> Result<PathBuf> {
    for entry in WalkDir::new("/Volumes").min_depth(1).max_depth(3) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        if entry.file_name().to_string_lossy() == app_file_name {
            return Ok(entry.path().to_owned());
        }
    }

    Err(anyhow::anyhow!("Failed to find the app in /Volumes"))
}

fn is_app_installed(id: &str) -> bool {
    // check if the dmg is installed
    let app_path_str = &format!("/Applications/{}", id);
    let path = Path::new(app_path_str);
    path.exists()
}
