use std::process::Command;

use toolkit::download_file;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let exe_path = download_file("https://vscode.download.prss.microsoft.com/dbazure/download/insider/81c89c4d00663e1718871bab2f9bf2064a060b63/VSCodeUserSetup-x64-1.90.0-insider.exe").await?;
    let output = Command::new(exe_path)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Installation failed with output:\n{}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Installation succeeded");
    }
    Ok(())
}
