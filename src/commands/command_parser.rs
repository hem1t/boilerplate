use std::{
    env,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};

// A code parse through commands
// If commands are available as files.

pub fn fetch_and_parse(file_name: &str) -> Vec<Vec<String>> {
    // fetching from file.
    let content = open_file(file_name)
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    // to commands
    // let mut commands = Vec::new();
    // // making commands out of cache.
    // for mut line in content {
    //     if line.len() > 1 {
    //         commands.push();
    //     }
    // }
    content.clone()
}

fn open_file(file_name: &str) -> String {
    let mut path = env::var("BOILERPLATES").unwrap_or(String::from("~/.local/boilerplates"));
    path.push_str("/");
    path.push_str(file_name);

    let path = Path::new(&path);
    let mut content = String::new();

    let mut file = File::open(&path).expect("Failed to open file from cache");
    // error
    file.read_to_string(&mut content)
        .expect("Failed to read file from cache");
    // handle error

    content
}
