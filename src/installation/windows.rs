#[cfg(target_os = "windows")]
pub mod windows {
    extern crate winreg;

    use std::{collections::HashSet, sync::{Arc, Mutex}, time::Duration};

    use console::style;
    use indicatif::{MultiProgress, ProgressBar};
    use winreg::RegKey;
    use winreg::enums::*;
    use winreg::HKEY;
    use anyhow::Result;

    use crate::{download_file, installation::handle_installation_finish_message, run_command_on_windows, InstallStatus, ToolInstallationInfo, Type, SPINNER_STYLE};

    pub async fn install(tools_installation_info: Vec<ToolInstallationInfo>) -> Result<()> {
        let multi_progress = MultiProgress::new();
        let tools_count = tools_installation_info.len();

        let installation_results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::with_capacity(tools_count)));

        let installed_app_display_names = get_installed_app_display_names()?;

        let handles = tools_installation_info
            .into_iter()
            .enumerate()
            .map(|(index, tool_installation_info)| {
                let pb = multi_progress.add(ProgressBar::new(100));
                pb.set_style(SPINNER_STYLE.clone());
                pb.set_prefix(format!("[{}/{}]", index + 1, tools_count));
                pb.enable_steady_tick(Duration::from_millis(120));

                let installation_results = Arc::clone(&installation_results);
                let installed_app_display_names = installed_app_display_names.clone();

                tokio::spawn(async move {
                    let mut installation_result: Option<Result<InstallStatus, anyhow::Error>> = None;

                    match tool_installation_info.r#type {
                        Type::Exe => {
                            let exe_installation_result = install_tool_by_exe(
                                &tool_installation_info.id,
                                &tool_installation_info.source,
                                tool_installation_info.post_install.as_deref(),
                                &installed_app_display_names,
                                |msg| pb.set_message(format!("{}: {}", style(&tool_installation_info.name).bold(), msg)),
                            ).await;
                            installation_result = Some(exe_installation_result);
                        },
                        _ => {
                            let errror_message = format!(
                                "Unsupported installation type: {}. App: {}",
                                style(&tool_installation_info.r#type).bold(),
                                style(&tool_installation_info.name).bold(),
                            );
                            pb.finish_with_message(errror_message.clone());
                            installation_results.lock().unwrap().push(errror_message);
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
        multi_progress.clear().expect("failed to clear progress bar");
        // print the installation results
        let installation_results = installation_results.lock().unwrap();
        for result in installation_results.iter() {
            println!("{}", result);
        }

        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn get_installed_app_display_names() -> Result<HashSet<String>> {
        let mut display_names_set: HashSet<String> = HashSet::new();

        let paths: Vec<(HKEY, &str)> = vec![
            (HKEY_CURRENT_USER, "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall"),
            (HKEY_LOCAL_MACHINE, "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall"),
            // (HKEY_LOCAL_MACHINE, "Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall"),
        ];

        for path in paths {
            let display_names = get_app_display_names(path);
            for display_name in display_names {
                if !display_names_set.contains(&display_name) {
                    display_names_set.insert(display_name);
                }
            }
        }
        Ok(display_names_set)
    }

    fn get_app_display_names(path: (HKEY, &str)) -> Vec<String> {
        let (hkey, path) = path;
        let mut display_names: Vec<String> = Vec::new();

        let hkcu = RegKey::predef(hkey);
        let uninstall = hkcu.open_subkey_with_flags(path, KEY_READ).expect("failed to open uninstall key");

        for key_result in uninstall.enum_keys().map(|x| x.unwrap()) {
            let key: RegKey = uninstall.open_subkey_with_flags(&key_result, KEY_READ).unwrap();

            if let Ok(display_name) = key.get_value::<String, _>("DisplayName") {
                display_names.push(display_name);
            }
        }
        display_names
    }

    async fn install_tool_by_exe(
        id: &str,
        source: &str,
        post_install: Option<&str>,
        installed_app_display_names: &HashSet<String>,
        set_process_message: impl Fn(&str),
    ) -> Result<InstallStatus> {
        if is_app_installed(id, installed_app_display_names) {
            return Ok(InstallStatus::AlreadyInstalled);
        } else {
            set_process_message("Downloading...");
            let exe_path = download_file(source).await?;

            set_process_message("Installing...");
            let output = run_command_on_windows(&exe_path.to_string_lossy())?;
            if !output.status.success() {
                return Err(anyhow::anyhow!(
                    "Installation failed with output: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }

            if let Some(post_install) = post_install {
                set_process_message("Running post-install script...");
                run_command_on_windows(post_install)?;
            }

            Ok(InstallStatus::Installed)
        }
    }

    fn is_app_installed(display_name: &str, installed_app_display_names: &HashSet<String>) -> bool {
        installed_app_display_names.contains(display_name)
    }
}
