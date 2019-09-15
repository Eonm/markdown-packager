use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::yaml_header;
use pulldown_cmark::CowStr;
use pulldown_cmark::Event;

#[derive(PartialEq, Debug)]
pub enum RessourceType {
    Javascript(String),
    Css(String),
    Html(String),
    Latex(String),
    Text(String),
    Md(String),
    Yaml(String),
    Unknown(String),
}

impl RessourceType {
    pub fn match_ressource_type(input: &str) -> RessourceType {
        let file_extension = Path::new(input).extension().and_then(OsStr::to_str);

        let file_content = || -> String {
            let mut buffer = Vec::new();
            let mut file = File::open(input).unwrap();
            file.read_to_end(&mut buffer).unwrap();

            String::from_utf8(buffer).unwrap()
        };

        if let Some(file_extension) = file_extension {
            match file_extension {
                "js" => RessourceType::Javascript(file_content()),
                "css" => RessourceType::Css(file_content()),
                "html" => RessourceType::Html(file_content()),
                "tex" => RessourceType::Latex(file_content()),
                "txt" => RessourceType::Text(file_content()),
                "md" => RessourceType::Md(file_content()),
                "yaml" | "yml" => RessourceType::Yaml(file_content()),
                _ => RessourceType::Unknown(file_content()),
            }
        } else {
            RessourceType::Unknown(file_content())
        }
    }

    pub fn format(self) -> String {
        match self {
            RessourceType::Md(content) => {
                let (_, body) = yaml_header::parse_yaml_header(&content);
                body.to_owned()
            }
            RessourceType::Javascript(content) => {
                format!("<script>\n{}\n</script>\n", content.trim())
            }
            RessourceType::Css(content) => format!("<style>\n{}</style>\n", content.trim()),
            RessourceType::Html(content) => content,
            RessourceType::Latex(content) => content,
            RessourceType::Text(content) => content,
            RessourceType::Md(content) => content,
            RessourceType::Yaml(content) => content,
            RessourceType::Unknown(content) => content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_match_ressource_type() {
        let file_content = if cfg!(target_os = "windows") {
            " \r\n".to_string()
        } else {
            " \n".to_string()
        };

        assert_eq!(
            RessourceType::match_ressource_type("./test/files/file.js"),
            RessourceType::Javascript(file_content.clone())
        );
        assert_eq!(
            RessourceType::match_ressource_type("./test/files/file.md"),
            RessourceType::Md(file_content.clone())
        );
        assert_eq!(
            RessourceType::match_ressource_type("./test/files/file.html"),
            RessourceType::Html(file_content.clone())
        );
        assert_eq!(
            RessourceType::match_ressource_type("./test/files/file.tex"),
            RessourceType::Latex(file_content.clone())
        );
        assert_eq!(
            RessourceType::match_ressource_type("./test/files/file.yaml"),
            RessourceType::Yaml(file_content.clone())
        );
        assert_eq!(
            RessourceType::match_ressource_type("./test/files/file.yml"),
            RessourceType::Yaml(file_content.clone())
        );
        assert_eq!(
            RessourceType::match_ressource_type("./test/files/file.txt"),
            RessourceType::Text(file_content.clone())
        );
        assert_eq!(
            RessourceType::match_ressource_type("./test/files/file.css"),
            RessourceType::Css(file_content.clone())
        );
        assert_eq!(
            RessourceType::match_ressource_type("./test/files/file.xyz"),
            RessourceType::Unknown(file_content.clone())
        );
    }
}
