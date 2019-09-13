use pulldown_cmark::CowStr;
use std::fs::File;
use std::io::Read;

use crate::image_embedder;
use crate::image_encoder;
use crate::image_linker;
use crate::yaml_header;
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use serde_yaml::Value;

extern crate pulldown_cmark_to_cmark;
use pulldown_cmark_to_cmark::fmt::cmark;

use crate::download::DownloadMod;
use crate::embed::{RessourceType};

#[derive(Clone)]
pub struct Document<'a> {
    pub header: Option<(serde_yaml::Value)>,
    pub nodes: Vec<Event<'a>>,
    pub download_mod: DownloadMod,
    pub image_dir: &'a str,
}

impl<'a> Document<'a> {
    pub fn new(input: &'a str, download_mod: DownloadMod, image_dir: &'a str) -> Document<'a> {
        let (header, body) = yaml_header::parse_yaml_header(input);
        let nodes = Parser::new_ext(body, Options::empty())
            .map(|event| event)
            .collect::<Vec<Event>>();

        Document {
            header: header,
            nodes: nodes,
            download_mod: download_mod,
            image_dir: image_dir,
        }
    }

    pub fn embed(&mut self, files: Option<Vec<&str>>) {
        if let Some(files) = files {
            for file in files {
                self.embed_file(file)
            }
        };

        self.nodes = self
            .nodes
            .clone()
            .into_iter()
            .map(|mut node| {
                self.embed_images(&mut node);
                node
            })
            .collect::<Vec<Event>>();
    }

    pub fn link(&mut self, files: Option<Vec<&str>>) {
        self.nodes = self
            .nodes
            .clone()
            .into_iter()
            .map(|mut node| {
                self.link_images(&mut node);
                node
            })
            .collect::<Vec<Event>>();
    }

    pub fn link_images(&self, event: &mut Event) {
        match event {
            Event::Start(Tag::Image(_, image_link, _))
            | Event::End(Tag::Image(_, image_link, _)) => {
                *image_link = CowStr::from(image_linker::link_image(
                    image_link.as_ref(),
                    self.download_mod,
                    self.image_dir,
                ));
            }
            _ => (),
        };
    }

    pub fn embed_images(&self, event: &mut Event) {
        match event {
            Event::Start(Tag::Image(_, image_link, _))
            | Event::End(Tag::Image(_, image_link, _)) => {
                //link first
                *image_link = CowStr::from(image_linker::link_image(
                    image_link.as_ref(),
                    self.download_mod,
                    self.image_dir,
                ));

                //then embed
                *image_link = CowStr::from(image_embedder::embed_image(image_link.as_ref()));
            }
            _ => (),
        }
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::new();

        if let Some(header) = &self.header {
            buf.push_str(&serde_yaml::to_string(&header).unwrap());
            buf.push_str("\n---\n");
        };

        cmark(self.nodes.iter(), &mut buf, None).unwrap();

        buf
    }

    fn embed_file(&mut self, path_to_file: &str) {
        //Determine the file type
        let ressource_type = RessourceType::match_ressource_type(path_to_file);

        match ressource_type {
            //Yaml file is included in the document header
            RessourceType::Yaml(value) => {
                let yml_values: serde_yaml::Value = serde_yaml::from_str(&value).unwrap();

                if let Some(header) = &mut self.header {
                    let mapping = header.as_mapping_mut().unwrap();
                    for value in yml_values.as_mapping().unwrap().iter() {
                        mapping.insert(value.0.to_owned(), value.1.to_owned());
                    }
                } else {
                    self.header = Some(yml_values);
                }
            }
            //Contents of other files are placed inside the document body
            _ => self
                .nodes
                .push(Event::Text(CowStr::from(ressource_type.format()))),
        }
    }
}

#[cfg(test)]
mod test_md_file {
    use super::*;

    #[test]
    fn should_format() {
        let markdown_input = "# Title\n\nLorem ispum";
        let document = Document::new(markdown_input, DownloadMod::Keep, "./");
        assert_eq!(document.to_string(), markdown_input)
    }

    #[test]
    fn should_print_yaml_header() {
        let markdown_input = "---\nheader_entry: entry\n---\n# Title\n\nLorem ispum";
        let document = Document::new(markdown_input, DownloadMod::Keep, "./");
        assert_eq!(document.to_string(), markdown_input)
    }

