use std::env;
mod reader;
mod memory;

const PROGRAM_START : u16 = 0x200;
const REGISTER_COUNT : usize = 16;

const DISPLAY_WIDTH : usize = 32;
const DISPLAY_HEIGHT : usize = 64;

fn main() {
    println!("CHIP8 Interpreter by @rekomodo");

    let mut argv = env::args();
    let rom_path: String = argv.nth(1).unwrap();

    let mut memory = memory::make_memory();
    let mut display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    let mut pc : u16 = PROGRAM_START;
    let mut index_register : u16 = 0;
    let mut stack : Vec<u16>;
    let mut delay_timer : u8 = 0;
    let mut sound_timer : u8 = 0;
    let mut registers : [u8; REGISTER_COUNT] = [0; REGISTER_COUNT];

    let mut instructions = reader::InstructionBuffer::new(&rom_path);
    while let Some(inst) = instructions.next_instruction() {
        println!("{:#06X}", reader::join_nibbles(&inst));
    }
}
