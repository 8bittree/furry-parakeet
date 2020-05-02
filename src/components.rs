use std::convert::TryInto;
use std::fmt::{Debug, Error, Formatter};

const MAX_MEM_SIZE: u32 = 16_777_216; // 2^24

#[derive(Clone,Default)]
pub struct Word {
    val: [u8; 3],
}

impl Word {
    pub fn new() -> Self {
        Word { val: [0; 3] }
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Word")
            .field("val", &format_args!("[{}, {}, {}]", &self.val[0], &self.val[1], &self.val[2]))
            .finish()
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
