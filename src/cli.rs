use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("markdown_packager")
        .version("0.0.1")
        .author("Eonm <eon.mathis@gmail.com>")
        .about("Link and embed files inside your markdown documents")
        .arg(
            Arg::with_name("log")
                .short("l")
                .long("log")
                .global(true)
                .help("Display log")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .global(true)
                .value_name("input_file")
                .help("Input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("output_file")
                .help("Output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("download_mod")
                .possible_values(&["keep", "erase"])
                .long("download-mod")
                .global(true)
                .value_name("download_mod")
                .default_value("keep")
                .help("Keep or erase image files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("image_dir")
                .long("image-dir")
                .global(true)
                .help("Image dir")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("pack").about("Pack files").arg(
                Arg::with_name("files")
                    .min_values(0)
                    .help("Embed md, js, html, css, latex, and yaml files. Images are embedded as base64 links"),
            ),
        )
        .subcommand(
            SubCommand::with_name("link").about("Link image files")
        )
}
