use std::fmt;

#[derive(Debug)]
pub enum Command {
    Pop(MemorySegment, u32),
    Push(MemorySegment, u32),
    Arithmetic(ArithmeticCommand),
}

#[derive(Debug)]
pub enum ArithmeticCommand {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug)]
pub enum MemorySegment {
    Local,
    Argument,
    This,
    That,
    Pointer,
    Temp,
    Constant,
    Static,
}

impl TryFrom<&str> for MemorySegment {
    type Error = String;

    fn try_from(memory: &str) -> Result<Self, Self::Error>  {
        match memory {
            "local" => Ok(MemorySegment::Local),
            "argument" => Ok(MemorySegment::Argument),
            "this" => Ok(MemorySegment::This),
            "that" => Ok(MemorySegment::That),
            "pointer" => Ok(MemorySegment::Pointer),
            "temp" => Ok(MemorySegment::Temp),
            "constant" => Ok(MemorySegment::Constant),
            "static" => Ok(MemorySegment::Static),
            _ => Err(format!("Unknown memory segment: {}", memory)),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Pop(memory, addr) => write!(f, "Pop {:?} {}", memory, addr),
            Command::Push(memory, addr) => write!(f, "Push {:?} {}", memory, addr),
            Command::Arithmetic(command) => write!(f, "{:?}", command),
        }
    }
}