mod memory;
mod nibbles;
pub mod chip8;

#[cfg(test)]
mod tests;

trait Instructions {
    fn op_0x00e0();
    // TODO: add other traits and impl
}