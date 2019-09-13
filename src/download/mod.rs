use std::error;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::{Path, PathBuf};

extern crate reqwest;
extern crate tempdir;
extern crate uuid;

use crate::colored::Colorize;
use log::info;
use tempdir::TempDir;
use uuid::Uuid;

#[derive(Copy, Clone)]
pub enum DownloadMod {
    Erase,
    Keep,
}

pub fn download(target: &str, dest_path: &str) -> Result<String, Box<error::Error>> {
    info!("downloading {}", target);
    let remote_file_name = get_download_dest_path(target)?;

    let tmp_dir = TempDir::new("local_markdown")?;
    let mut response = reqwest::get(target)?.error_for_status()?;

    let temp_dest = tmp_dir.path().join(remote_file_name.clone());
    let mut temp_file = File::create(temp_dest.clone())?;
    copy(&mut response, &mut temp_file)?;

    fs::create_dir_all(dest_path)?;

    let final_path = if dest_path.ends_with("/") || dest_path.ends_with("\\") {
        Path::new(&dest_path).join(remote_file_name.clone())
    } else {
        Path::new(dest_path).to_path_buf()
    };

    fs::copy(temp_dest, final_path.clone())?;
    info!(" {}\n", "✓".green());

    Ok(final_path.into_os_string().into_string().unwrap())
}

pub fn get_download_dest_path(target: &str) -> Result<PathBuf, Box<error::Error>> {
    let client = reqwest::Client::new();
    let response = client.head(target).send()?.error_for_status()?;

    let name = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| {
                if name.is_empty() {
                    None
                } else {
                    Some(name.to_string())
                }
            })
            .unwrap_or(Uuid::new_v4().to_string());
        fname
    };

    Ok(Path::new(&name).to_path_buf())
}
