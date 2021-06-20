use cjemu_api::ReadableMemory;

pub struct Rom {
    max_size: u16,
    data: Vec<u8>,
}

impl Rom {
    pub fn new(default: u8, size: u16) -> Self {
        Self {
            max_size: size,
            data: vec![default; size as usize],
        }
    }
}

impl Default for Rom {
    fn default() -> Self {
        Self::new(0, u16::MAX)
    }
}

impl ReadableMemory for Rom {
    fn size(&self) -> u16 {
        self.max_size
    }

    fn byte(&self, address: u16) -> Option<u8> {
        if address >= self.max_size {
            None
        } else {
            Some(unsafe { *self.data.get_unchecked(address as usize) })
        }
    }
}
