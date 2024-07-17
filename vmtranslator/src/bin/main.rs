use std::env;

use vmtranslator::parser::Parser;
use vmtranslator::writer::Writer;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let name = &args[1];
            let parser = Parser::new(&name).unwrap();
            let mut writer = Writer::new(&name).unwrap();

            writer.initialize_stack_pointer().unwrap();
            for element in parser {
                writer.write_comment(&format!("{}", element)).unwrap();
                writer.write(element).unwrap();
            }
            writer.add_final_loop().unwrap();
        }
        _ => {
            println!("Usage:g input_file");
        }
    }
}