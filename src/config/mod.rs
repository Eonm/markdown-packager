use crate::download::DownloadMod;
use crate::markdown_document::Document;

#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod test_config {
    use super::*;
    use pulldown_cmark::{CowStr, Event, Tag};

    #[test]
    fn should_set_image_dir() {
        let mut config = Config::new();
        config = config.set_image_dir("./images/");

        let expected_config = Config {
            download_mod: DownloadMod::Keep,
            image_dir: "./images/",
        };

        assert_eq!(config, expected_config)
    }

    #[test]
    fn should_set_download_mod() {
        let mut config = Config::new();
        config = config.set_download_mod(DownloadMod::Erase);

        let expected_config = Config {
            download_mod: DownloadMod::Erase,
            image_dir: "./",
        };

        assert_eq!(config, expected_config)
    }

    #[test]
    fn should_return_document() {
        let mut config = Config::new();
        let document = config.to_document("test");
        let expected_document = Document {
            header: None,
            nodes: vec![
                Event::Start(Tag::Paragraph),
                Event::Text(CowStr::Borrowed("test")),
                Event::End(Tag::Paragraph),
            ],
            download_mod: DownloadMod::Keep,
            image_dir: "./",
        };

        assert_eq!(document, expected_document)
    }
}
