use std::env;
use std::fs::File;
use std::io::Read;

use cpu::cpu::CPU;

mod cpu;

fn main() {
    let args: Vec<String> = env::args().collect();// env::args()返回一个迭代器，collect()方法将迭代器转换为一个Vec<String>
    if args.len() != 2 {
        panic!("Usage: {} <filename>", args[0]);// 这个panic!宏会打印出错误信息并退出程序
    }// 如果命令行参数不是两个，就会打印出错误信息并退出程序
    let mut file = File::open(&args[1]).unwrap();// unwrap()方法会将Result类型解包，如果是Ok，就返回Ok中的值，如果是Err，就会调用panic!宏
    let mut buf = Vec::new();// Vec是一个可增长的数组，这里创建了一个空的Vec<u8>
    file.read_to_end(&mut buf).unwrap();// 将文件中的内容读入到buf中
    let mut cpu = CPU::new();// 创建一个CPU实例
    cpu.mem.mem[0..buf.len()].copy_from_slice(&buf);// 将buf中的内容复制到内存中
    cpu.run();// 运行CPU
}