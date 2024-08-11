use std::env;
mod reader;
mod memory;
use memory::Memory;
use reader::join_nibbles;

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
    let mut pc : usize;
    let mut index_register : u16 = 0;
    let mut stack : Vec<usize> = vec![PROGRAM_START];
    let mut delay_timer : u8 = 0;
    let mut sound_timer : u8 = 0;
    let mut registers : [u8; REGISTER_COUNT] = [0; REGISTER_COUNT];

    // load program to memory
    let program = reader::get_program_bytes(&rom_path).unwrap();
    memory.set(PROGRAM_START, &program);

    // FDE Loop

    loop {
        // increments program counter
        pc = *stack.last().unwrap();
        stack.pop();
        let instr = memory.get_instruction(&mut pc);

        let nibbles = reader::get_nibbles(instr);

        let nnn = join_nibbles(&nibbles[1..4]);

        let x : usize = nibbles[1] as usize;
        let nn = join_nibbles(&nibbles[2..4]) as u8;

        let y = nibbles[2];
        let n = nibbles[3];

        match instr {
            0x00E0 => display.fill([false; 32]),
            _ => ()
        }

        match nibbles[0] {
            0x1 => pc = nnn as usize, // 0x1NNN : jump to NNN
            0x6 => registers[x] = nn, // 0x6XNN : set register VX to NN
            0x7 => registers[x] += nn, // 0x7XNN : add NN to register VX
            0xA => index_register = nnn, // 0xANNN : set index register I to NNN
            0xD => (), // 0xDXYN : display/draw,
            _ => ()
        }

        stack.push(pc);
    }
}