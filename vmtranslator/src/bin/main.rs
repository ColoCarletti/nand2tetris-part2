use std::env;
use std::path::Path;

use vmtranslator::parser::Parser;
use vmtranslator::writer::Writer;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let in_name = &args[1];        
            let out_path = Path::new(in_name).with_extension("asm");
            let out_name = out_path.to_str().unwrap();
            let parser = Parser::new(in_name).unwrap();
            let mut writer = Writer::new(&out_name).unwrap();

            writer.initialize_stack_pointer().unwrap();
            for element in parser {
                writer.write(element).unwrap();
            }
        }
        _ => {
            println!("Usage: vm-to-hack input_file");
        }
    }
}