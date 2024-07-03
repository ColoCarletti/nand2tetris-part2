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