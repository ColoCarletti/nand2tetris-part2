use std::{env, fs};
use std::path::Path;

use vmtranslator::parser::Parser;
use vmtranslator::writer::Writer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_or_directory>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let input_files: Vec<_> = if fs::metadata(input).unwrap().is_dir() {
        fs::read_dir(input)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file())
            .filter(|path| path.extension().unwrap_or_default() == "vm")
            .collect()
    } else {
        if Path::new(input).extension().unwrap_or_default() == "vm" {
            vec![Path::new(input).to_path_buf()]
        } else {
            eprintln!("Error: The provided file is not a .vm file.");
            std::process::exit(1);
        }
    };
    let mut writer = Writer::new(input).unwrap();
    writer.initialize_stack_pointer().unwrap();
    for file in input_files {
        let parser = Parser::new(file).unwrap();
        for element in parser {
            writer.write_comment(&format!("{:?}", element)).unwrap();
            writer.write(element, "module".into()).unwrap();
        }
    };
    writer.add_final_loop().unwrap(); 
}