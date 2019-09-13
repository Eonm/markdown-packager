extern crate clap;
extern crate colored;

mod cli;
mod download;
mod embed;
mod errors;
mod image_embedder;
mod image_encoder;
mod image_linker;
mod logger;
mod markdown_document;
mod yaml_header;
mod config;
use config::Config;

use download::DownloadMod;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

use crate::logger::LOGGER;
use log::{info, LevelFilter};

fn main() {
    let matches = cli::build_cli().get_matches();

    //Setting up logger
    if matches.is_present("log") {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
    }

    let mut config = Config::new();

    //parsing download mod
    if let Some(download_mod) = matches.value_of("download_mod") {
        match download_mod {
            "keep" => config = config.set_download_mod(DownloadMod::Keep),
            "erase" => config = config.set_download_mod(DownloadMod::Erase),
            _ => config = config.set_download_mod(DownloadMod::Keep),
        }
    } else {
        config = config.set_download_mod(DownloadMod::Keep);
    };

    if let Some(image_dir) = matches.value_of("image_dir") {
        config = config.set_image_dir(image_dir);
    }

    //Getting content from input file or from stdin
    let mut contents = String::new();

    if let Some(input_file) = matches.value_of("input") {
        let mut f = File::open(input_file).unwrap();
        f.read_to_string(&mut contents);
    } else {
        eprintln!("Type your text (press C^D)");
        io::stdin().read_to_string(&mut contents);
        eprintln!("\n");
    };

    let mut md_document = config.to_document(&contents);

    if let Some(sub_matches) = matches.subcommand_matches("pack") {
        if let Some(files) = sub_matches.values_of("files") {
            md_document.embed(Some(files.collect::<Vec<&str>>()));
        } else {
            md_document.embed(None);
        }
    }

    if let Some(sub_matches) = matches.subcommand_matches("link") {
        if let Some(files) = sub_matches.values_of("files") {
            md_document.link(Some(files.collect::<Vec<&str>>()));
        } else {
            md_document.link(None);
        }
    }

    if let Some(output_file) = matches.value_of("output") {
        let mut file = File::create(output_file).unwrap();
        file.write_all(md_document.to_string().as_bytes());
    } else {
        print!("{}", md_document.to_string());
    };
}
