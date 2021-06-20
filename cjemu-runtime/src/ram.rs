use cjemu_api::{ReadableMemory, WritableMemory};

pub struct Ram {
    max_size: u16,
    data: Vec<u8>,
}

impl Ram {
    pub fn new(default: u8, size: u16) -> Self {
        Self {
            max_size: size,
            data: vec![default; size as usize],
        }
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self::new(0, u16::MAX)
    }
}

impl ReadableMemory for Ram {
    fn size(&self) -> u16 {
        self.max_size
    }

    fn byte(&self, address: u16) -> Option<u8> {
        if address >= self.max_size {
            None
        } else {
            Some(*unsafe { self.data.get_unchecked(address as usize) })
        }
    }
}

impl WritableMemory for Ram {
    fn set_byte(&mut self, address: u16, value: u8) -> Option<()> {
        if address >= self.max_size {
            None
        } else {
            *unsafe { self.data.get_unchecked_mut(address as usize) } = value;

            Some(())
        }
    }
}
