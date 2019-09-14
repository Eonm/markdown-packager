use std::error;
use std::fs::File;
use std::io::Read;
use std::string::String;

extern crate rustc_serialize;

use crate::errors;

use log::info;
use rustc_serialize::base64::{ToBase64, MIME};
use rustc_serialize::hex::ToHex;

fn get_file_type(hex: &str) -> Result<&str, errors::CustomError> {
    if hex.starts_with("ffd8ff") {
        return Ok("jpeg");
    } else if hex.starts_with("89504e47") {
        return Ok("png");
    } else if hex.starts_with("47494638") {
        return Ok("gif");
    } else if hex.contains("3c737667") && hex.starts_with("3c3f786d6c") || hex.starts_with("3c737667") {
        return Ok("svg+xml");
    }

    Err(errors::CustomError::UknownImageType)
}

pub fn to_base64(image_data: &Vec<u8>) -> Result<String, Box<error::Error>> {
    let base64 = image_data.to_base64(MIME);
    let hex = image_data.to_hex();
    Ok(format!(
        "data:image/{};base64,{}",
        get_file_type(&hex)?,
        base64.replace("\r\n", "")
    ))
}

#[cfg(test)]
mod test_image_type {
    use super::*;

    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    #[test]
    fn should_get_image_type() {
        let open_file = |input: &str| -> String {
            let mut buffer = Vec::new();
            let mut f = File::open(input).unwrap();
            f.read_to_end(&mut buffer).unwrap();
            get_file_type(&buffer.to_hex()).unwrap().to_string()
        };

        let jpeg_file = open_file("./test/files/image.jpeg");
        let jpg_file = open_file("./test/files/image.jpg");
        let gif_file = open_file("./test/files/image.gif");
        let png_file = open_file("./test/files/image.png");
        let svg_file = open_file("./test/files/image.svg");

        assert_eq!(jpeg_file, "jpeg");
        assert_eq!(jpg_file, "jpeg");
        assert_eq!(gif_file, "gif");
        assert_eq!(png_file, "png");
        assert_eq!(svg_file, "svg+xml");
    }
}
