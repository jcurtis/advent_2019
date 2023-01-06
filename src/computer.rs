use std::ops::ControlFlow;

pub type Memory = Vec<i32>;

#[derive(Debug, Clone)]
pub struct Computer {
    pub pointer: usize,
    pub memory: Memory,
    pub input: Option<i32>,
    pub output: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpTrue = 5,
    JumpFalse = 6,
    LessThan = 7,
    Equal = 8,
    Exit = 99,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

type Modes = [ParameterMode; 3];
type Command = (OpCode, Modes);

impl Computer {
    pub fn new() -> Self {
        Self::new_with(vec![])
    }

    pub fn new_with(memory: Memory) -> Self {
        Self {
            pointer: 0,
            memory,
            input: None,
            output: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn read_command(&self) -> Command {
        let command = self.memory[self.pointer] as u32;
        let op = (command.digit(1) * 10) + command.digit(0);
        let mode_1 = ParameterMode::from(command.digit(2));
        let mode_2 = ParameterMode::from(command.digit(3));
        let mode_3 = ParameterMode::from(command.digit(4));

        (OpCode::from(op), [mode_1, mode_2, mode_3])
    }

    pub fn run_command(&mut self) -> ControlFlow<Result<String, String>, i32> {
        let (op, modes) = self.read_command();
        // println!("Running command at {} = {op:?}", self.pointer);
        match op {
            OpCode::Add => match self.add(modes) {
                Ok(res) => ControlFlow::Continue(res),
                Err(msg) => ControlFlow::Break(Err(msg)),
            },
            OpCode::Mul => match self.mul(modes) {
                Ok(res) => ControlFlow::Continue(res),
                Err(msg) => ControlFlow::Break(Err(msg)),
            },
            OpCode::Input => match self.input(modes) {
                Ok(res) => ControlFlow::Continue(res),
                Err(msg) => ControlFlow::Break(Err(msg)),
            },
            OpCode::Output => match self.output(modes) {
                Ok(res) => ControlFlow::Continue(res),
                Err(msg) => ControlFlow::Break(Err(msg)),
            },
            OpCode::JumpTrue => match self.jump_true(modes) {
                Ok(res) => ControlFlow::Continue(res),
                Err(msg) => ControlFlow::Break(Err(msg)),
            },
            OpCode::JumpFalse => match self.jump_false(modes) {
                Ok(res) => ControlFlow::Continue(res),
                Err(msg) => ControlFlow::Break(Err(msg)),
            },
            OpCode::LessThan => match self.jump_less_than(modes) {
                Ok(res) => ControlFlow::Continue(res),
                Err(msg) => ControlFlow::Break(Err(msg)),
            },
            OpCode::Equal => match self.jump_equal(modes) {
                Ok(res) => ControlFlow::Continue(res),
                Err(msg) => ControlFlow::Break(Err(msg)),
            },
            OpCode::Exit => ControlFlow::Break(Ok("exit 99".to_string())),
            // _ => unimplemented!(),
        }
    }

    pub fn run_program(&mut self) -> Result<i32, String> {
        const MAX_ITER: u32 = 1000;
        let err_msg = format!("Hit max iterations {MAX_ITER}");
        let mut i = 0;
        loop {
            i += 1;
            if i > MAX_ITER {
                return Err(err_msg);
            }

            if let ControlFlow::Break(res) = self.run_command() {
                if let Err(msg) = res {
                    eprintln!("Program error: {msg}");
                    return Err(msg);
                }
                break;
            }
        }
        Ok(self.memory[0])
    }

    pub fn run_from(program: &str, input: Option<i32>) -> Result<i32, String> {
        let mut computer = Computer::from(program);
        computer.input = input;
        computer.run_program()?;
        computer
            .check_diagnostics()
            .or(Err("Error running program".to_string()))
    }

    pub fn set_parameters(&mut self, (noun, verb): (i32, i32)) -> Result<(), String> {
        if self.len() >= 2 {
            self.memory[1] = noun;
            self.memory[2] = verb;
            Ok(())
        } else {
            let msg = format!("Not enough memory to set parameters {:?}", &self.memory);
            Err(msg)
        }
    }

    fn get_pos(&self, pos: usize, mode: &ParameterMode) -> usize {
        match mode {
            ParameterMode::Immediate => pos,
            ParameterMode::Position => self.memory[pos] as usize,
        }
    }

    fn get_pos_3(&self, modes: Modes) -> Result<(usize, usize, usize), String> {
        let a = self.get_pos(self.pointer + 1, &modes[0]);
        let b = self.get_pos(self.pointer + 2, &modes[1]);
        let to = self.get_pos(self.pointer + 3, &modes[2]);

        let mem_size = self.memory.len();
        if a >= mem_size || b >= mem_size || to >= mem_size {
            let err_msg =
                format!("Index out of bounds, memory size={mem_size}, a={a}, b={b}, to={to}.");
            return Err(err_msg);
        }

        Ok((a, b, to))
    }

    fn add(&mut self, modes: Modes) -> Result<i32, String> {
        let (a, b, to) = self.get_pos_3(modes)?;

        self.memory[to] = self.memory[a] + self.memory[b];
        self.pointer += 4;
        Ok(self.memory[to])
    }

    fn mul(&mut self, modes: Modes) -> Result<i32, String> {
        let (a, b, to) = self.get_pos_3(modes)?;

        self.memory[to] = self.memory[a] * self.memory[b];
        self.pointer += 4;
        Ok(self.memory[to])
    }

    fn input(&mut self, modes: Modes) -> Result<i32, String> {
        let address = self.get_pos(self.pointer + 1, &modes[0]);
        if self.len() > address {
            if let Some(input) = self.input {
                self.memory[address] = input;
                self.pointer += 2;
                Ok(address as i32)
            } else {
                Err("Input is None".to_string())
            }
        } else {
            let msg = format!("Address {address} not in memory");
            Err(msg)
        }
    }

    fn output(&mut self, modes: Modes) -> Result<i32, String> {
        let address = self.get_pos(self.pointer + 1, &modes[0]);
        if self.len() > address {
            let res = self.memory[address];
            self.output.push(res);
            self.pointer += 2;
            Ok(res)
        } else {
            let msg = format!("Address {address} not in memory");
            Err(msg)
        }
    }

    pub fn check_diagnostics(&self) -> Result<i32, &[i32]> {
        let len = self.output.len();
        let diagnostics = &self.output[0..(len - 1)];
        if diagnostics.iter().any(|&diag| diag != 0) {
            eprintln!("Diagnostics failed! Output: {:?}", &diagnostics);
            Err(diagnostics)
        } else {
            let res = self.output.last().unwrap();
            Ok(*res)
        }
    }

    fn jump_if(&mut self, modes: Modes, cmp: fn(i32) -> bool) -> Result<i32, String> {
        let a = self.get_pos(self.pointer + 1, &modes[0]);
        let a_val = self.memory[a];
        if cmp(a_val) {
            let b = self.get_pos(self.pointer + 2, &modes[1]);
            self.pointer = self.memory[b] as usize;
            Ok(a_val)
        } else {
            self.pointer += 3;
            Ok(a_val)
        }
    }

    fn jump_true(&mut self, modes: Modes) -> Result<i32, String> {
        self.jump_if(modes, |val| val != 0)
    }

    fn jump_false(&mut self, modes: Modes) -> Result<i32, String> {
        self.jump_if(modes, |val| val == 0)
    }

    fn jump_less_than(&mut self, modes: Modes) -> Result<i32, String> {
        let a = self.get_pos(self.pointer + 1, &modes[0]);
        let b = self.get_pos(self.pointer + 2, &modes[1]);
        let to = self.get_pos(self.pointer + 3, &modes[2]);

        let a_val = self.memory[a];
        let b_val = self.memory[b];

        let res = if a_val < b_val { 1 } else { 0 };

        self.memory[to] = res;
        self.pointer += 4;

        Ok(res)
    }

    fn jump_equal(&mut self, modes: Modes) -> Result<i32, String> {
        let a = self.get_pos(self.pointer + 1, &modes[0]);
        let b = self.get_pos(self.pointer + 2, &modes[1]);
        let to = self.get_pos(self.pointer + 3, &modes[2]);

        let a_val = self.memory[a];
        let b_val = self.memory[b];

        let res = if a_val == b_val { 1 } else { 0 };

        self.memory[to] = res;
        self.pointer += 4;

        Ok(res)
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

impl From<u32> for OpCode {
    fn from(input: u32) -> Self {
        match input {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpTrue,
            6 => OpCode::JumpFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equal,
            99 => OpCode::Exit,
            _ => unimplemented!(),
        }
    }
}

impl From<u32> for ParameterMode {
    fn from(input: u32) -> Self {
        match input {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => unimplemented!(),
        }
    }
}

pub trait Digits<T> {
    fn digit(&self, pos: T) -> T;
}

impl Digits<u32> for u32 {
    fn digit(&self, pos: u32) -> u32 {
        self / 10_u32.pow(pos) % 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit() {
        assert_eq!(1002.digit(0), 2);
        assert_eq!(1002.digit(1), 0);
        assert_eq!(1002.digit(2), 0);
        assert_eq!(1002.digit(3), 1);
        assert_eq!(1002.digit(4), 0);
    }
}
