use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::image_encoder;

pub fn embed_image(image_path: &str) -> String {
    let mut buffer = Vec::new();
    let file = File::open(&image_path);

    match file {
        Ok(mut f) => {
            f.read_to_end(&mut buffer).unwrap();
            image_encoder::to_base64(&buffer).unwrap_or(image_path.to_string())
        }
        Err(_) => image_path.to_string(),
    }
}
