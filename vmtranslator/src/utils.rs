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

    fn try_from(segment: &str) -> Result<Self, Self::Error>  {
        match segment {
            "local" => Ok(MemorySegment::Local),
            "argument" => Ok(MemorySegment::Argument),
            "this" => Ok(MemorySegment::This),
            "that" => Ok(MemorySegment::That),
            "pointer" => Ok(MemorySegment::Pointer),
            "temp" => Ok(MemorySegment::Temp),
            "constant" => Ok(MemorySegment::Constant),
            "static" => Ok(MemorySegment::Static),
            _ => Err(format!("Unknown segment segment: {}", segment)),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Pop(segment, addr) => write!(f, "Pop {:?} {}", segment, addr),
            Command::Push(segment, addr) => write!(f, "Push {:?} {}", segment, addr),
            Command::Arithmetic(command) => write!(f, "{:?}", command),
        }
    }
}