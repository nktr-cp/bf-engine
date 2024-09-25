use std::fs::File;
use std::io::Read;

const MEMORY_SIZE: usize = 30000;

pub struct Interpreter {
    memory: [u8; MEMORY_SIZE],
    data_pointer: usize,
    instruction_pointer: usize,
    instructions: Vec<char>,
}

impl Interpreter {
    pub fn new(filename: &str) -> Self {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                std::process::exit(1);
            }
        };

        let mut code = String::new();
        if let Err(e) = file.read_to_string(&mut code) {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }

        Interpreter {
            memory: [0; MEMORY_SIZE],
            data_pointer: 0,
            instruction_pointer: 0,
            instructions: code.chars().filter(|c| "+-<>.,[]".contains(*c)).collect(),
        }
    }

    fn consume(&mut self, c: char) -> bool {
        if self.instructions[self.instruction_pointer] == c {
            self.instruction_pointer += 1;
            return true;
        }
        false
    }

    fn equal(&mut self, c: char) -> bool {
        self.instructions[self.instruction_pointer] == c
    }

    pub fn run(&mut self) {
        while self.instruction_pointer < self.instructions.len() {
            if self.consume('>') {
                self.data_pointer = (self.data_pointer + 1) % MEMORY_SIZE;
            } else if self.consume('<') {
                self.data_pointer = (self.data_pointer + MEMORY_SIZE - 1) % MEMORY_SIZE;
            } else if self.consume('+') {
                self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_add(1);
            } else if self.consume('-') {
                self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_sub(1);
            } else if self.consume('.') {
                print!("{}", self.memory[self.data_pointer] as char);
            } else if self.consume(',') {
                let mut buf = [0];
                std::io::stdin().read_exact(&mut buf).unwrap();
                self.memory[self.data_pointer] = buf[0];
            } else if self.equal('[') {
                if self.memory[self.data_pointer] == 0 {
                    let mut level = 1;
                    while level > 0 {
                        self.instruction_pointer += 1;
                        match self.instructions[self.instruction_pointer] {
                            '[' => level += 1,
                            ']' => level -= 1,
                            _ => {}
                        }
                    }
                }
                self.instruction_pointer += 1;
            } else if self.equal(']') {
                if self.memory[self.data_pointer] != 0 {
                    let mut level = 1;
                    while level > 0 {
                        self.instruction_pointer -= 1;
                        match self.instructions[self.instruction_pointer] {
                            '[' => level -= 1,
                            ']' => level += 1,
                            _ => {}
                        }
                    }
                }
                self.instruction_pointer += 1;
            } else {
                unreachable!();
            }
        }
    }
}