    #[test]
    fn should_add_yaml_header() {
        let markdown_input = "# Title\n\nLorem ispum";
        let expected_output = "---\nnew_entry_1: entry_value_1\nnew_entries:\n  - entry_1\n  - entry_2\n---\n# Title\n\nLorem ispum";

        let mut document = Document::new(markdown_input, DownloadMod::Keep, "./");
        document.embed(Some(vec!["./test/files/fake_header.yaml"]));

        assert_eq!(document.to_string(), expected_output)
    }

    #[test]
    fn should_merge_yaml_header() {
        let markdown_input = "---\nheader_entry: entry\n---\n# Title\n\nLorem ispum";
        let expected_output = "---\nheader_entry: entry\nnew_entry_1: entry_value_1\nnew_entries:\n  - entry_1\n  - entry_2\n---\n# Title\n\nLorem ispum";

        let mut document = Document::new(markdown_input, DownloadMod::Keep, "./");
        document.embed(Some(vec!["./test/files/fake_header.yaml"]));

        assert_eq!(document.to_string(), expected_output)
    }

    #[test]
    fn should_add_css() {
        let markdown_input = "# Title\n\nLorem ispum";
        let expected_output =
            "# Title\n\nLorem ispum\n\n<style>\np {\n  background-color: red;\n}</style>\n";

        let mut document = Document::new(markdown_input, DownloadMod::Keep, "./");
        document.embed(Some(vec!["./test/files/fake_css.css"]));

        assert_eq!(document.to_string(), expected_output)
    }

    #[test]
    fn should_add_js() {
        let markdown_input = "# Title\n\nLorem ispum";
        let expected_output =
            "# Title\n\nLorem ispum\n\n<script>\nconst a = \"value\";\n</script>\n";

        let mut document = Document::new(markdown_input, DownloadMod::Keep, "./");
        document.embed(Some(vec!["./test/files/fake_script.js"]));

        assert_eq!(document.to_string(), expected_output)
    }

    #[test]
    fn should_embed_image() {
        let markdown_input = "# Title\n\nLorem ispum\n\n![my_image](./test/files/image.png)";
        let expected_output = "# Title\n\nLorem ispum\n\n![my_image](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAIAAACQd1PeAAABhGlDQ1BJQ0MgcHJvZmlsZQAAKJF9kT1Iw0AcxV9bpSotDnYQdchQnSxIFXHUKhShQqgVWnUwufQLmjQkKS6OgmvBwY/FqoOLs64OroIg+AHi4uqk6CIl/i8ptIjx4Lgf7+497t4B/kaFqWbXBKBqlpFOJoRsblUIvqIXYYQwjLjETH1OFFPwHF/38PH1LsazvM/9OcJK3mSATyCeZbphEW8QT29aOud94ggrSQrxOfG4QRckfuS67PIb56LDfp4ZMTLpeeIIsVDsYLmDWclQiaeIo4qqUb4/67LCeYuzWqmx1j35C0N5bWWZ6zRHkMQiliBCgIwayqjAQoxWjRQTadpPePiHHL9ILplcZTByLKAKFZLjB/+D392ahcm4mxRKAN0vtv0xCgR3gWbdtr+Pbbt5AgSegSut7a82gJlP0uttLXoE9G8DF9dtTd4DLneAwSddMiRHCtD0FwrA+xl9Uw4YuAX61tzeWvs4fQAy1FXqBjg4BMaKlL3u8e6ezt7+PdPq7wd8x3Kr3zREKAAAAAlwSFlzAAAuIwAALiMBeKU/dgAAAAd0SU1FB+MJDQwgG7WFbQoAAAAZdEVYdENvbW1lbnQAQ3JlYXRlZCB3aXRoIEdJTVBXgQ4XAAAADElEQVQI12NgYGAAAAAEAAEnNCcKAAAAAElFTkSuQmCC)";

        let mut document = Document::new(markdown_input, DownloadMod::Keep, "./");
        document.embed(None);

        assert_eq!(document.to_string(), expected_output)
    }

    #[test]
    fn should_link_image() {
        let markdown_input = "# Title\n\nLorem ispum\n\n![my_image](http://example.com/image.jpg)";
        let expected_output = "# Title\n\nLorem ispum\n\n![my_image](./test/files/image.jpg)";

        let mut document = Document::new(markdown_input, DownloadMod::Keep, "./test/files/");
        document.link(None);

        assert_eq!(document.to_string(), expected_output)
    }
}
