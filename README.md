# 用rust设计的R-I-J型指令的CPU

## 1. 项目简介

本项目是一个用rust设计的R-I-J型指令的MIPS的CPU，支持的指令集如下：

### 1.1 R型指令

| 指令 | 功能 | 用法 |
| --- | --- | --- |
| add | 寄存器加法 | add rd, rs, rt |
| sub | 寄存器减法 | sub rd, rs, rt |
| and | 寄存器与 | and rd, rs, rt |
| or | 寄存器或 | or rd, rs, rt |
| xor | 寄存器异或 | xor rd, rs, rt |
| nor | 寄存器或非 | nor rd, rs, rt |
| slt | 寄存器比较 | slt rd, rs, rt |
| sll | 寄存器逻辑左移 | sll rd, rt, sa |
| srl | 寄存器逻辑右移 | srl rd, rt, sa |
| sra | 寄存器算术右移 | sra rd, rt, sa |
| jr | 寄存器跳转 | jr rs |

### 1.2 I型指令

| 指令 | 功能 | 用法 |
| --- | --- | --- |
| addi | 立即数加法 | addi rt, rs, imm |
| andi | 立即数与 | andi rt, rs, imm |
| ori | 立即数或 | ori rt, rs, imm |
| xori | 立即数异或 | xori rt, rs, imm |
| slti | 立即数比较 | slti rt, rs, imm |
| beq | 立即数比较 | beq rs, rt, offset |
| bne | 立即数比较 | bne rs, rt, offset |
| lw | 立即数比较 | lw rt, offset(rs) |
| sw | 立即数比较 | sw rt, offset(rs) |

### 1.3 J型指令

| 指令 | 功能 | 用法 |
| --- | --- | --- |
| j | 无条件跳转 | j target |
| jal | 无条件跳转 | jal target |

## 2. 项目结构

```bash

├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── cpu
│   │   ├── cpu.rs
│   │   ├── instruction.rs
│   │   ├── memory.rs
│   │   ├── alu.rs
│   │   ├── add.rs
│   │   ├── register.rs
│   │   └── mod.rs
│   ├── main.rs
│   └── test
│       ├── add.asm
│       ├── addi.asm
│       ├── and.asm
│       ├── andi.asm
│       ├── beq.asm
│       ├── bne.asm
│       ├── jal.asm
│       ├── jr.asm
│       ├── j.asm
│       ├── lw.asm
│       ├── nor.asm
│       ├── or.asm
│       ├── ori.asm
│       ├── slt.asm
│       ├── slti.asm
│       ├── sll.asm
│       ├── sra.asm
│       ├── srl.asm
│       ├── sub.asm
│       ├── sw.asm
│       ├── xor.asm
│       └── xori.asm
└── target
    ├── debug
    │   ├── build
    │   ├── cpu
    │   ├── deps
    │   ├── examples
    │   ├── incremental
    │   ├── native
    │   └── test
    └── debug-deps

```

## 3. 项目运行

```bash
# 编译
cargo build

# 运行
cargo run

# 测试
cargo test
```

## 4. 项目测试

```bash
# 测试用例
src/test/*.asm

# 测试结果
src/test/*.out
```

## 5. 项目参考

- [Rust实现的RISC-V CPU]

## 6. 项目代码

### 6.1 cpu.rs

```rust
use crate::cpu::instruction::Instruction;
use crate::cpu::memory::Memory;
use crate::cpu::register::Register;

pub mod instruction;
pub mod memory;
pub mod register;

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
```

### 6.2 instruction.rs

```rust
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
```

### 6.3 memory.rs

```rust
pub struct Memory {
    pub mem: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: vec![0; 1024 * 1024],
        }
    }

    pub fn read(&self, addr: u32) -> u32 {
        let addr = addr as usize;
        let mut ret = 0;
        for i in 0..4 {
            ret |= (self.mem[addr + i] as u32) << (i * 8);
        }
        ret
    }

    pub fn write(&mut self, addr: u32, data: u32) {
        let addr = addr as usize;
        for i in 0..4 {
            self.mem[addr + i] = (data >> (i * 8)) as u8;
        }
    }
}
```

### 6.4 register.rs

```rust
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
        self.reg[rd as usize] = (self.reg[rt as usize] as i32) >> sa;
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
```

### 6.5 main.rs

```rust
use std::env;
use std::fs::File;
use std::io::Read;

use cpu::cpu::CPU;

mod cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <filename>", args[0]);
    }
    let mut file = File::open(&args[1]).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let mut cpu = CPU::new();
    cpu.mem.mem[0..buf.len()].copy_from_slice(&buf);
    cpu.run();
}
```
### 6.6 mod.rs

```rust
pub mod cpu;
```
### 6.7 add.asm

```asm
# add.asm
add $t0, $t1, $t2
add $t3, $t4, $t5
add $t6, $t7, $t8
add $t9, $t0, $t1
```

### 6.8 addi.asm

```asm
# addi.asm
addi $t0, $t1, 0x1234
addi $t2, $t3, 0x5678
addi $t4, $t5, 0x9abc
addi $t6, $t7, 0xdef0
```
## 7. 项目作者

- [Sd0ric4](https://github.com/sd0ric4)
