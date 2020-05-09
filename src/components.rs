use std::convert::TryInto;
use std::fmt::{Binary, Debug, Display, Error, Formatter, LowerHex, Octal, UpperHex};
use std::ops::Index;

/// Maximum size of a [`Memory`](struct.Memory.html) block in
/// [`Word`](struct.Word.html)s
///
/// = 2^24
const MAX_MEM_SIZE: u32 = 16_777_216;

#[derive(Clone, Copy, Default)]
pub struct Word {
    // little endian
    val: [u8; 3],
}

impl Word {
    pub fn new() -> Self {
        Word { val: [0; 3] }
    }

    pub fn set(&mut self, val: u32) {
        let bytes = val.to_le_bytes();
        self.val[0] = bytes[0];
        self.val[1] = bytes[1];
        self.val[2] = bytes[2];
    }
}

impl Binary for Word {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Binary::fmt(&u32::from(*self), f)
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Word")
            .field("val", &format_args!("[{:?}, {:?}, {:?}]",
                                        &self.val[2],
                                        &self.val[1],
                                        &self.val[0]))
            .finish()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let tmp = u32::from(*self).to_string();
        f.pad_integral(true, "", &tmp)
    }
}

impl LowerHex for Word {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        LowerHex::fmt(&u32::from(*self), f)
    }
}

impl Octal for Word {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Octal::fmt(&u32::from(*self), f)
    }
}

impl UpperHex for Word {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        UpperHex::fmt(&u32::from(*self), f)
    }
}

impl From<Word> for u32 {
    fn from(word: Word) -> u32 {
        word.val[0] as u32
            + ((word.val[1] as u32) << 8)
            + ((word.val[2] as u32) << 16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_word() {
        let mut w = Word::new();
        assert_eq!(0, u32::from(w));
        w.set(0x5a);
        assert_eq!(0x5a, u32::from(w));
    }
}

#[derive(Debug)]
pub struct Memory {
    data: Vec<Word>,
    size: u32,
}

impl Memory {
    /// Creates a new `Memory` block of the given `size`.
    ///
    /// The maximum size of a `Memory` block is
    /// [`MAX_MEM_SIZE`](constant.MAX_MEM_SIZE.html). If `size` is greater
    /// than that, the created `Memory` block will be
    /// [`MAX_MEM_SIZE`](constant.MAX_MEM_SIZE.html) long.
    pub fn new(size: u32) -> Self {
        let size = if size > MAX_MEM_SIZE {
            MAX_MEM_SIZE
        } else {
            size
        };
        Memory { data: vec![Word::default(); size.try_into().unwrap()], size }
    }
}

impl Index<usize> for Memory {
    type Output = Word;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
