use crate::cpu::memory::Memory;

pub struct Register {
    pub reg: Vec<u32>,
    pub pc: u32,
    pub ra: u32,
}

impl Register {
    pub fn new() -> Register {
        Register {
            reg: vec![0; 32],
            pc: 0,
            ra: 0,
        }
    }

    pub fn add(&mut self, rd: u8, rs: u8, rt: u8) {
        self.reg[rd as usize] = self.reg[rs as usize].wrapping_add(self.reg[rt as usize]);
    }

    pub fn sub(&mut self, rd: u8, rs: u8, rt: u8) {
        self.reg[rd as usize] = self.reg[rs as usize].wrapping_sub(self.reg[rt as usize]);
    }

    pub fn and(&mut self, rd: u8, rs: u8, rt: u8) {
        self.reg[rd as usize] = self.reg[rs as usize] & self.reg[rt as usize];
    }

    pub fn or(&mut self, rd: u8, rs: u8, rt: u8) {
        self.reg[rd as usize] = self.reg[rs as usize] | self.reg[rt as usize];
    }

    pub fn xor(&mut self, rd: u8, rs: u8, rt: u8) {
        self.reg[rd as usize] = self.reg[rs as usize] ^ self.reg[rt as usize];
    }

    pub fn nor(&mut self, rd: u8, rs: u8, rt: u8) {
        self.reg[rd as usize] = !(self.reg[rs as usize] | self.reg[rt as usize]);
    }

    pub fn slt(&mut self, rd: u8, rs: u8, rt: u8) {
        self.reg[rd as usize] = if (self.reg[rs as usize] as i32) < (self.reg[rt as usize] as i32) {
            1
        } else {
            0
        };
    }

    pub fn sll(&mut self, rd: u8, rt: u8, sa: u8) {
        self.reg[rd as usize] = self.reg[rt as usize] << sa;
    }

    pub fn srl(&mut self, rd: u8, rt: u8, sa: u8) {
        self.reg[rd as
            usize] = (self.reg[rt as usize] as u32) >> sa;
    }

    pub fn sra(&mut self, rd: u8, rt: u8, sa: u8) {
        self.reg[rd as usize] = (self.reg[rt as usize] as u32) >> sa;
    }

    pub fn jr(&mut self, rs: u8) -> u32 {
        self.reg[rs as usize]
    }

    pub fn addi(&mut self, rt: u8, rs: u8, imm: u16) {
        self.reg[rt as usize] = self.reg[rs as usize].wrapping_add(imm as u32);
    }

    pub fn andi(&mut self, rt: u8, rs: u8, imm: u16) {
        self.reg[rt as usize] = self.reg[rs as usize] & (imm as u32);
    }

    pub fn ori(&mut self, rt: u8, rs: u8, imm: u16) {
        self.reg[rt as usize] = self.reg[rs as usize] | (imm as u32);
    }

    pub fn xori(&mut self, rt: u8, rs: u8, imm: u16) {
        self.reg[rt as usize] = self.reg[rs as usize] ^ (imm as u32);
    }

    pub fn slti(&mut self, rt: u8, rs: u8, imm: u16) {
        self.reg[rt as usize] = if (self.reg[rs as usize] as i32) < (imm as i16) {
            1
        } else {
            0
        };
    }

    pub fn beq(&mut self, rs: u8, rt: u8, imm: u16) -> bool {
        self.reg[rs as usize] == self.reg[rt as usize]
    }

    pub fn bne(&mut self, rs: u8, rt: u8, imm: u16) -> bool {
        self.reg[rs as usize] != self.reg[rt as usize]
    }

    pub fn lw(&mut self, rt: u8, imm: u16, rs: u8) {
        self.reg[rt as usize] = self.mem.read(self.reg[rs as usize].wrapping_add(imm as u32));
    }

    pub fn sw(&mut self, rt: u8, imm: u16, rs: u8) {
        self.mem.write(self.reg[rs as usize].wrapping_add(imm as u32), self.reg[rt as usize]);
    }

    pub fn mem(&mut self, mem: Memory) {
        self.mem = mem;
    }

    pub fn dump(&self) {
        println!("pc: {:08x}", self.pc);
        println!("ra: {:08x}", self.ra);
        for i in 0..32 {
            println!("r{:02}: {:08x}", i, self.reg[i]);
        }
    }
}