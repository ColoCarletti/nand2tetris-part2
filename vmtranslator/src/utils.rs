use std::fmt;

#[derive(Debug)]
pub enum Command {
    Pop(MemorySegment, u32),
    Push(MemorySegment, u32),
    Arithmetic(ArithmeticCommand),
    Label(String),
    GoTo(String),
    IfGoTo(String),
    Function(String, u32),
    Call(String),
    Return,
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
            _ => Err(format!("Unknown segment: {}", segment)),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Pop(segment, addr) => write!(f, "Pop {:?} {}", segment, addr),
            Command::Push(segment, addr) => write!(f, "Push {:?} {}", segment, addr),
            Command::Arithmetic(command) => write!(f, "{:?}", command),
            Command::Label(label) => write!(f, "Label {}", label),
            Command::GoTo(label) => write!(f, "GoTo {}", label),
            Command::IfGoTo(label) => write!(f, "If GoTo {}", label),
            Command::Function(name, arguments) => write!(f, "Function {} {}", name, arguments),
            Command::Call(name) => write!(f, "Call {}", name),
            Command::Return => write!(f, "Return"),
        }
    }
}