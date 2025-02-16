const CARRY: u8 = 0b0000_0001;
const ZERO: u8 = 0b0000_0010;
const INTERRUPT: u8 = 0b0000_0100;
const DECIMAL: u8 = 0b0000_1000;
const OVERFLOW: u8 = 0b0100_0000;
const NEGETIVE: u8 = 0b1000_0000;

pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status_register: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    fn new() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status_register: 0,
            program_counter: 0,
            memory: [0 as u8; 0xFFFF],
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status_register = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn run_program(&mut self, program: Vec<u8>) {
        self.load_program(program);
        self.reset();
        self.interpret();
    }

    pub fn interpret(&mut self) {
        loop {
            let opcode: u8 = self.mem_read(self.program_counter);
            println!("op: {:X}", opcode);
            self.program_counter += 1;
            match opcode {
                //INX implied opcode
                0xE8 => {
                    self.register_x = self.register_x.wrapping_add(1);
                    self.set_zero_flag(self.register_x);
                    self.set_negetive_flag(self.register_x);
                }
                //AND immediate
                0x29 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.and(address);
                }
                //AND ZeroPage
                0x25 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.and(address);
                }
                //AND ZeroPageX
                0x35 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.and(address);
                }
                //AND Absolute
                0x2D => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.and(address);
                }
                //AND AbsoluteX
                0x3D => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.and(address);
                }
                //AND AbsoluteY
                0x39 => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteY);
                    self.and(address);
                }
                //AND IndirectX
                0x21 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectX);
                    self.and(address);
                }
                //AND IndirectY
                0x31 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectY);
                    self.and(address);
                }
                //ASL accumulator
                0x0A => {
                    self.set_carry_flag(self.register_a);
                    self.register_a <<= 1;
                    self.set_zero_flag(self.register_a);
                    self.set_negetive_flag(self.register_a);
                }
                //ASL ZeroPage
                0x06 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.asl(address);
                }
                //ASL ZeroPageX
                0x16 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.asl(address);
                }
                //ASL AbsoluteX
                0x0E => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.asl(address);
                }
                //ASL AbsoluteX
                0x1E => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.asl(address);
                }
                //BCS relative
                0xB0 => {
                    self.branch_if_true(self.status_register & CARRY == CARRY);
                }
                //BEQ relative
                0xF0 => {
                    self.branch_if_true(self.status_register & ZERO == ZERO);
                }
                //BIT bit test ZeroPage
                0x24 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.bit(address);
                }
                //BIT bit test Absolute
                0x2C => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.bit(address);
                }
                //Lda immediate opcode
                0xA9 => {
                    let address = self.get_operand_address(&AddressingMode::Immediate);
                    self.lda(address);
                }
                //Lda ZeroPage opcode
                0xA5 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPage);
                    self.lda(address);
                }
                //Lda ZeroPageX opcode
                0xB5 => {
                    let address = self.get_operand_address(&AddressingMode::ZeroPageX);
                    self.lda(address);
                }
                //Lda Absolute opcode
                0xAD => {
                    let address = self.get_operand_address(&AddressingMode::Absolute);
                    self.lda(address);
                }
                //Lda AbsoluteX opcode
                0xBD => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteX);
                    self.lda(address);
                }
                //Lda AbsoluteY opcode
                0xB9 => {
                    let address = self.get_operand_address(&AddressingMode::AbsoluteY);
                    self.lda(address);
                }
                //Lda IndirectX opcode
                0xA1 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectX);
                    self.lda(address);
                }
                //Lda IndirectY opcode
                0xB1 => {
                    let address = self.get_operand_address(&AddressingMode::IndirectY);
                    self.lda(address);
                }
                //TAX implied opcode
                0xAA => {
                    self.register_x = self.register_a;
                    self.set_zero_flag(self.register_a);
                    self.set_negetive_flag(self.register_a);
                }
                //BRk opcode
                0x00 => {
                    return;
                }
                _ => {
                    println!("Entered wild Branch with opcode {:X}", opcode);
                    panic!("wild branch");
                }
            }
        }
    }

    fn branch_if_true(&mut self, value: bool) {
        if value {
            let address = self.get_operand_address(&AddressingMode::Immediate);
            let value = self.mem_read(address);
            self.program_counter += (value - 1) as u16;
        } else {
            self.program_counter += 1;
        }
    }
    fn asl(&mut self, address: u16) {
        let mut mem_value = self.mem_read(address);
        self.set_carry_flag(mem_value);
        mem_value <<= 1;
        self.mem_write(address, mem_value);
        self.set_zero_flag(mem_value);
        self.set_negetive_flag(mem_value);
    }

    fn and(&mut self, address: u16) {
        let value = self.mem_read(address);
        self.register_a &= value;
        self.set_zero_flag(self.register_a);
        self.set_negetive_flag(self.register_a);
    }

    fn lda(&mut self, address: u16) {
        let value = self.mem_read(address);
        self.register_a = value;
        self.set_zero_flag(value);
        self.set_negetive_flag(value);
    }

    fn bit(&mut self, address: u16) {
        let value = self.mem_read(address);
        let result = self.register_a & value;
        self.set_zero_flag(result);
        self.set_negetive_flag(value);
        self.set_overflow_flag(value);
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => {
                self.program_counter += 1;
                self.program_counter - 1
            }

            AddressingMode::ZeroPage => {
                self.program_counter += 1;
                self.mem_read(self.program_counter - 1) as u16
            }

            AddressingMode::ZeroPageX => {
                self.program_counter += 1;
                self.mem_read(self.program_counter - 1)
                    .wrapping_add(self.register_x) as u16
            }

            AddressingMode::ZeroPageY => {
                self.program_counter += 1;
                self.mem_read(self.program_counter - 1)
                    .wrapping_add(self.register_y) as u16
            }

            AddressingMode::Absolute => {
                self.program_counter += 2;
                self.mem_read_u16(self.program_counter - 2)
            }

            AddressingMode::AbsoluteX => {
                self.program_counter += 2;
                self.mem_read_u16(self.program_counter - 2)
                    .wrapping_add(self.register_x as u16)
            }

            AddressingMode::AbsoluteY => {
                self.program_counter += 2;
                self.mem_read_u16(self.program_counter - 2)
                    .wrapping_add(self.register_y as u16)
            }

            AddressingMode::IndirectX => {
                self.program_counter += 1;
                let ptr = self.mem_read(self.program_counter - 1);
                self.mem_read_u16(ptr.wrapping_add(self.register_x) as u16)
            }

            AddressingMode::IndirectY => {
                self.program_counter += 1;
                let ptr = self.mem_read(self.program_counter - 1);
                self.mem_read_u16(ptr.wrapping_add(self.register_y) as u16)
            }

            AddressingMode::NoneAddressing => {
                panic!("Address mode error: NoneAddressing provided");
            }
        }
    }

    fn mem_read_u16(&mut self, addr: u16) -> u16 {
        let addr = addr as usize;
        let bytes: [u8; 2] = [self.memory[addr], self.memory[addr + 1]];
        let out = u16::from_le_bytes(bytes);
        out
    }

    fn mem_write_u16(&mut self, addr: u16, value: u16) {
        let addr = addr as usize;
        let [a, b]: [u8; 2] = value.to_le_bytes();
        self.memory[addr] = a;
        self.memory[addr + 1] = b;
    }

    fn mem_read(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    fn mem_write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    fn set_overflow_flag(&mut self, value: u8) {
        if value & OVERFLOW == OVERFLOW {
            self.status_register |= OVERFLOW;
        } else {
            self.status_register &= OVERFLOW;
        }
    }
    fn set_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.status_register |= ZERO;
        } else {
            self.status_register &= !ZERO;
        }
    }

    fn set_carry_flag(&mut self, value: u8) {
        if value & NEGETIVE == NEGETIVE {
            self.status_register |= CARRY;
        } else {
            self.status_register &= !CARRY;
        }
    }

    fn set_negetive_flag(&mut self, value: u8) {
        if value & NEGETIVE == NEGETIVE {
            self.status_register |= NEGETIVE;
        } else {
            self.status_register &= !NEGETIVE;
        }
        return;
    }
}

#[cfg(test)]
#[path = "./cpu_test.rs"]
mod cpu_tests;
