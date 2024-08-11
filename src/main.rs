use std::env;
mod reader;
mod memory;
use memory::Memory;

const PROGRAM_START : usize = 0x200;
const REGISTER_COUNT : usize = 16;

const DISPLAY_WIDTH : usize = 32;
const DISPLAY_HEIGHT : usize = 64;

fn main() {
    println!("CHIP8 Interpreter by @rekomodo");

    let mut argv = env::args();
    let rom_path: String = argv.nth(1).unwrap();

    let mut memory = Memory::new();
    let mut display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    let mut pc : usize = PROGRAM_START;
    let mut index_register : u16 = 0;
    let mut stack : Vec<u16>;
    let mut delay_timer : u8 = 0;
    let mut sound_timer : u8 = 0;
    let mut registers : [u8; REGISTER_COUNT] = [0; REGISTER_COUNT];

    // load program to memory
    let program = reader::get_program_bytes(&rom_path).unwrap();
    memory.set(PROGRAM_START, &program);
}
