mod parser;

use clap::Parser as ClapParser;
use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::parser::LineParser;

#[derive(ClapParser, Debug)]
#[command(author = "hem1t <sunny10fb@gmail.com>",
          version=  "0.1.0",
          about = "choose a Boilerplate, can define new ones in ~/.config/boilerplate folder.",
          long_about = None)]
struct Options {
    /// choose a Boilerplate, can define new ones in ~/.config/boilerplate folder.
    #[arg(value_name = "FILE")]
    input: String,
    /// path to create the stuffs.
    #[arg(short, long)]
    output: PathBuf,
    /// config name to be aplied.
    #[arg(short, long)]
    config: String,
    /// will print more info.
    #[arg(short, long, default_value_t = false)]
    verbosity: bool,
    /// save the boilerplate.
    #[arg(short, long, default_value_t = false)]
    save: bool,
    /// vars
    #[arg(long)]
    vars: String,
}

fn main() {
    let app = Options::parse();
    let _root_path = app.output;
    let _config = app.config;
    let _code_file_name = app.input;

    let vars: HashMap<&str, Rc<String>> = get_vars(&app.vars);

    println!("{:?}", _code_file_name);

    let file = open_file(&_code_file_name);
    println!("{:?}", file);

    println!("{:?}", vars);
    let lines = LineParser::new().parse(file).ok();
    println!("{:?}", lines);
    Validator::validate()
}

fn get_vars(var_string: &String) -> HashMap<&str, Rc<String>> {
    var_string
        .split(':')
        .map(|var| {
            let mut t = var.split('=');
            let k: &str = t.next().unwrap().trim().into();
            let v: Rc<String> = Rc::new(t.next().unwrap().trim().into());
            (k, v)
        })
        .collect::<HashMap<&str, Rc<String>>>()
}

fn open_file(file_name: &String) -> BufReader<File> {
    // does exists in saved boilerplate
    if let Ok(path) = Path::new(&format!("~/.config/boilerplate/{}", file_name)).canonicalize() {
        if path.is_file() {
            return BufReader::new(File::open(path).unwrap());
        }
    }
    // otherwise
    if let Ok(path) = Path::new(file_name).canonicalize() {
        if path.is_file() {
            return BufReader::new(File::open(path).unwrap());
        }
    }
    // Err
    eprintln!("Error:\n    {file_name} may not exist or have non readable permissions.");
    std::process::exit(1);
}
