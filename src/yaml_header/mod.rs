use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use serde_yaml::Sequence;
use std::collections::HashMap;

pub fn get_yaml_header_position<'a>(input_str: &'a str) -> Option<(usize, usize)> {
    if !input_str.starts_with("---") {
        return None;
    }

    let start_pattern = "---\n";
    let end_pattern = "\n---\n";

    match input_str.find(start_pattern) {
        Some(start) => match input_str[start + start_pattern.len()..].find(end_pattern) {
            Some(end) => {
                return Some((start, end + start_pattern.len() + end_pattern.len()));
            }
            None => return None,
        },
        None => return None,
    }
}

pub fn parse_yaml_header<'a>(input_str: &'a str) -> (Option<serde_yaml::Value>, &str) {
    match get_yaml_header_position(input_str) {
        Some((start, end)) => {
            let header = serde_yaml::from_str(&input_str[start..end - 4]).unwrap();
            let body = &input_str[end..];
            (Some(header), body)
        }
        None => (None, input_str),
    }
}

#[cfg(test)]
mod test_image_type {
    use super::*;

    #[test]
    fn should_parse_yaml_header() {
        let markdown_input = "---\nvalue: val\n---\n\n# Title";
        let expected_header : serde_yaml::Value =  serde_yaml::from_str("---\nvalue: val").unwrap();
        assert_eq!(parse_yaml_header(markdown_input), (Some(expected_header), "\n# Title"));

        let markdown_input_no_header = "# Title";
        assert_eq!(parse_yaml_header(markdown_input_no_header), (None, "# Title"));
    }
}
