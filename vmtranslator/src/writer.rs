use std::{fs::File, io::{self, BufWriter, Write}};

use crate::utils::{ArithmeticCommand, Command};

pub struct Writer<W: Write> {
    pub writer: BufWriter<W>,
    jump_index: u8,
}

impl Writer<File> {
    pub fn new(out_name: &str) -> io::Result<Self>  {
        let out_file = File::create(out_name)?;
        let writer = BufWriter::new(out_file);

        Ok(Writer {writer, jump_index: 0})
    }    
}

impl<W: Write> Writer<W> {
    fn writeln(&mut self, string: &str) -> io::Result<()> {
        self.writer.write(format!("{}\n", string).as_bytes())?;
        Ok(())
    }

    pub fn write_comment(&mut self, string: &str) -> io::Result<()> {
        self.writeln(&format!("// {}", string))?;
        Ok(())
    }

    pub fn initialize_stack_pointer(&mut self) -> io::Result<()> {
        self.write_comment("Initialize stack pointer")?;
        self.writeln("@256")?;
        self.writeln("D=A")?;
        self.writeln("@SP")?;
        self.writeln("M=D")?;
        self.writeln("")?;
        Ok(())
    }

    pub fn write(&mut self, command: Command) -> io::Result<()> {
        let _ = match command {
            Command::Pop(_memory, _value) => {
                self.decrese_sp()?;
            },
            Command::Push(_memory, value) => {
                self.load_value_into_d(value)?;
                self.push_d_into_stack()?;
                self.increase_sp()?;
            },
            Command::Arithmetic(ArithmeticCommand::Add) => {
                self.load_last_two_stack_values()?;
                self.writeln("M=D+M")?;
            },
            Command::Arithmetic(ArithmeticCommand::Sub) => {
                self.load_last_two_stack_values()?;
                self.writeln("M=D-M")?;
            },
            Command::Arithmetic(ArithmeticCommand::Neg) => {
                self.load_last_stack_value()?;
                self.writeln("M=-D")?;
            },
            Command::Arithmetic(ArithmeticCommand::Eq) => {
                self.load_last_two_stack_values()?;
                self.add_compare_instructions("JEQ")?;
            },
            Command::Arithmetic(ArithmeticCommand::Gt) => {
                self.load_last_two_stack_values()?;
                self.add_compare_instructions("JGT")?;
            },
            Command::Arithmetic(ArithmeticCommand::Lt) => {
                self.load_last_two_stack_values()?;
                self.add_compare_instructions("JLT")?;
            },
            Command::Arithmetic(ArithmeticCommand::And) => {
                self.load_last_two_stack_values()?;
                self.writeln("M=D&M")?;
            },
            Command::Arithmetic(ArithmeticCommand::Or) => {
                self.load_last_two_stack_values()?;
                self.writeln("M=D|M")?;
            },
            Command::Arithmetic(ArithmeticCommand::Not) => todo!(),
            _ => println!("{:?}", command),
        };
        self.writeln("")?;
        Ok(())
    }

    fn increase_sp(&mut self) -> io::Result<()> {
        self.writeln("@SP")?;
        self.writeln("M=M+1")?;
        Ok(())
    }

    fn decrese_sp(&mut self) -> io::Result<()> {
        self.writeln("@SP")?;
        self.writeln("M=M-1")?;
        Ok(())
    }

    fn load_value_into_d(&mut self, value: u32) -> io::Result<()> {
        self.writeln(&format!("@{}", value))?;
        self.writeln("D=A")?;
        Ok(())
    }

    fn push_d_into_stack(&mut self) -> io::Result<()> {
        self.writeln("@SP")?;
        self.writeln("A=M")?;
        self.writeln("M=D")?;          
        Ok(())
    }

    fn load_last_stack_value(&mut self) -> io::Result<()> {
        self.writeln("@SP")?;
        self.writeln("AM=M-1")?;
        self.writeln("D=M")?;
        Ok(())
    }

    fn load_last_two_stack_values(&mut self) -> io::Result<()> {
        self.writeln("@SP")?;
        self.writeln("AM=M-1")?;
        self.writeln("D=M")?;
        self.writeln("@SP")?;
        self.writeln("A=M-1")?;
        Ok(())
    }

    fn add_compare_instructions(&mut self, operation: &str) -> io::Result<()> {
        self.writeln("D-M")?;
        self.writeln(&format!("@j{}", self.jump_index))?;
        self.writeln(&format!("D;{}", operation))?;

        self.writeln("@SP")?;
        self.writeln("A=M-1")?;
        self.writeln("M=0")?;
        self.writeln(&format!("j{}end", self.jump_index))?;
        self.writeln("0;JMP")?;

        self.writeln(&format!("(j{})", self.jump_index))?;
        self.writeln("@SP")?;
        self.writeln("A=M-1")?;
        self.writeln("M=-1")?;

        self.writeln(&format!("(j{}end)", self.jump_index))?;
        self.jump_index += 1;
        Ok(())
    }

    pub fn add_final_loop(&mut self) -> io::Result<()> {
        self.write_comment("Final loop")?;
        self.writeln("(end)")?;
        self.writeln("@end")?;
        self.writeln("0;JMP")?;
        Ok(())
    }
}