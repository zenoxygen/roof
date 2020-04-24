// Copyright (c) 2020 zenoxygen
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use anyhow::anyhow;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header;
use tokio::{fs, io::AsyncWriteExt};
use url::Url;

use std::path::Path;

/// Download file.
#[tokio::main]
pub async fn download_file(url: &str) -> Result<(), anyhow::Error> {
    // Parse URL
    let url = match Url::parse(url) {
        Ok(url) => url,
        Err(_) => return Err(anyhow!("could not parse URL provided")),
    };

    // Fetch data
    fetch_data(url).await
}

/// Fetch data.
async fn fetch_data(url: Url) -> Result<(), anyhow::Error> {
    // Create new path from URL
    let path = Path::new(
        url.path_segments()
            .and_then(std::iter::Iterator::last)
            .unwrap(),
    );

    // Check if path already exists
    if path.exists() {
        return Err(anyhow!("path '{}' already exists", path.to_str().unwrap()));
    }

    // Request URL
    let mut req = match reqwest::get(url.as_str()).await {
        Ok(req) => req,
        Err(_) => return Err(anyhow!("could not send request for URL")),
    };

    // Get file size
    let total_size = {
        if req.status().is_success() {
            req.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|len| len.to_str().ok())
                .and_then(|len| len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(anyhow!("could not download file ({})", req.status()));
        }
    };

    // Create new file from path
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .await?;

    // Display status
    println!("Downloading {}", path.to_str().unwrap());

    // Create progress bar
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {bytes}/{total_bytes} [{bar:40.cyan/blue}] {percent}%")
            .progress_chars("#>-"),
    );

    // Write into file and update progress bar
    while let Some(chunk) = req.chunk().await? {
        file.write_all(&chunk).await?;
        pb.inc(chunk.len() as u64);
    }

    Ok(())
}
