use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

use crate::utils::{ArithmeticCommand, Command};

pub struct Parser<R: Read> {
    pub reader: BufReader<R>
}

impl Parser<File> {
    pub fn new(in_name: &str) -> io::Result<Self>  {
        let in_file = File::open(in_name)?;
        let reader = BufReader::new(in_file);

        Ok(Parser {reader: reader})
    } 
}

impl<R: Read> Iterator for Parser<R> {
    type Item = Command;
    fn next(&mut self) -> Option<Command> {
        let mut line = String::new();
        loop {
            line.clear();
            match self.reader.read_line(&mut line) {
                Ok(0) => return None,
                Ok(_) => {
                    let line = get_trimmed_line(&line);
                    if line.is_empty() {
                        continue;
                    }
                    let mut split = line.split_whitespace(); 
                    let command = split.next().unwrap();
                    match command {
                        "add" => return Some(Command::Arithmetic(ArithmeticCommand::Add)),
                        "sub" => return Some(Command::Arithmetic(ArithmeticCommand::Sub)),
                        "neg" => return Some(Command::Arithmetic(ArithmeticCommand::Neg)),
                        "eq" => return Some(Command::Arithmetic(ArithmeticCommand::Eq)),
                        "gt" => return Some(Command::Arithmetic(ArithmeticCommand::Gt)),
                        "lt" => return Some(Command::Arithmetic(ArithmeticCommand::Lt)),
                        "and" => return Some(Command::Arithmetic(ArithmeticCommand::And)),
                        "or" => return Some(Command::Arithmetic(ArithmeticCommand::Or)),
                        "not" => return Some(Command::Arithmetic(ArithmeticCommand::Not)),
                        _ => panic!("Invalid command {:}", command),

                    }
                },
                Err(_) => return None
            }
        }
    }
}

fn get_trimmed_line(line: &str) -> &str {
    let without_comments = line.split("//").next().unwrap_or(&line);
    without_comments.trim()
}