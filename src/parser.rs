#![allow(unused)]
use std::{error::Error, fmt, fs::File, io::BufRead, io::BufReader};

#[derive(Debug)]
pub enum ParserError {
    // line number
    LineError(usize),
    ParamError,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let ParserError::LineError(a) = self {
            write!(f, "{:?}", a)
        } else {
            write!(f, "Param")
        }
    }
}

impl Error for ParserError {}

struct Line {
    line: usize,
    leading_tab_count: usize,
    command: SingleLineCode,
}

#[derive(Debug)]
pub enum SingleLineCode {
    Config(Vec<String>),
    DirOpen(String, String),
    FileOpen(String, String),
    Parah(String),
    Text(String),
    Close,
    Empty,
}

// impl Line {
//     pub verify(self) -> Result<(), String> {
//     }
// }

pub struct LineParser {
    line_count: usize,
}

impl LineParser {
    pub fn new() -> Self {
        Self { line_count: 0 }
    }

    pub fn parse(&mut self, buffer: BufReader<File>) -> Result<Vec<SingleLineCode>, ParserError> {
        let mut all = vec![];
        let mut put = |all: &mut Vec<SingleLineCode>, a: SingleLineCode| all.push(a);
        buffer
            .lines()
            .try_for_each(|line| -> Result<(), ParserError> {
                let line = line.unwrap();
                let tab = count_tabs(&line);
                let line = line.trim();
                match line.get(0..1) {
                    Some("!") => put(&mut all, SingleLineCode::Close),
                    Some("$") => put(&mut all, self.get_dir(line)?),
                    Some("#") => put(&mut all, self.get_file(line)?),
                    Some("@") => put(&mut all, self.get_parah(line)?),
                    Some("%") => put(&mut all, self.get_config(line)?),
                    Some(_) => put(&mut all, SingleLineCode::Text(line.into())),
                    None => put(&mut all, SingleLineCode::Empty),
                }
                self.line_count += 1;
                Ok(())
            })?;
        Ok(all)
    }

    fn get_dir(&mut self, line: &str) -> Result<SingleLineCode, ParserError> {
        let params = self.parse_params(line);
        if params.len() == 2 {
            Ok(SingleLineCode::DirOpen(
                params[0].to_owned(),
                params[1].to_owned(),
            ))
        } else {
            Err(ParserError::ParamError)
        }
    }

    fn get_file(&mut self, line: &str) -> Result<SingleLineCode, ParserError> {
        let params = self.parse_params(line);
        if params.len() == 2 {
            Ok(SingleLineCode::FileOpen(
                params[0].to_owned(),
                params[1].to_owned(),
            ))
        } else {
            Err(ParserError::ParamError)
        }
    }

    fn get_parah(&mut self, line: &str) -> Result<SingleLineCode, ParserError> {
        let params = self.parse_params(line);
        if params.len() == 1 {
            // validate_ident(params[0].to_owned())?;
            Ok(SingleLineCode::Parah(params[0].to_owned()))
        } else {
            Err(ParserError::ParamError)
        }
    }

    fn get_config(&mut self, line: &str) -> Result<SingleLineCode, ParserError> {
        let params = self.parse_params(line);
        if params.len() > 1 {
            Ok(SingleLineCode::Config(params))
        } else {
            Err(ParserError::ParamError)
        }
    }

    fn parse_params(&mut self, line: &str) -> Vec<String> {
        line.trim()
            .split(':')
            .map(|string| string.to_string())
            .collect::<Vec<String>>()
    }
}

fn count_tabs(line: &String) -> usize {
    line.chars()
        .take_while(|ch| *ch == '\t')
        .fold(0, |acc, _| acc + 1)
}
