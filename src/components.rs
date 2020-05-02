use std::convert::TryInto;

const MAX_MEM_SIZE: u32 = 16_777_216; // 2^24

#[derive(Clone,Debug,Default)]
pub struct Word {
    val: [u8; 3],
}

impl Word {
    pub fn new() -> Self {
        Word { val: [0; 3] }
    }
}

#[derive(Debug)]
pub struct Memory {
    data: Vec<Word>,
    size: u32,
}

impl Memory {
    pub fn new(size: u32) -> Self {
        let size = if size > MAX_MEM_SIZE {
            MAX_MEM_SIZE
        } else {
            size
        };
        Memory { data: vec![Word::default(); size.try_into().unwrap()], size }
    }
}
