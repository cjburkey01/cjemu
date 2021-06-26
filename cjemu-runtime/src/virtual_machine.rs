use crate::{Ram, Rom};
use cjemu_api::{AluOutputs, VirtualMachine};

pub struct CJEmuVirtualMachine {
    last_alu: AluOutputs,

    reg_a: u16,
    reg_b: u16,

    rom: Rom,
    ram: Ram,
}

impl CJEmuVirtualMachine {
    pub fn new(rom_size: u16, ram_size: u16) -> Self {
        Self {
            last_alu: AluOutputs::default(),

            reg_a: 0,
            reg_b: 0,

            rom: Rom::new(0, rom_size),
            ram: Ram::new(0, ram_size),
        }
    }
}

impl VirtualMachine<Rom, Ram> for CJEmuVirtualMachine {
    type TickErrorTy = ();

    fn last_alu(&self) -> AluOutputs {
        self.last_alu
    }

    fn reg_a(&self) -> u16 {
        self.reg_a
    }

    fn reg_b(&self) -> u16 {
        self.reg_b
    }

    fn rom(&self) -> &Rom {
        &self.rom
    }

    fn ram(&self) -> &Ram {
        &self.ram
    }

    fn perform_tick(&mut self) -> Result<(), Self::TickErrorTy> {
        Ok(())
    }
}
