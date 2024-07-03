#[derive(Debug)]
pub enum Command {
    Arithmetic(ArithmeticCommand),
}

#[derive(Debug)]
pub enum ArithmeticCommand {
    Add,    // x + y
    Sub,    // x - y
    Neg,    // -y
    Eq,     // x == y
    Gt,     // x > y
    Lt,     // x < y
    And,    // x & y
    Or,     // x | y
    Not,    // !y
}