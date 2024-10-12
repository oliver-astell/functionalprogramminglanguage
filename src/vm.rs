use crate::instructions;

pub trait ByteStack {
    fn push(&mut self, byte: u8);

    fn push_array(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.push(*byte);
        }
    }

    fn write(&mut self, position: usize, byte: u8);

    fn write_array(&mut self, position: usize, bytes: &[u8]) {
        let mut cursor = position;
        for byte in bytes {
            self.write(cursor, *byte);
            cursor += 1;
        }
    }
}

pub trait Program {
    fn exec(&mut self);
    fn read_params(&mut self, param_type: instructions::ParamType) -> Vec<u8>;
    fn step(&mut self) -> bool;
}

#[derive(Debug)]
pub struct VirtualMachine<const MEM_SIZE: usize> {
    pub memory: [u8; MEM_SIZE],
    pub stack_pointer: usize,
    pub program_pointer: usize
}

impl <const MEM_SIZE: usize> VirtualMachine<MEM_SIZE> {
    pub fn new<>() -> Self {
        Self {
            memory: [0; MEM_SIZE],
            stack_pointer: 0,
            program_pointer: 0,
        }
    }
}

impl <const MEM_SIZE: usize> ByteStack for VirtualMachine<MEM_SIZE> {
    fn push(&mut self, byte: u8) {
        self.memory[self.stack_pointer] = byte;
        self.stack_pointer += 1;
    }

    fn write(&mut self, position: usize, byte: u8) {
        self.memory[position] = byte;
    }
}

impl <const MEM_SIZE: usize> Program for VirtualMachine<MEM_SIZE> {
    fn exec(&mut self) {
        while self.step() {};
    }

    fn read_params(&mut self, param_type: instructions::ParamType) -> Vec<u8> {
        match param_type {
            instructions::ParamType::Nil => vec![],
            instructions::ParamType::Static(param_count) => {
                let mut vec: Vec<u8> = vec![];
                for _ in 0..param_count {
                    vec.push(self.memory[self.program_pointer]);
                    self.program_pointer += 1;
                };
                vec
            }
            instructions::ParamType::Variadic(param_count) => {
                let mut vec: Vec<u8> = vec![];
                for _ in 0..param_count {
                    vec.push(self.memory[self.program_pointer]);
                    self.program_pointer += 1;
                };
                let param_size = self.memory[self.program_pointer];
                self.program_pointer += 1;
                for _ in 0..param_size {
                    vec.push(self.memory[self.program_pointer]);
                    self.program_pointer += 1;
                }
                vec
            }
        }
    }

    fn step(&mut self) -> bool {
        let instruction = self.memory[self.program_pointer];
        self.program_pointer += 1;

        if instruction == instructions::EXIT.byte { return false }
        else if instruction == instructions::DEBUG.byte { println!("howdy"); }
        else if instruction == instructions::GOTO.byte {
            let params = self.read_params(instructions::GOTO.param_type);
            self.program_pointer = params[0] as usize;
        }
        true
    }
}