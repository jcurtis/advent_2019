use std::ops::ControlFlow;

pub type Memory = Vec<u32>;

#[derive(Debug, Clone)]
pub struct Computer {
    pub pointer: usize,
    pub memory: Memory,
}

impl Computer {
    pub fn new() -> Self {
        Self::new_with(vec![])
    }

    pub fn new_with(memory: Vec<u32>) -> Self {
        Self { pointer: 0, memory }
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn run_command(&mut self) -> ControlFlow<Result<String, String>, u32> {
        let command = self.memory[self.pointer];
        println!("Running command at {} = {command}", self.pointer);
        match command {
            1 => match self.add() {
                Ok(res) => ControlFlow::Continue(res),
                Err(msg) => ControlFlow::Break(Err(msg)),
            },
            2 => match self.mul() {
                Ok(res) => ControlFlow::Continue(res),
                Err(msg) => ControlFlow::Break(Err(msg)),
            },
            99 => ControlFlow::Break(Ok("exit 99".to_string())),
            _ => unimplemented!(),
        }
    }

    pub fn run_program(&mut self, parameters: Option<(u32, u32)>) -> Result<u32, String> {
        if let Some((noun, verb)) = parameters {
            self.memory[1] = noun;
            self.memory[2] = verb;
        }
        const MAX_ITER: u32 = 100;
        let err_msg = format!("Hit max iterations {MAX_ITER}");
        let mut i = 0;
        loop {
            i += 1;
            if i > MAX_ITER {
                return Err(err_msg);
            }

            if let ControlFlow::Break(res) = self.run_command() {
                if let Err(msg) = res {
                    println!("Program error: {msg}");
                    return Err(msg);
                }
                break;
            }
        }
        Ok(self.memory[0])
    }

    fn add(&mut self) -> Result<u32, String> {
        let a = self.memory[self.pointer + 1] as usize;
        let b = self.memory[self.pointer + 2] as usize;
        let to = self.memory[self.pointer + 3] as usize;

        let mem_size = self.memory.len();
        if a >= mem_size || b >= mem_size || to >= mem_size {
            let err_msg =
                format!("Index out of bounds, memory size={mem_size}, a={a}, b={b}, to={to}.");
            return Err(err_msg);
        }

        self.memory[to] = self.memory[a] + self.memory[b];
        self.pointer += 4;
        Ok(self.memory[to])
    }

    fn mul(&mut self) -> Result<u32, String> {
        let a = self.memory[self.pointer + 1] as usize;
        let b = self.memory[self.pointer + 2] as usize;
        let to = self.memory[self.pointer + 3] as usize;

        let mem_size = self.memory.len();
        if a >= mem_size || b >= mem_size || to >= mem_size {
            let err_msg =
                format!("Index out of bounds, memory size={mem_size}, a={a}, b={b}, to={to}.");
            return Err(err_msg);
        }

        self.memory[to] = self.memory[a] * self.memory[b];
        self.pointer += 4;
        Ok(self.memory[to])
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
