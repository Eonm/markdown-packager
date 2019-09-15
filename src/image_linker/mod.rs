use std::path::PathBuf;
use std::str;

use crate::colored::Colorize;
use crate::download::{download, get_download_dest_path, DownloadMod};

use log::info;
use url::Url;

pub fn link_image(input: &str, image_erase: DownloadMod, image_dir: &str) -> String {
    //Only link remote images
    let input = match Url::parse(&input) {
        Ok(e) => match e.scheme() {
            "https" | "http" => input.to_string(),
            _ => return input.to_string(),
        },
        Err(_) => return input.to_string(),
    };

    let image_path = match image_erase {
        DownloadMod::Keep => {
            info!("checking : {}", input);
            match file_already_exists(&input, image_dir) {
                Some(file) => {
                    info!(" {}\n", "✓".green());
                    info!("File already exists : {}\n", file);
                    Ok(file)
                }
                None => {
                    info!(" {}\n", "✗".red());
                    download(&input, image_dir)
                }
            }
        }
        DownloadMod::Erase => download(&input, image_dir),
    };

    match image_path {
        Ok(path) => path,
        Err(_) => input.to_string(),
    }
}

fn file_already_exists(url: &str, dest_path: &str) -> Option<String> {
    match Url::parse(url).unwrap().path_segments() {
        //by checking the last segment of url
        Some(segments) => {
            let mut temp_dest_path = PathBuf::from(dest_path);
            temp_dest_path.push(segments.last().unwrap());

            if temp_dest_path.is_file() {
                return Some(temp_dest_path.into_os_string().into_string().unwrap());
            }
        }
        None => (),
    }

    //by checking remote file name
    let remote_temp_dest_path: PathBuf = match get_download_dest_path(url) {
        Ok(path) => PathBuf::from(dest_path).join(path),
        Err(_) => return None,
    };

    if remote_temp_dest_path.is_file() {
        Some(
            remote_temp_dest_path
                .into_os_string()
                .into_string()
                .unwrap(),
        )
    } else {
        None
    }
}

#[cfg(test)]
mod linker_test {
    use super::*;

    #[test]
    fn should_only_process_url() {
        let input = "./image.jpeg";
        assert_eq!(link_image(input, DownloadMod::Keep, "./"), input);

        let remote_image_file_url =
            "https://raw.githubusercontent.com/Eonm/markdown-packager/master/test/files/image.gif";
        assert_eq!(
            link_image(remote_image_file_url, DownloadMod::Keep, "./test/files/"),
            "./test/files/image.gif"
        );
    }

    #[test]
    fn file_should_already_exists() {
        let result = file_already_exists("https://example.com/image.jpeg", "./test/files/");
        assert_eq!(result, Some("./test/files/image.jpeg".to_string()));
    }

    #[test]
    fn file_should_not_already_exists() {
        let result = file_already_exists(
            "https://upload.wikimedia.org/wikipedia/commons/4/48/Markdown-mark.svg",
            "./test/files/",
        );
        assert_eq!(result, None);
    }
}
