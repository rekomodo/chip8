mod display;
mod memory;
mod reader;

use display::Display;
use memory::Memory;
use std::env;

const PROGRAM_START: usize = 0x200;
const REGISTER_COUNT: usize = 16;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

fn main() {
    println!("CHIP8 Interpreter by @rekomodo");

    let mut argv = env::args();
    let rom_path: String = argv.nth(1).unwrap();

    let mut memory = Memory::new();
    let mut display = Display::new(DISPLAY_WIDTH, DISPLAY_HEIGHT);
    let mut pc: usize;
    let mut index_register: usize = 0;
    let mut stack: Vec<usize> = vec![PROGRAM_START];
    let mut delay_timer: u8 = 0;
    let mut sound_timer: u8 = 0;
    let mut registers: [u8; REGISTER_COUNT] = [0; REGISTER_COUNT];
    let mut display_flag: bool = false;

    // load program to memory
    let program = reader::get_program_bytes(&rom_path).unwrap();
    memory.set(PROGRAM_START, &program);

    // FDE Loop

    loop {
        pc = *stack.last().unwrap();
        stack.pop();

        // increments program counter
        let instr = memory.get_instruction(&mut pc);

        let nibbles = reader::get_nibbles(instr);

        let nnn = reader::join_nibbles(&nibbles[1..4]);

        let x: usize = nibbles[1] as usize;
        let nn = reader::join_nibbles(&nibbles[2..4]) as u8;

        let y = nibbles[2];
        let n = nibbles[3];

        match instr {
            0x00E0 => {
                display.clear();
                display_flag = true
            }
            _ => (),
        }

        match nibbles[0] {
            0x1 => pc = nnn as usize,             // 0x1NNN : jump to NNN
            0x6 => registers[x] = nn,             // 0x6XNN : set register VX to NN
            0x7 => registers[x] += nn,            // 0x7XNN : add NN to register VX
            0xA => index_register = nnn as usize, // 0xANNN : set index register I to NNN
            0xD => {
                display_flag = true;

                let row = registers[y as usize] as usize % DISPLAY_HEIGHT;
                let col = registers[x] as usize % DISPLAY_WIDTH;
                registers[0xF as usize] = 0;

                for i in 0..n as usize {
                    // println!("{}", memory.data[PROGRAM_START + index_register + i]);
                    for j in 0..8 {
                        let bit_state = memory.data[index_register + i]
                            .checked_shr((8 - j) as u32)
                            .unwrap_or(0)
                            & 1;

                        if (0..DISPLAY_HEIGHT).contains(&(row + i))
                            && (0..DISPLAY_WIDTH).contains(&(col + j))
                        {
                            display.xor_pixel(row + i, col + j, bit_state as u32);
                        }
                    }
                }
            } // 0xDXYN : display/draw,
            _ => (),
        }

        stack.push(pc); // push to stack after modifiers

        if display_flag {
            display.update_display();
            display_flag = false;
        }

        // timer management
        if delay_timer > 0 {
            delay_timer -= 1
        };
        if sound_timer > 0 {
            sound_timer -= 1
        };
    }
}
