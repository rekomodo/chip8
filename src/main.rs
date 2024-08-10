use std::env;
mod reader;

fn main() {
    println!("CHIP8 Interpreter by @rekomodo");

    let mut argv = env::args();
    let rom_path: String = argv.nth(1).unwrap();

    let mut instructions = reader::InstructionBuffer::new(&rom_path);

    while let Some(inst) = instructions.next_instruction() {
        println!("{:#06X}", reader::join_nibbles(&inst));
    }
}
