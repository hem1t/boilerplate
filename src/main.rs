use clap::{App, Arg, ArgMatches};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    let app = read_args();
    let root_path = PathBuf::from(app.value_of("Output").unwrap_or("./"));
    let config = app.value_of("config").unwrap_or("");
    let code_file_name = app.value_of("Input").unwrap_or_else(|| panic!(""));

    // just checking
    print!(
        "root_path = {:?}, config = {}, code_file_name = {}",
        root_path, config, code_file_name
    );
}

fn read_args<'a>() -> ArgMatches<'a> {
    let app = App::new("Boilerplate")
        .version("0.1.0")
        .author("hem1t <sunny10fb@gmail.com>")
        .about("")
        // command.file file to read code from
        .arg(
            Arg::with_name("Input")
                .index(1)
                .required(true)
                .help("choose a Boilerplate, can define new ones in ~/.config/boilerplate folder."),
        )
        // command.output
        .arg(
            Arg::with_name("Output")
                .short("o")
                .long("output")
                .help("path to create the stuffs.")
                .takes_value(true),
        )
        // command.config
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("config name to be aplied.")
                .takes_value(true),
        )
        // command.verbosity
        // .arg(Arg::with_name("verbosity")
        //      .short("v")
        //      .help("will print more info.")
        // )
        // command.debug
        .arg(
            Arg::with_name("debug")
                .short("d")
                .help("logs the details and stores temporarily in ~/.config/boilerplate/.log"),
        )
        .get_matches();
    app
}
