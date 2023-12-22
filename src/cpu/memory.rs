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