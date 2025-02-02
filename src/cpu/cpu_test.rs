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
    assert!(cpu.status_register & ZERO == 0b00);
    assert!(cpu.status_register & NEGETIVE == 0b00);
}

#[test]
fn test_0xe8_inx_overflow_x() {
    let mut cpu = CPU::new();
    cpu.run_program(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);
    assert_eq!(cpu.register_x, 1);
}

#[test]
fn test_0xa9_lda_load_data() {
    let mut cpu = CPU::new();
    //Lda Immediate opcode
    cpu.run_program(vec![0xa9, 0x05, 0x00]);
    assert!(cpu.status_register & ZERO == 0b00);
    assert!(cpu.status_register & NEGETIVE == 0b00);
    assert_eq!(cpu.register_a, 0x05);
    //Lda ZeroPage opcode
    cpu.memory[0xaa] = 0x05;
    cpu.run_program(vec![0xA5, 0xaa, 0x00]);
    assert!(cpu.status_register & ZERO == 0b00);
    assert!(cpu.status_register & NEGETIVE == 0b00);
    assert_eq!(cpu.register_a, 0x05);
    //Lda ZeroPageX opcode
    cpu.memory[0xa9] = 0x05;
    cpu.run_program(vec![0xB5, 0xa9, 0x00]);
    assert!(cpu.status_register & ZERO == 0b00);
    assert!(cpu.status_register & NEGETIVE == 0b00);
    assert_eq!(cpu.register_a, 0x05);
    //Lda Absolute opcode
    cpu.memory[0x8500] = 0x05;
    cpu.run_program(vec![0xAD, 0x00, 0x85, 0x00]);
    assert!(cpu.status_register & ZERO == 0b00);
    assert!(cpu.status_register & NEGETIVE == 0b00);
    assert_eq!(cpu.register_a, 0x05);
    //Lda AbsoluteX opcode
    cpu.run_program(vec![0xBD, 0x00, 0x85, 0x00]);
    cpu.memory[0xaa] = 0x05;
    assert!(cpu.status_register & ZERO == 0b00);
    assert!(cpu.status_register & NEGETIVE == 0b00);
    assert_eq!(cpu.register_a, 0x05);
    //Lda AbsoluteY opcode
    cpu.run_program(vec![0xB9, 0x00, 0x85, 0x00]);
    cpu.memory[0xaa] = 0x05;
    assert!(cpu.status_register & ZERO == 0b00);
    assert!(cpu.status_register & NEGETIVE == 0b00);
    assert_eq!(cpu.register_a, 0x05);
    //Lda IndirectX opcode
    cpu.memory[0xa9] = 0x00;
    cpu.memory[0xaa] = 0x85;
    cpu.memory[0x8500] = 0x05;
    cpu.run_program(vec![0xA1, 0xa9, 0x00]);
    assert!(cpu.status_register & ZERO == 0b00);
    assert!(cpu.status_register & NEGETIVE == 0b00);
    assert_eq!(cpu.register_a, 0x05);
    //Lda IndirectY opcode
    cpu.memory[0xa9] = 0x00;
    cpu.memory[0xaa] = 0x85;
    cpu.memory[0x8500] = 0x05;
    cpu.run_program(vec![0xB1, 0xa9, 0x00]);
    assert!(cpu.status_register & ZERO == 0b00);
    assert!(cpu.status_register & NEGETIVE == 0b00);
    assert_eq!(cpu.register_a, 0x05);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.run_program(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.status_register & ZERO == 0b10);
}

#[test]
fn test_0xa9_lda_negetive_flag() {
    let mut cpu = CPU::new();
    cpu.run_program(vec![0xa9, 0x80, 0x00]);
    assert!(cpu.status_register & NEGETIVE == NEGETIVE);
}

#[test]
fn test_0xaa_tax_implied_copy_data() {
    let mut cpu = CPU::new();
    cpu.run_program(vec![0xa9, 0x16, 0xaa, 0x00]);

    assert_eq!(cpu.register_x, 22);
    assert!(cpu.status_register & ZERO == 0b00);
    assert!(cpu.status_register & NEGETIVE == 0b00);
}

#[test]
fn test_0xaa_tax_zero_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0;
    cpu.run_program(vec![0xaa, 0x00]);

    assert!(cpu.status_register & ZERO == 0b10);
}

#[test]
fn test_0xaa_tax_negetive_flag() {
    let mut cpu = CPU::new();
    cpu.register_a = 0b1000_0010;
    cpu.run_program(vec![0xa9, 0b1000_0010, 0xaa, 0x00]);

    assert!(cpu.status_register & NEGETIVE == NEGETIVE);
}
