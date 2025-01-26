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
            memory: [0; 0xFFFF],
        }
    }
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let mut opcode: u8 = 0x00;
            if let Some(&instruction) = program.get(self.program_counter as usize) {
                opcode = instruction;
            }
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
                    let mut value = 0x00;
                    if let Some(&argument) = program.get(self.program_counter as usize) {
                        value = argument;
                    }
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

    fn set_zero_flag(&mut self, value: u8) {
        if value == 0 {
            self.status_register |= 0b0000_0010;
        } else {
            self.status_register &= 0b1111_1101;
        }
        return;
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
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1);
    }

    #[test]
    fn test_0xe8_inx_implied_increment_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0;
        cpu.interpret(vec![0xe8, 0x00]);
        assert!(cpu.status_register & 0b0000_0010 == 0b00);
        assert!(cpu.status_register & 0b1000_0000 == 0b00);
    }

    #[test]
    fn test_0xe8_inx_overflow_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert!(cpu.status_register & 0b0000_0010 == 0b00);
        assert!(cpu.status_register & 0b1000_0000 == 0b00);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status_register & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_lda_negetive_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x80, 0x00]);
        assert!(cpu.status_register & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_0xaa_tax_implied_copy_data() {
        let mut cpu = CPU::new();
        cpu.register_a = 22;
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 22);
        assert!(cpu.status_register & 0b0000_0010 == 0b00);
        assert!(cpu.status_register & 0b1000_0000 == 0b00);
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0;
        cpu.interpret(vec![0xaa, 0x00]);

        assert!(cpu.status_register & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_negetive_flag() {
        let mut cpu = CPU::new();
        cpu.register_a = 0b1000_0010;
        cpu.interpret(vec![0xaa, 0x00]);

        assert!(cpu.status_register & 0b1000_0000 == 0b1000_0000);
    }
}
