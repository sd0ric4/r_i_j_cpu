use crate::cpu::instruction::Instruction;
use crate::cpu::memory::Memory;
use crate::cpu::register::Register;

pub struct CPU {
    pub pc: u32,
    pub reg: Register,
    pub mem: Memory,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: 0,
            reg: Register::new(),
            mem: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let inst = Instruction::new(self.mem.read(self.pc));
            self.pc += 4;
            match inst.opcode {
                0x00 => {
                    match inst.funct {
                        0x20 => self.reg.add(inst.rd, inst.rs, inst.rt),
                        0x22 => self.reg.sub(inst.rd, inst.rs, inst.rt),
                        0x24 => self.reg.and(inst.rd, inst.rs, inst.rt),
                        0x25 => self.reg.or(inst.rd, inst.rs, inst.rt),
                        0x26 => self.reg.xor(inst.rd, inst.rs, inst.rt),
                        0x27 => self.reg.nor(inst.rd, inst.rs, inst.rt),
                        0x2a => self.reg.slt(inst.rd, inst.rs, inst.rt),
                        0x00 => self.reg.sll(inst.rd, inst.rt, inst.sa),
                        0x02 => self.reg.srl(inst.rd, inst.rt, inst.sa),
                        0x03 => self.reg.sra(inst.rd, inst.rt, inst.sa),
                        0x08 => {
                            self.pc = self.reg.jr(inst.rs);
                            continue;
                        }
                        _ => panic!("Unknown instruction: {:x}", inst.funct),
                    }
                }
                0x08 => self.reg.addi(inst.rt, inst.rs, inst.imm),
                0x0c => self.reg.andi(inst.rt, inst.rs, inst.imm),
                0x0d => self.reg.ori(inst.rt, inst.rs, inst.imm),
                0x0e => self.reg.xori(inst.rt, inst.rs, inst.imm),
                0x0a => self.reg.slti(inst.rt, inst.rs, inst.imm),
                0x04 => {
                    if self.reg.beq(inst.rs, inst.rt, inst.imm) {
                        self.pc = self.pc.wrapping_add((inst.imm as i16) as u32);
                    }
                }
                0x05 => {
                    if self.reg.bne(inst.rs, inst.rt, inst.imm) {
                        self.pc
                            = self.pc.wrapping_add((inst.imm as i16) as u32);
                    }
                }
                0x23 => self.reg.lw(inst.rt, inst.imm, inst.rs),
                0x2b => self.reg.sw(inst.rt, inst.imm, inst.rs),
                0x02 => {
                    self.pc = (self.pc & 0xf0000000) | (inst.target << 2);
                    continue;
                }
                0x03 => {
                    self.reg.ra = self.pc;
                    self.pc = (self.pc & 0xf0000000) | (inst.target << 2);
                    continue;
                }
                _ => panic!("Unknown instruction: {:x}", inst.opcode),
            }
        }
    }
}
