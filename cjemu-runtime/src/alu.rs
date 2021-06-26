use cjemu_api::{Alu, AluOutputs};

pub struct CJEmuAlu {}

#[allow(unused_variables)]
impl Alu for CJEmuAlu {
    fn add16(&mut self, a: u16, b: u16) -> AluOutputs {
        let (value, overflow) = match a.checked_add(b) {
            Some(val) => (val, false),
            // Wrap back around in the event of an overflow
            None => (b - (u16::MAX - a), true),
        };

        AluOutputs {
            value,
            carry_out: false, // TODO
            zero: value == 0,
            negative: false,
            overflow,
            parity: false, // TODO
        }
    }

    fn add16_carry(&mut self, a: u16, b: u16, carry: bool) -> AluOutputs {
        todo!()
    }

    fn sub16(&mut self, a: u16, b: u16) -> AluOutputs {
        todo!()
    }

    fn sub16_borrow(&mut self, a: u16, b: u16, borrow: bool) -> AluOutputs {
        todo!()
    }

    fn neg16(&mut self, a: u16) -> AluOutputs {
        todo!()
    }

    fn inc16(&mut self, a: u16) -> AluOutputs {
        todo!()
    }

    fn pass16(&mut self, a: u16) -> AluOutputs {
        todo!()
    }

    fn and16(&mut self, a: u16, b: u16) -> AluOutputs {
        todo!()
    }

    fn or16(&mut self, a: u16, b: u16) -> AluOutputs {
        todo!()
    }

    fn xor16(&mut self, a: u16, b: u16) -> AluOutputs {
        todo!()
    }

    fn complement(&mut self, a: u16) -> AluOutputs {
        todo!()
    }

    fn shift16l(&mut self, a: u16, b: u16) -> AluOutputs {
        todo!()
    }

    fn shift16r(&mut self, a: u16, b: u16) -> AluOutputs {
        todo!()
    }

    fn ushift16l(&mut self, a: u16, b: u16) -> AluOutputs {
        todo!()
    }

    fn ushift16r(&mut self, a: u16, b: u16) -> AluOutputs {
        todo!()
    }

    fn rot16l(&mut self, a: u16, b: u16) -> AluOutputs {
        todo!()
    }

    fn rot16r(&mut self, a: u16, b: u16) -> AluOutputs {
        todo!()
    }

    fn rot16l_carry(&mut self, a: u16, b: u16, carry: bool) -> AluOutputs {
        todo!()
    }

    fn rot16r_carry(&mut self, a: u16, b: u16, carry: bool) -> AluOutputs {
        todo!()
    }
}
