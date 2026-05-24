use std::fs::File;

use indicatif::ProgressBar;

pub fn download() -> Result<(), std::io::Result<()>> {
    let mut response = reqwest::blocking::get("https://example.com/large_file.zip");
    let total_size = response.content_length().unwrap_or(0);
    let pb = ProgressBar::new(total_size);
    // ... (set style)

    let mut dest = File::create("downloaded_file.zip")?;
    while let Some(chunk) = response.chunk()? {
        dest.write_all(&chunk)?;
        pb.inc(chunk.len() as u64);
    }
    pb.finish_with_message("Download complete!");

    Ok(())
}
