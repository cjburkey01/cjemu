/// A container for the virtual machine's data.
pub trait VirtualMachine<Rom: ReadableMemory, Ram: ReadableMemory> {
    /// The read-only memory available to the virtual machine.
    fn rom(&self) -> &Rom;

    /// The random access memory available to the virtual machine.
    fn ram(&self) -> &Ram;

    /// Attempts to perform a clock cycle on this virtual machine
    fn perform_tick(&mut self) -> Result<(), ()>;
}

/// A read-only memory container.
pub trait ReadableMemory {
    /// The number of bytes this memory container may hold.
    fn size(&self) -> u16;

    /// Retrieves the byte at this address, or `None` if the address is out of
    /// this memory's bounds.
    fn byte(&self, address: u16) -> Option<u8>;
}

/// A read and write memory container.
pub trait WritableMemory: ReadableMemory {
    /// Updates the byte at `address` to be `value` unless out of bounds, in
    /// which case, `None` is returned. Success yields Some(()).
    fn set_byte(&mut self, address: u16, value: u8) -> Option<()>;
}
