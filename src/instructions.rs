#[derive(Debug)]
pub enum InstructionType {
    Load,
    Store,
    Add,
    Subtract,
    Input,
    Output,
    Halt,
}

#[derive(Debug)]
pub struct Instruction {
    pub kind: InstructionType,
    pub address: u64,
}
