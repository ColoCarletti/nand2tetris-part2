use core::panic;
use std::{fs::File, io::{self, BufWriter, Write}, path::Path};

use crate::utils::{ArithmeticCommand, Command, MemorySegment};

pub struct Writer<W: Write> {
    pub writer: BufWriter<W>,
    jump_index: u8,
}

impl Writer<File> {
    pub fn new(name: &str) -> io::Result<Self>  {
        let out_path = Path::new(name).with_extension("asm");
        let out_name = out_path.to_str().unwrap();

        let out_file = File::create(out_name)?;
        let writer = BufWriter::new(out_file);

        Ok(Writer {
            writer,
            jump_index: 0})
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

    pub fn write(&mut self, command: Command, module: &str) -> io::Result<()> {
        let _ = match command {
            Command::Pop(MemorySegment::Constant, _value) => {
                panic!()
            },
            Command::Pop(MemorySegment::Local, addr) => {
                self.pop_stack_into_segment("LCL", addr)?;
                self.decrese_sp()?;
            },
            Command::Pop(MemorySegment::Argument, addr) => {
                self.pop_stack_into_segment("ARG", addr)?;
                self.decrese_sp()?;
            },
            Command::Pop(MemorySegment::This, addr) => {
                self.pop_stack_into_segment("THIS", addr)?;
                self.decrese_sp()?;
            },
            Command::Pop(MemorySegment::That, addr) => {
                self.pop_stack_into_segment("THAT", addr)?;
                self.decrese_sp()?;
            },
            Command::Pop(MemorySegment::Static, addr) => {
                self.writeln("@SP")?;
                self.writeln("A=M-1")?;
                self.writeln("D=M")?;
                self.writeln(&format!("@{}.{}", module, addr))?;
                self.writeln("M=D")?;
                self.decrese_sp()?;
            },
            Command::Pop(MemorySegment::Temp, addr) => {
                self.writeln("@R5")?;
                self.writeln("D=A")?;
                self.writeln(&format!("@{}", addr))?;
                self.writeln("D=A+D")?;
                self.writeln("@R13")?;
                self.writeln("M=D")?;
                self.load_last_stack_value()?;
                self.writeln("@R13")?;
                self.writeln("A=M")?;
                self.writeln("M=D")?;
            },
            Command::Pop(MemorySegment::Pointer, selector) => {
                let symbol: &str;
                match selector {
                    0 => symbol = "THIS",
                    1 => symbol = "THAT",
                    _ => panic!(),
                }
                self.load_last_stack_value()?;
                self.writeln(&format!("@{}", symbol))?;
                self.writeln("M=D")?;
            },
            Command::Push(MemorySegment::Constant, value) => {
                self.load_value_into_d(value)?;
                self.push_d_into_stack()?;
                self.increase_sp()?;
            },
            Command::Push(MemorySegment::Local, addr) => {
                self.load_from_segment_into_d("LCL", addr)?;
                self.push_d_into_stack()?;
                self.increase_sp()?;
            },
            Command::Push(MemorySegment::Argument, addr) => {
                self.load_from_segment_into_d("ARG", addr)?;
                self.push_d_into_stack()?;
                self.increase_sp()?;
            },
            Command::Push(MemorySegment::This, addr) => {
                self.load_from_segment_into_d("THIS", addr)?;
                self.push_d_into_stack()?;
                self.increase_sp()?;
            },
            Command::Push(MemorySegment::That, addr) => {
                self.load_from_segment_into_d("THAT", addr)?;
                self.push_d_into_stack()?;
                self.increase_sp()?;
            },
            Command::Push(MemorySegment::Static, addr) => {
                self.writeln(&format!("@{}.{}", module, addr))?;
                self.writeln("D=M")?;
                self.push_d_into_stack()?;
                self.increase_sp()?;
            },
            Command::Push(MemorySegment::Temp, addr) => {
                self.writeln("@R5")?;
                self.writeln("D=M")?;
                self.writeln(&format!("@{}", addr))?;
                self.writeln("A=A+D")?;
                self.writeln("D=M")?;
                self.push_d_into_stack()?;
                self.increase_sp()?;
            },
            Command::Push(MemorySegment::Pointer, selector) => {
                let symbol: &str;
                match selector {
                    0 => symbol = "THIS",
                    1 => symbol = "THAT",
                    _ => panic!(),
                }
                self.writeln(&format!("@{}", symbol))?;
                self.writeln("D=M")?;
                self.push_d_into_stack()?;
                self.increase_sp()?;
            },
            Command::Arithmetic(ArithmeticCommand::Add) => {
                self.load_last_two_stack_values()?;
                self.writeln("M=M+D")?;
            },
            Command::Arithmetic(ArithmeticCommand::Sub) => {
                self.load_last_two_stack_values()?;
                self.writeln("M=M-D")?;
            },
            Command::Arithmetic(ArithmeticCommand::Neg) => {
                self.load_last_stack_value()?;
                self.writeln("M=-D")?;
                self.increase_sp()?;
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
                self.writeln("M=M&D")?;
            },
            Command::Arithmetic(ArithmeticCommand::Or) => {
                self.load_last_two_stack_values()?;
                self.writeln("M=M|D")?;
            },
            Command::Arithmetic(ArithmeticCommand::Not) => {
                self.load_last_stack_value()?;
                self.writeln("M=!D")?;
                self.increase_sp()?;
            },
            Command::Label(label) => self.writeln(&format!("({})", label))?, 
            Command::GoTo(label) => {
                self.writeln(&format!("@{}", label))?;
                self.writeln("0;JMP")?;
            }, 
            Command::IfGoTo(label) => {
                self.load_last_stack_value()?;
                self.writeln(&format!("@{label}"))?;
                self.writeln("D;JNE")?;
            },
            Command::Function(..) => todo!(),
            Command::Call(_) => todo!(),
            Command::Return => todo!(),
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

    fn load_from_segment_into_d(&mut self, memory_segment: &str, addr: u32) -> io::Result<()> {
        self.writeln(&format!("@{}", memory_segment))?;
        self.writeln("D=M")?;
        self.writeln(&format!("@{}", addr))?;
        self.writeln("A=A+D")?;
        self.writeln("D=M")?;
        Ok(())
    }

    fn pop_stack_into_segment(&mut self, memory_segment: &str, addr: u32) -> io::Result<()> {
        self.writeln(&format!("@{}", memory_segment))?;
        self.writeln("D=M")?;
        self.writeln(&format!("@{}", addr))?;
        self.writeln("D=A+D")?;
        self.writeln("@R13")?;
        self.writeln("M=D")?;
        self.writeln("@SP")?;
        self.writeln("A=M-1")?;
        self.writeln("D=M")?;
        self.writeln("@R13")?;
        self.writeln("A=M")?;
        self.writeln("M=D")?;
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
        self.writeln("D=M-D")?;
        self.writeln(&format!("@j{}", self.jump_index))?;
        self.writeln(&format!("D;{}", operation))?;

        self.writeln("@SP")?;
        self.writeln("A=M-1")?;
        self.writeln("M=0")?;
        self.writeln(&format!("@j{}end", self.jump_index))?;
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