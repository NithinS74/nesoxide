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
            self.program_counter += 1;

            match opcode {
                //INX implied opcode
                0xE8 => {
                    self.register_x = self.register_x.wrapping_add(1);
                    self.set_zero_flag(self.register_x);
                    self.set_negetive_flag(self.register_x);
                }
                //LAX immediate opcode
                0xA9 => {
                    let value = self.mem_read(self.program_counter);
                    self.program_counter += 1;
                    self.register_a = value;
                    self.set_zero_flag(value);
                    self.set_negetive_flag(value);
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
                    todo!("wild branch")
                }
            }
        }
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

            AddressingMode::ZeroPageX => self
                .mem_read(self.program_counter)
                .wrapping_add(self.register_x) as u16,

            AddressingMode::ZeroPageY => self
                .mem_read(self.program_counter)
                .wrapping_add(self.register_y) as u16,

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::AbsoluteX => self
                .mem_read_u16(self.program_counter)
                .wrapping_add(self.register_x as u16),

            AddressingMode::AbsoluteY => self
                .mem_read_u16(self.program_counter)
                .wrapping_add(self.register_y as u16),

            AddressingMode::IndirectX => {
                let ptr = self.mem_read(self.program_counter);
                self.mem_read_u16(ptr.wrapping_add(self.register_x) as u16)
            }

            AddressingMode::IndirectY => {
                let ptr = self.mem_read(self.program_counter);
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

    fn set_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.status_register |= 0b0000_0010;
        } else {
            self.status_register &= 0b1111_1101;
        }
    }

    fn set_negetive_flag(&mut self, value: u8) {
        if value & 0b1000_0000 == 0b1000_0000 {
            self.status_register |= 0b1000_0000;
        } else {
            self.status_register &= 0b0111_1111;
        }
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.run_program(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1);
    }

    #[test]
    fn test_0xe8_inx_implied_increment_x() {
        let mut cpu = CPU::new();
        cpu.run_program(vec![0xe8, 0x00]);
        assert!(cpu.status_register & 0b0000_0010 == 0b00);
        assert!(cpu.status_register & 0b1000_0000 == 0b00);
    }

    #[test]
    fn test_0xe8_inx_overflow_x() {
        let mut cpu = CPU::new();
        cpu.run_program(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.run_program(vec![0xa9, 0x05, 0x00]);
        assert!(cpu.status_register & 0b0000_0010 == 0b00);
        assert!(cpu.status_register & 0b1000_0000 == 0b00);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.run_program(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status_register & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_lda_negetive_flag() {
        let mut cpu = CPU::new();
        cpu.run_program(vec![0xa9, 0x80, 0x00]);
        assert!(cpu.status_register & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_0xaa_tax_implied_copy_data() {
        let mut cpu = CPU::new();
        cpu.run_program(vec![0xa9, 0x16, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 22);
        assert!(cpu.status_register & 0b0000_0010 == 0b00);
        assert!(cpu.status_register & 0b1000_0000 == 0b00);
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0;
        cpu.run_program(vec![0xaa, 0x00]);

        assert!(cpu.status_register & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_negetive_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1000_0010;
        cpu.run_program(vec![0xa9, 0b1000_0010, 0xaa, 0x00]);

        assert!(cpu.status_register & 0b1000_0000 == 0b1000_0000);
    }
}
