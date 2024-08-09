use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

use crate::utils::{ArithmeticCommand, Command, MemorySegment};

pub struct Parser<R: Read> {
    pub reader: BufReader<R>,
}

impl Parser<File> {
    pub fn new(input_path: PathBuf) -> io::Result<Self>  {
        let input_file = File::open(input_path)?;
        let reader = BufReader::new(input_file);
        Ok(Parser {reader})
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
                        "pop" => return {
                            let segment_str = split.next().unwrap();
                            let segment = MemorySegment::try_from(segment_str).unwrap();
                            let index_str = split.next().unwrap();
                            let index = index_str.parse::<u32>().unwrap();
                            Some(Command::Pop(segment, index))
                        },
                        "push" => return {
                            let segment_str = split.next().unwrap();
                            let segment = MemorySegment::try_from(segment_str).unwrap();
                            let index_str = split.next().unwrap();
                            let index = index_str.parse::<u32>().unwrap();
                            Some(Command::Push(segment, index))
                        },
                        "label" => return Some(Command::Label(split.next().unwrap().into())),
                        "goto" => return Some(Command::GoTo(split.next().unwrap().into())),
                        "if-goto" => return Some(Command::IfGoTo(split.next().unwrap().into())),
                        "function" => return { 
                            let function_name = split.next().unwrap();
                            let arguments_str = split.next().unwrap();
                            let arguments = arguments_str.parse::<u32>().unwrap();
                            Some(Command::Function(function_name.into(), arguments))
                        },
                        _ => panic!("Invalid command {:}", command),
                        }
                    }
                Err(_) => return None
            }
        }
    }
}

fn get_trimmed_line(line: &str) -> &str {
    let without_comments = line.split("//").next().unwrap_or(&line);
    without_comments.trim()
}