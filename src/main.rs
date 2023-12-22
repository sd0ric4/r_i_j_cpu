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