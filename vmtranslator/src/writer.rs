use std::{fs::File, io::{self, BufWriter, Write}};

use crate::utils::{ArithmeticCommand, Command};

pub struct Writer<W: Write> {
    pub writer: BufWriter<W>,
}

impl Writer<File> {
    pub fn new(out_name: &str) -> io::Result<Self>  {
        let out_file = File::create(out_name)?;
        let writer = BufWriter::new(out_file);

        Ok(Writer {writer: writer})
    }    
}

impl<W: Write> Writer<W> {
    pub fn writeln(&mut self, string: &str) -> io::Result<()> {
        self.writer.write(format!("{}\n", string).as_bytes())?;
        Ok(())
    }

    pub fn write(&mut self, command: Command) -> io::Result<()> {
        let _ = match command {
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
            Command::Arithmetic(ArithmeticCommand::Eq) => self.writeln("EQ")?,
            Command::Arithmetic(ArithmeticCommand::Gt) => self.writeln("GT")?,
            Command::Arithmetic(ArithmeticCommand::Lt) => self.writeln("LT")?,
            Command::Arithmetic(ArithmeticCommand::And) => {
                self.load_last_two_stack_values()?;
                self.writeln("M=D&M")?;
            },
            Command::Arithmetic(ArithmeticCommand::Or) => {
                self.load_last_two_stack_values()?;
                self.writeln("M=D|M")?;
            },
            Command::Arithmetic(ArithmeticCommand::Not) => self.writeln("NOT")?,
        };
        self.writeln("")?;
        Ok(())
    }

    pub fn initialize_stack_pointer(&mut self) -> io::Result<()> {
        self.writeln("// Initialize stack pointer")?;
        self.writeln("@256")?;
        self.writeln("D=A")?;
        self.writeln("@SP")?;
        self.writeln("M=D")?;
        self.writeln("")?;
        Ok(())
    }

    pub fn load_last_stack_value(&mut self) -> io::Result<()> {
        self.writeln("@SP")?;
        self.writeln("AM=M-1")?;
        self.writeln("D=M")?;
        Ok(())
    }

    pub fn load_last_two_stack_values(&mut self) -> io::Result<()> {
        self.writeln("@SP")?;
        self.writeln("AM=M-1")?;
        self.writeln("D=M")?;
        self.writeln("@SP")?;
        self.writeln("A=M-1")?;
        Ok(())
    }
}