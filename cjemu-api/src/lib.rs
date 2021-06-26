//! The Rust version of the CJEmu API.
//!
//! ### Opcodes
//!
//! The following opcodes are available:
//!
//! | Name   | Value    | Bytes | Description                                                              |
//! |--------|----------|-------|--------------------------------------------------------------------------|
//! | nop    | 00000000 | 1     | Do nothing this cycle                                                    |
//! | lda16  | 00000001 | 3     | Load the value in the next two bytes to the `a` register                 |
//! | ldb16  | 00000010 | 3     | Load the value in the next two bytes to the `b` register                 |
//! | sta16  | 00000011 | 3     | Store the value in the `a` register to the address in the next two bytes |
//! | stb16  | 00000100 | 3     | Store the value in the `b` register to the address in the next two bytes |
//! | lda8   | 00000101 | 2     | Load the value in the next byte to the `a` register                      |
//! | ldb8   | 00000110 | 2     | Load the value in the next byte to the `b` register                      |
//! | sta8   | 00000111 | 2     | Store the value in the `a` register to the address in the next byte      |
//! | stb8   | 00001000 | 2     | Store the value in the `b` register to the address in the next byte      |
//!
//! > Note: In the table, the `Value` column represents the first byte of an
//! instruction being executed. The `Bytes` column displays how many bytes this
//! instruction will take, including the opcode's byte.

type Ty = u16;

/// The possible operations that can be performed by the emulator (on the cycle level).
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Opcode {
    /// Do nothing this cycle.
    NoOp,

    /// Load the next two bytes into the `A` register.
    LdA16,
    /// Load the next two bytes into the `B` register.
    LdB16,
    /// Store the value in the `A` register to the next two bytes.
    StA16,
    /// Store the value in the `B` register to the next two bytes.
    StB16,

    /// Load the next byte into the `A` register.
    LdA8,
    /// Load the next byte into the `B` register.
    LdB8,
    /// Store the value in the `A` register to the next byte.
    StA8,
    /// Store the value in the `B` register to the next byte.
    StB8,

    /// Add the values in the `A` and `B` registers.
    Add,
    /// Add the values in the `A` and `B` registers.
    Sub,
    /// Negate the value in the `A` register.
    NegA,
    /// Negate the value in the `B` register.
    NegB,
    /// Increment the value in the `A` register.
    IncA,
    /// Increment the value in the `B` register.
    IncB,

    /// Update the ALU outputs with the value of the `A` register.
    PassA,
    /// Update the ALU outputs with the value of the `B` register.
    PassB,

    /// Perform bitwise AND between the `A` and `B` registers.
    And,
    /// Perform bitwise OR between the `A` and `B` registers.
    Or,
    /// Perform bitwise XOR between the `A` and `B` registers.
    XOr,
    /// Find the bitwise complement of the `A` register.
    BitFlpA,
    /// Find the bitwise complement of the `B` register.
    BitFlpB,

    /// Signed shift the bits in `A` left by the value of `B`
    ShftL,
    /// Signed shift the bits in `A` right by the value of `B`
    ShftR,
    /// Unsigned shift the bits in `A` left by the value of `B`
    UShftL,
    /// Unsigned shift the bits in `A` right by the value of `B`
    UShftR,
    /// Rotate the bits in `A` left by the value of `B`
    RotL,
    /// Rotate the bits in `A` right by the value of `B`
    RotR,
}

/// Represents a container for the virtual machine's data.
pub trait VirtualMachine<Rom: ReadableMemory, Ram: ReadableMemory> {
    /// Possible errors during a tick.
    type TickErrorTy;

    /// Retrieve the last state of the ALU outputs.
    fn last_alu(&self) -> AluOutputs;

    /// Retrieve the value of the `a` register.
    fn reg_a(&self) -> u16;
    /// Retrieve the value of the `a` register.
    fn reg_b(&self) -> u16;

    /// The read-only memory available to the virtual machine.
    fn rom(&self) -> &Rom;

    /// The random access memory available to the virtual machine.
    fn ram(&self) -> &Ram;

    /// Attempts to perform a clock cycle on this virtual machine
    fn perform_tick(&mut self) -> Result<(), Self::TickErrorTy>;
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
#[derive(Copy, Clone, Debug, Default)]
pub struct AluOutputs {
    /// The value of the previous operation.
    pub value: Ty,

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
pub trait Alu {
    // Arithmetic

    /// Add `a` and `b` and return the outputs.
    fn add16(&mut self, a: Ty, b: Ty) -> AluOutputs;
    /// Add `a`, `b`, and the carry value, updating the outputs.
    fn add16_carry(&mut self, a: Ty, b: Ty, carry: bool) -> AluOutputs;
    /// Subtract `b` from `a` and return the outputs.
    fn sub16(&mut self, a: Ty, b: Ty) -> AluOutputs;
    /// Subtract `b` from `a` with the borrow value and return the outputs.
    fn sub16_borrow(&mut self, a: Ty, b: Ty, borrow: bool) -> AluOutputs;
    /// Find the two's complement of the input.
    fn neg16(&mut self, a: Ty) -> AluOutputs;
    /// Increment and return the `a` value.
    fn inc16(&mut self, a: Ty) -> AluOutputs;

    // Dummy

    /// Return the outputs as if value `a` is the result of some previous
    /// output.
    fn pass16(&mut self, a: Ty) -> AluOutputs;

    // Bit logic

    /// Perform a bitwise AND between `a` and `b` and return the outputs.
    fn and16(&mut self, a: Ty, b: Ty) -> AluOutputs;
    /// Perform a bitwise OR between `a` and `b` and return the outputs.
    fn or16(&mut self, a: Ty, b: Ty) -> AluOutputs;
    /// Perform a bitwise Exclusive OR between `a` and `b` and return the outputs.
    fn xor16(&mut self, a: Ty, b: Ty) -> AluOutputs;
    /// Perform a bit flip on the value of `a` and return the outputs.
    fn complement(&mut self, a: Ty) -> AluOutputs;

    // Bit shifting

    /// Perform an arithmetic bitshift left by `b` bits on `a` and return the
    /// outputs. The `a` value is treated as a two's complement, so the most
    /// significant bit is preserved.
    fn shift16l(&mut self, a: Ty, b: Ty) -> AluOutputs;
    /// Perform an arithmetic bitshift left by `b` bits on `a` and return the
    /// outputs.The `a` value is treated as a two's complement, so the most
    /// significant bit is preserved.
    fn shift16r(&mut self, a: Ty, b: Ty) -> AluOutputs;
    /// Perform a logical bitshift left by `b` bits on `a` and return the
    /// outputs. The `a` value is treated as an unsigned integer.
    fn ushift16l(&mut self, a: Ty, b: Ty) -> AluOutputs;
    /// Perform a logical bitshift left by `b` bits on `a` and return the
    /// outputs. The `a` value is treated as an unsigned integer.
    fn ushift16r(&mut self, a: Ty, b: Ty) -> AluOutputs;
    /// Rotate the bits left `b` times in `a`.
    fn rot16l(&mut self, a: Ty, b: Ty) -> AluOutputs;
    /// Rotate the bits right `b` times in `a`.
    fn rot16r(&mut self, a: Ty, b: Ty) -> AluOutputs;
    /// Rotate the bits left `b` times in `a` with the carry bit.
    fn rot16l_carry(&mut self, a: Ty, b: Ty, carry: bool) -> AluOutputs;
    /// Rotate the bits right `b` times in `a` with the carry bit.
    fn rot16r_carry(&mut self, a: Ty, b: Ty, carry: bool) -> AluOutputs;
}
