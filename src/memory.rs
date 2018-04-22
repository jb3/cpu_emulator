#[derive(Debug, Clone)]
pub struct Memory {
    pub items: Vec<u64>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            items: vec![0; 100],
        }
    }
}
