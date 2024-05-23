use anyhow::Result;
use std::path::Path;

use crate::run_command_on_unix;

pub fn extract_zip<T: AsRef<Path>>(zip_path: T, extract_path: &str) -> Result<()> {
    run_command_on_unix(&format!("unzip -o {} -d {}", zip_path.as_ref().display(), extract_path))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::download_file;

    pub use super::*;

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_extract_zip() -> Result<()> {
        let extract_path = "tmp/extract_zip_test";
        fs::create_dir_all(extract_path)?;

        let zip_path = download_file("https://vscode.download.prss.microsoft.com/dbazure/download/insider/5f78b58b57b7cf84d28d801fed6bb4a48f908601/VSCode-darwin-arm64.zip").await?;
        extract_zip(&zip_path, extract_path)?;

        assert!(Path::new(extract_path).exists());
        let entries = fs::read_dir(extract_path)?.collect::<Result<Vec<_>, _>>()?;
        let entry = &entries[0];
        assert_eq!(entry.file_name(), "Visual Studio Code - Insiders.app");
        // clean download files
        fs::remove_dir_all(extract_path)?;
        fs::remove_file(&zip_path)?;
        Ok(())
    }
}
