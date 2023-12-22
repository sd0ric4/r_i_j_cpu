pub struct Instruction {
    pub opcode: u8,
    pub rs: u8,
    pub rt: u8,
    pub rd: u8,
    pub sa: u8,
    pub funct: u8,
    pub imm: u16,
    pub target: u32,
}

impl Instruction {
    pub fn new(inst: u32) -> Instruction {
        Instruction {
            opcode: ((inst & 0xfc000000) >> 26) as u8,
            rs: ((inst & 0x03e00000) >> 21) as u8,
            rt: ((inst & 0x001f0000) >> 16) as u8,
            rd: ((inst & 0x0000f800) >> 11) as u8,
            sa: ((inst & 0x000007c0) >> 6) as u8,
            funct: (inst & 0x0000003f) as u8,
            imm: (inst & 0x0000ffff) as u16,
            target: inst & 0x03ffffff,
        }
    }
}