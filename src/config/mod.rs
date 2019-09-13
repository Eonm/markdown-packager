use crate::markdown_document::Document;
use crate::download::DownloadMod;

pub struct Config<'a> {
    download_mod: DownloadMod,
    image_dir: &'a str,
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        Config {
            download_mod: DownloadMod::Keep,
            image_dir: "./",
        }
    }

    pub fn set_image_dir(mut self, image_dir: &'a str) -> Self {
        self.image_dir = image_dir;
        self
    }

    pub fn set_download_mod(mut self, download_mod: DownloadMod) -> Self {
        self.download_mod = download_mod;
        self
    }

    pub fn to_document(&self, input: &'a str) -> Document<'a> {
        Document::new(input, self.download_mod, self.image_dir)
    }
}
