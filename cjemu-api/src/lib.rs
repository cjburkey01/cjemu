//! The Rust version of the CJEmu API.
//!
//! ### Opcodes
//!
//! The following opcodes are available:
//!
//! | Name   | Value | Bytes | Description                                              |
//! |--------|-------|-------|----------------------------------------------------------|
//! | nop    | 00000 | 1     | Do nothing this cycle                                    |
//! | mva16  | 00001 | 3     | Move the value in the next two bytes to the `a` register |
//! | mvb16  | 00010 | 3     | Move the value in the next two bytes to the `b` register |
//!
//! > Note: In the table, the `Value` column represents the first 5 bits of an
//! instruction being executed. The `Bytes` column displays how many bytes this
//! instruction will take, including the opcode's byte.

use num::{PrimInt, Unsigned};

/// Represents a container for the virtual machine's data.
pub trait VirtualMachine<Rom: ReadableMemory, Ram: ReadableMemory> {
    /// The read-only memory available to the virtual machine.
    fn rom(&self) -> &Rom;

    /// The random access memory available to the virtual machine.
    fn ram(&self) -> &Ram;

    /// Attempts to perform a clock cycle on this virtual machine
    fn perform_tick(&mut self) -> Result<(), ()>;
}

/// Represents a read-only memory container.
pub trait ReadableMemory {
    /// The number of bytes this memory container may hold.
    fn size(&self) -> u16;

    /// Retrieves the byte at this address, or `None` if the address is out of
    /// this memory's bounds.
    fn byte(&self, address: u16) -> Option<u8>;
}

/// Represents a read and write memory container.
pub trait WritableMemory: ReadableMemory {
    /// Updates the byte at `address` to be `value` unless out of bounds, in
    /// which case, `None` is returned. Success yields Some(()).
    fn set_byte(&mut self, address: u16, value: u8) -> Option<()>;
}

/// A wrapper around the outputs of an ALU after performing an operation.
pub struct AluOutputs<ValueType: Unsigned + PrimInt> {
    /// The value of the previous operation.
    pub value: ValueType,

    /// Whether the previous addition operation resulted in a final carry, the
    /// previous subtraction operation resulted in a final borrow, or there was
    /// a bitshifting overflow.
    pub carry_out: bool,

    /// Whether the previous arithmetic operation is equivalent to 0.
    pub zero: bool,

    /// Whether the previous arithmetic operation resulted in a negative number.
    pub negative: bool,

    /// Whether the previous operation resulted in a value larger than can be
    /// held in the number of bytes available.
    pub overflow: bool,

    /// Whether there are an even (or odd) number of `1` bits in the previous output.
    pub parity: bool,
}

/// Represents an arithmetic logic unit.
pub trait Alu<ValueType: Unsigned + PrimInt> {
    // Arithmetic

    /// Add `a` and `b` and return the outputs.
    fn add16(&mut self, a: ValueType, b: ValueType) -> AluOutputs<ValueType>;
    /// Add `a`, `b`, and the carry value, updating the outputs.
    fn add16_carry(&mut self, a: ValueType, b: ValueType, carry: bool) -> AluOutputs<ValueType>;
    /// Subtract `b` from `a` and return the outputs.
    fn sub16(&mut self, a: ValueType, b: ValueType) -> AluOutputs<ValueType>;
    /// Subtract `b` from `a` with the borrow value and return the outputs.
    fn sub16_borrow(&mut self, a: ValueType, b: ValueType, borrow: bool) -> AluOutputs<ValueType>;
    /// Find the two's complement of the input.
    fn neg16(&mut self, a: ValueType) -> AluOutputs<ValueType>;
    /// Increment and return the `a` value.
    fn inc16(&mut self, a: ValueType) -> AluOutputs<ValueType>;

    // Dummy

    /// Return the outputs as if value `a` is the result of some previous
    /// output.
    fn pass16(&mut self, a: ValueType) -> AluOutputs<ValueType>;

    // Bit logic

    /// Perform a bitwise AND between `a` and `b` and return the outputs.
    fn and16(&mut self, a: ValueType, b: ValueType) -> AluOutputs<ValueType>;
    /// Perform a bitwise OR between `a` and `b` and return the outputs.
    fn or16(&mut self, a: ValueType, b: ValueType) -> AluOutputs<ValueType>;
    /// Perform a bitwise Exclusive OR between `a` and `b` and return the outputs.
    fn xor16(&mut self, a: ValueType, b: ValueType) -> AluOutputs<ValueType>;
    /// Perform a bit flip on the value of `a` and return the outputs.
    fn complement(&mut self, a: ValueType) -> AluOutputs<ValueType>;

    // Bit shifting

    /// Perform an arithmetic bitshift left by `b` bits on `a` and return the
    /// outputs. The `a` value is treated as a two's complement, so the most
    /// significant bit is preserved.
    fn shift16l(&mut self, a: ValueType, b: ValueType) -> AluOutputs<ValueType>;
    /// Perform an arithmetic bitshift left by `b` bits on `a` and return the
    /// outputs.The `a` value is treated as a two's complement, so the most
    /// significant bit is preserved.
    fn shift16r(&mut self, a: ValueType, b: ValueType) -> AluOutputs<ValueType>;
    /// Perform a logical bitshift left by `b` bits on `a` and return the
    /// outputs. The `a` value is treated as an unsigned integer.
    fn ushift16l(&mut self, a: ValueType, b: ValueType) -> AluOutputs<ValueType>;
    /// Perform a logical bitshift left by `b` bits on `a` and return the
    /// outputs. The `a` value is treated as an unsigned integer.
    fn ushift16r(&mut self, a: ValueType, b: ValueType) -> AluOutputs<ValueType>;
    /// Rotate the bits left `b` times in `a`.
    fn rot16l(&mut self, a: ValueType, b: ValueType) -> AluOutputs<ValueType>;
    /// Rotate the bits right `b` times in `a`.
    fn rot16r(&mut self, a: ValueType, b: ValueType) -> AluOutputs<ValueType>;
    /// Rotate the bits left `b` times in `a` with the carry bit.
    fn rot16l_carry(&mut self, a: ValueType, b: ValueType, carry: bool) -> AluOutputs<ValueType>;
    /// Rotate the bits right `b` times in `a` with the carry bit.
    fn rot16r_carry(&mut self, a: ValueType, b: ValueType, carry: bool) -> AluOutputs<ValueType>;
}
