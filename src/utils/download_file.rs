use anyhow::Result;
use std::env;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn download_file(url: &str) -> Result<PathBuf> {
    let response = reqwest::get(url).await?;
    let file_path = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .map(|name| {
            let mut path = env::temp_dir();
            path.push(name);
            path
        });
    let file_path = file_path.expect("Failed to create file");
    let mut out = File::create(&file_path).await?;
    let bytes = response.bytes().await?;
    out.write_all(&bytes).await?;
    Ok(file_path)
}
