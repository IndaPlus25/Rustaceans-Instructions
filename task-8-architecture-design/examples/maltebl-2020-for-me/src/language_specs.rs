#[repr(u8)]
enum OPCODE {
    ADD = 0b000,
    SUB = 0b001,
    IFEQ = 0b010,
    ADDI = 0b011,
    SUBI = 0b100,
    LI = 0b101,
    J = 0b110,
    SYSCALL = 0b111,
}
#[repr(u8)]
enum REGISTRY {
    R1 = 0b00,
    R2 = 0b01,
    R3 = 0b10,
    R4 = 0b11,
}

#[repr(u8)]
enum SYSCALL {
    PRINT = 0b00,
    READ = 0b01,
    TERMINATE = 0b10,
}

pub const OP_DICTIONARY: [(char, u8); 8] = [
    ('💘', OPCODE::ADD as u8),
    ('💔', OPCODE::SUB as u8),
    ('🤔', OPCODE::IFEQ as u8),
    ('👆', OPCODE::ADDI as u8),
    ('👇', OPCODE::SUBI as u8),
    ('👈', OPCODE::LI as u8),
    ('♿', OPCODE::J as u8),
    ('🤖', OPCODE::SYSCALL as u8),
];

pub const REG_DICTIONARY: [(char, u8); 4] = [
    ('🍩', REGISTRY::R1 as u8),
    ('👀', REGISTRY::R2 as u8),
    ('🍎', REGISTRY::R3 as u8),
    ('🍊', REGISTRY::R4 as u8),
];

pub const SYSCALL_DICTIONARY: [(char, u8); 3] = [
    ('📢', SYSCALL::PRINT as u8),
    ('📜', SYSCALL::READ as u8),
    ('🔪', SYSCALL::TERMINATE as u8),
];

pub const MACRO: char = '🔓';
pub const END_MACRO: char = '🔒';
pub const CODE: char = '💬';
