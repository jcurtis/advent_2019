pub type Memory = Vec<u32>;

#[derive(Debug)]
pub struct Computer {
    pub ins_pointer: usize,
    pub memory: Memory,
}

impl Computer {
    pub fn new() -> Self {
        Self::new_with(vec![])
    }

    pub fn new_with(memory: Vec<u32>) -> Self {
        Self {
            ins_pointer: 0,
            memory,
        }
    }

    // pub fn from(input: &str) -> Self {
    //     let memory = input.split(',').map(|num| num.parse().unwrap()).collect();
    //     Self::new_with(memory)
    // }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn run_command(&mut self) -> Option<u32> {
        let command = self.memory[self.ins_pointer];
        match command {
            1 => Some(self.add()),
            2 => Some(self.mul()),
            99 => None,
            _ => unreachable!(),
        }
    }

    fn add(&mut self) -> u32 {
        let a = self.memory[self.ins_pointer + 1];
        let b = self.memory[self.ins_pointer + 2];
        let to = self.ins_pointer + 3 as usize;
        self.memory[to] = a + b;
        self.memory[to]
    }

    fn mul(&mut self) -> u32 {
        let a = self.memory[self.ins_pointer + 1];
        let b = self.memory[self.ins_pointer + 2];
        let to = self.ins_pointer + 3 as usize;
        self.memory[to] = a * b;
        self.memory[to]
    }
}

impl Default for Computer {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for Computer {
    fn from(input: &str) -> Self {
        let memory = input.split(',').map(|num| num.parse().unwrap()).collect();
        Self::new_with(memory)
    }
}
