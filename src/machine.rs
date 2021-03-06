use components;
use std::fmt::{Display, Error};

#[derive(Debug)]
pub struct Machine {
    acca: u32,
    accb: u32,
    accc: u64,
    baka: u32,
    bakb: u32,
    bakc: u32,
    ip: u32,
    sp: u32,
    bp: u32,
    flag: u32,
    mem: Box<components::Memory>,
    rom: Vec<u8>,
}

impl Machine {
    pub fn new(mem_size: u32, rom: Vec<u8>) -> Self {
        Self { 
            acca: 0,
            accb: 0,
            accc: 0,
            baka: 0,
            bakb: 0,
            bakc: 0,
            ip: 0,
            sp: 0,
            bp: 0,
            flag: 0,
            mem: Box::new(components::Memory::new(mem_size)),
            rom: rom
        }
    }

    pub fn start(&mut self) {
        for i in 0..255 {
            // little endian words
            let val = self.rom[3*i+2] as u32
                + ((self.rom[3*i+1] as u32) << 8)
                + ((self.rom[3*i] as u32) << 16);
            self.mem[i].set(val);
        }
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
        write!(f, "acca: {:06x} accb: {:06b} accc: {:012b}\nbaka: {:06x} bakb: {:06x} bakc: {:012x}\nip: {:06x} sp: {:06x} bp: {:06x} flag: {:06x}\nmem size: {}",
            self.acca, self.accb, self.accc, self.baka, self.bakb, self.bakc, self.ip, self.sp, self.bp, self.flag, self.mem.size())
    }
}
