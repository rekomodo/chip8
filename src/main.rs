mod display;
mod memory;
mod reader;

use core::panic;
use display::Display;
use memory::*;
use rand::{prelude::*, seq::index};
use std::env;

const PROGRAM_START: usize = 0x200;
const REGISTER_COUNT: usize = 16;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

fn main() {
    println!("CHIP8 Interpreter by @rekomodo");

    let mut argv = env::args();
    let rom_path: String = argv.nth(1).unwrap();

    let mut mem = Memory::new();
    let mut display = Display::new(DISPLAY_WIDTH, DISPLAY_HEIGHT);
    let mut pc: usize;
    let mut index_register: usize = 0;
    let mut stack: Vec<usize> = vec![PROGRAM_START];
    let mut delay_timer: u8 = 0;
    let mut sound_timer: u8 = 0;
    let mut registers: [u8; REGISTER_COUNT] = [0; REGISTER_COUNT];
    let mut display_flag: bool = false;

    // randomness setup
    let mut rng = rand::thread_rng();

    // load program to mem
    let program = reader::get_program_bytes(&rom_path).unwrap();
    mem.set(PROGRAM_START, &program);

    // FDE Loop

    loop {
        pc = *stack.last().unwrap();
        stack.pop();

        // increments program counter
        let instr = mem.get_instruction(&mut pc);

        let nibbles = reader::get_nibbles(instr);

        let nnn = reader::join_nibbles(&nibbles[1..4]);

        let x: usize = nibbles[1] as usize;
        let nn = reader::join_nibbles(&nibbles[2..4]) as u8;

        let y = nibbles[2] as usize;
        let n = nibbles[3];

        match instr {
            _ => (),
        }

        match nibbles[0] {
            0x0 => match nn {
                0xE0 => {
                    display.clear();
                    display_flag = true
                }
                0xEE => pc = stack.pop().unwrap(),
                _ => panic!("No match on ??={nn:#04x} for 0x00?? opcode."),
            },
            0x1 => pc = nnn as usize,
            0x2 => {
                stack.push(pc);
                pc = nnn as usize;
            }
            0x3 => {
                if registers[x] == nn {
                    pc += 2
                }
            }
            0x4 => {
                if registers[x] != nn {
                    pc += 2
                }
            }
            0x5 => {
                if registers[x] == registers[y] {
                    pc += 2
                }
            }
            0x6 => registers[x] = nn,
            0x7 => registers[x] = registers[x].overflowing_add(nn).0,
            0x8 => {
                match n {
                    0 => registers[x] = registers[y],
                    1 => registers[x] |= registers[y],
                    2 => registers[x] &= registers[y],
                    3 => registers[x] ^= registers[y],
                    4 => {
                        let (new_v, overflow) = registers[x].overflowing_add(registers[y]);
                        registers[x] = new_v;
                        registers[0xF as usize] = overflow as u8;
                    }
                    5 => {
                        let (new_v, underflow) = registers[x].overflowing_sub(registers[y]);
                        registers[x] = new_v;
                        registers[0xF as usize] = (!underflow) as u8;
                    }
                    6 => {
                        // registers[x] = registers[y]; // TODO: OPTIONAL, MAKE INTO CFG
                        let shift_out = registers[x] & 0x1;
                        registers[x] >>= 1;

                        registers[0xF as usize] = shift_out;
                    }
                    7 => {
                        let (new_v, underflow) = registers[y].overflowing_sub(registers[x]);
                        registers[x] = new_v;
                        registers[0xF as usize] = (!underflow) as u8;
                    }
                    0xE => {
                        // registers[x] = registers[y]; // TODO: OPTIONAL, MAKE INTO CFG
                        let shift_out = registers[x] & 0b10000000;
                        registers[x] <<= 1;

                        registers[0xF as usize] = shift_out;
                    }
                    _ => panic!("No match on ?={n:#03x} for 0x8XY? opcode."),
                }
            }
            0x9 => {
                if registers[x] != registers[y] {
                    pc += 2
                }
            }
            0xA => index_register = nnn as usize,
            0xB => pc = (nnn + registers[0x0] as u16) as usize, // TODO: OPTIONAL BXNN, MAKE INTO CFG
            0xC => registers[x] = rng.gen::<u8>() & nn,
            0xD => {
                display_flag = true;

                let row = registers[y as usize] as usize % DISPLAY_HEIGHT;
                let col = registers[x] as usize % DISPLAY_WIDTH;
                registers[0xF as usize] = 0;

                for i in 0..n as usize {
                    // println!("{}", mem.data[PROGRAM_START + index_register + i]);
                    for j in 0..8 {
                        let bit_state = mem.data[index_register + i]
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
            }
            0xE => match nn {
                _ => todo!("opcodes 0xEXNN not implemented! Waiting on keyboard impl."),
            },
            0xF => match nn {
                0x07 => registers[x] = delay_timer,
                0x15 => delay_timer = registers[x],
                0x18 => sound_timer = registers[x],
                0x1E => {
                    let overflow;
                    (index_register, overflow) = index_register.overflowing_add(x);
                    registers[0xF] = overflow as u8;
                }
                0x0A => todo!("Instruction FX0A not implemented! Waiting on keyboard impl."),
                0x29 => {
                    index_register =
                        memory::FONTS_START + registers[x] as usize * memory::FONT_HEIGHT
                }
                0x33 => {
                    let mut num = registers[x];

                    let mut digits = vec![];
                    while num > 0 {
                        digits.push(num % 10);
                        num /= 10;
                    }
                    digits.reverse();

                    mem.data[index_register..index_register + digits.len()]
                        .copy_from_slice(&digits);
                }
                0x55 => {
                    for i in 0..x {
                        mem.data[index_register + i] = registers[i];
                    }
                },
                0x65 => {
                    for i in 0..x {
                        registers[i] = mem.data[index_register + i] ;
                    }
                }
                _ => panic!("No match on ??={nn:#04x} for 0xFX?? opcode."),
            },
            _ => (),
        }

        stack.push(pc); // push to stack after modifiers

        if display_flag {
            display.update_display();
            display_flag = false;
        }

        if delay_timer > 0 {
            todo!("Proper timing not yet implemented.")
        };
        if sound_timer > 0 {
            todo!("Proper timing not yet implemented.")
        };
    }
}
