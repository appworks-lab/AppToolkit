use anyhow::Result;
use reqwest::Client;
use std::{cmp::min, env, path::PathBuf, fs::File, io::Write};
use futures_util::StreamExt;

pub async fn download_file(url: &str, set_process_message: impl Fn(&str)) -> Result<PathBuf> {
    let client = Client::new();
    let response = client.get(url)
      .send()
      .await
      .map_err(|err| anyhow::anyhow!("Failed to GET from '{}'. Error: {}", url, err))?;
    let total_size = response
      .content_length()
      .ok_or(anyhow::anyhow!("Failed to get content length from '{}'", &url))?;
    let file_path = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .map(|name| {
            let mut path = env::temp_dir();
            path.push(name);
            path
        }).ok_or(anyhow::anyhow!("Failed to get file name from url '{}'", url))?;
    let mut file = File::create(&file_path)?;

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
      let chunk = item.map_err(|err| anyhow::anyhow!("Failed to get next item from stream. Error: {}", err))?;
      file.write_all(&chunk).map_err(|err| anyhow::anyhow!("Failed to write to file '{:?}'. Error: {}", &file_path, err))?;
      let new = min(downloaded + (chunk.len() as u64), total_size);
      downloaded = new;

      set_process_message(
        &format!("Downloaded: {:.1} MB / {:.1} MB ({:.1}%)", downloaded as f64 / 1024.0 / 1024.0, total_size as f64 / 1024.0 / 1024.0, 100_f64 * (downloaded as f64 / total_size as f64))
      );
    }

    set_process_message(&format!("Downloaded {} to {:?}", url, &file_path));

    Ok(file_path)
}
