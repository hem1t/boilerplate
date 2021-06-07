mod command_parser;
use command_parser::fetch_and_parse;
use std::path::{Path, PathBuf};

// This struct takes command name and options
// and then runs them

struct Evaluater {
    path: PathBuf,
    commands: Vec<Vec<String>>,
    // file object
    file: Option<std::fs::File>,
}
impl Evaluater {
    pub fn new(root: PathBuf, commands: Vec<Vec<String>>) -> Self {
        Evaluater {
            path: root,
            commands,
            file: None,
        }
    }

    pub fn evaluate(&self) -> Result<(), String> {
        for command in &self.commands {
            match command[0].as_str() {
                // Format -> $ "token_name" "path"
                // todo
                // change root to path
                // and create the path
                "$" => {
                    // change root to path (command[2])
                }
                "$!" => {}
                "#" => {}
                "#!" => {}
                "@" => {}
                "@!" => {}
                _ => (),
            }
        }
        Ok(())
    }
}

pub fn evaluate_from_cache(file: &str, root: PathBuf) -> Result<(), String> {
    // fetch commands
    let commands = fetch_and_parse(file);
    // TODO change commands by config
    let evaluater = Evaluater::new(root, commands);
    evaluater.evaluate()?;
    Ok(())
}
