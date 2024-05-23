use std::io::{BufRead, BufReader};
use std::process::Output;
use std::{
    env,
    path::Path,
    process::{Command, Stdio},
};

pub fn run_command_pipe_on_unix(program: &str, message_callback: impl Fn(&str)) -> anyhow::Result<()> {
    let shell = get_default_shell();
    let unix_config_path = get_unix_shell_config_path(&shell);
    let mut command = if Path::new(&unix_config_path).exists() {
        format!("source {}", &unix_config_path)
    } else {
        "".to_string()
    };
    command = format!("{} && {}", command, program);

    let mut child = Command::new(&shell)
        .args(["-c", &command])
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(ref mut stdout) = child.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            message_callback(line?.as_str());
        }
    } else if let Some(ref mut stderr) = child.stderr {
        let reader = BufReader::new(stderr);
        let error = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");
        return Err(anyhow::anyhow!(error));
    }
    Ok(())
}

pub fn run_command_on_unix(program: &str) -> anyhow::Result<Output> {
    let shell = get_default_shell();
    let unix_config_path = get_unix_shell_config_path(&shell);
    let mut command = if Path::new(&unix_config_path).exists() {
        format!("source {}", &unix_config_path)
    } else {
        "".to_string()
    };
    command = format!("{} && {}", command, program);
    let output = Command::new(&shell)
        .args(["-c", &command])
        .output()
        .expect("failed to execute process");
    Ok(output)
}

pub fn is_cmd_exists<T: AsRef<str>>(program: T) -> anyhow::Result<bool> {
    if cfg!(windows) {
        is_windows_cmd_exists(program)
    } else {
        is_unix_cmd_exists(program)
    }
}

fn is_windows_cmd_exists<T: AsRef<str>>(program: T) -> anyhow::Result<bool> {
    let output = Command::new("where").arg(program.as_ref()).output()?;
    if output.status.success() {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn is_unix_cmd_exists<T: AsRef<str>>(program: T) -> anyhow::Result<bool> {
    let shell = get_default_shell();
    let unix_config_path = get_unix_shell_config_path(&shell);
    let mut command = if Path::new(&unix_config_path).exists() {
        format!("source {}", &unix_config_path)
    } else {
        "".to_string()
    };
    command = format!("{} && command -v {}", command, program.as_ref());

    let output = Command::new(&shell).args(["-c", &command]).output()?;

    if output.status.success() {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn get_default_shell() -> String {
    match env::var("SHELL") {
        Ok(shell) => shell,
        Err(_) => "/bin/sh".to_string(),
    }
}

fn get_unix_shell_config_path(shell: &str) -> String {
    let home = match home::home_dir() {
        Some(path) if !path.as_os_str().is_empty() => path.to_str().unwrap().to_string(),
        _ => panic!("Unable to get your home dir!"),
    };
    match shell {
        "/bin/zsh" => format!("{}/.zshrc", home).to_string(),
        "/bin/bash" => format!("{}/.bashrc", home).to_string(),
        "/bin/fish" => format!("{}/.config/fish/config.fish", home).to_string(),
        _ => format!("{}/.bashrc", home).to_string(),
    }
}
