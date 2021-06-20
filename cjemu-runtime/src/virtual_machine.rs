use crate::{Ram, Rom};
use cjemu_api::VirtualMachine;

pub struct CJEmuVirtualMachine {
    rom: Rom,
    ram: Ram,
}

impl CJEmuVirtualMachine {
    pub fn new(rom_size: u16, ram_size: u16) -> Self {
        Self {
            rom: Rom::new(0, rom_size),
            ram: Ram::new(0, ram_size),
        }
    }
}

impl VirtualMachine<Rom, Ram> for CJEmuVirtualMachine {
    fn rom(&self) -> &Rom {
        &self.rom
    }

    fn ram(&self) -> &Ram {
        &self.ram
    }

    fn perform_tick(&mut self) -> Result<(), ()> {
        //println!("performing tick");

        Ok(())
    }
}
