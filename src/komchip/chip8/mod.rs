use crate::memory;
use crate::nibbles;
use rand::prelude::*; // TODO: Clean imports.

const PROGRAM_START: usize = 0x200;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;
const REGISTER_COUNT: usize = 16;

pub struct Interpreter {
    pub ram: memory::Memory,
    pub display_buffer: [u64; DISPLAY_HEIGHT],
    index_register: usize,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; REGISTER_COUNT],
    pub display_flag: bool,
    pub keyboard: u16,
    rng: StdRng,

    stack: Box<Vec<usize>>, // TODO: make fixed size stack
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            ram: memory::Memory::new(),
            display_buffer: [0u64; DISPLAY_HEIGHT],
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; REGISTER_COUNT],
            display_flag: false,
            rng: StdRng::from_entropy(), // TODO: allow seeding
            keyboard: 0,

            stack: Box::new(vec![PROGRAM_START]),
        }
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1
        };
        if self.sound_timer > 0 {
            self.sound_timer -= 1
        };
    }

    pub fn load_rom(&mut self, bytes: &[u8]) {
        self.ram.set(PROGRAM_START, bytes);
    }

    pub fn step(&mut self) {
        let pc = *self.stack.last().unwrap();
        self.process_instruction(self.ram.get_instruction(pc));
    }

    pub fn process_instruction(&mut self, instr: u16) {
        let mut pc = *self.stack.last().unwrap() + 2;
        self.stack.pop();

        let nibbles = nibbles::get_nibbles(instr);
        let nnn = nibbles::join_nibbles(&nibbles[1..4]);

        let x = nibbles[1] as usize;
        let nn = nibbles::join_nibbles(&nibbles[2..4]) as u8;

        let y = nibbles[2] as usize;
        let n = nibbles[3];

        match nibbles[0] {
            0x0 => match nn {
                0xE0 => {
                    self.display_buffer.fill(0u64);
                    self.display_flag = true;
                }
                0xEE => pc = self.stack.pop().expect("Empty stack on 0xEE instruction."),
                _ => panic!("No match on ??={nn:#04x} for 0x00?? instruction."),
            },
            0x1 => pc = nnn as usize,
            0x2 => {
                self.stack.push(pc);
                pc = nnn as usize;
            }
            0x3 => {
                if self.registers[x] == nn {
                    pc += 2
                }
            }
            0x4 => {
                if self.registers[x] != nn {
                    pc += 2
                }
            }
            0x5 => {
                if self.registers[x] == self.registers[y] {
                    pc += 2
                }
            }
            0x6 => self.registers[x] = nn,
            0x7 => self.registers[x] = self.registers[x].overflowing_add(nn).0,
            0x8 => {
                match n {
                    0 => self.registers[x] = self.registers[y],
                    1 => self.registers[x] |= self.registers[y],
                    2 => self.registers[x] &= self.registers[y],
                    3 => self.registers[x] ^= self.registers[y],
                    4 => {
                        let (new_v, overflow) =
                            self.registers[x].overflowing_add(self.registers[y]);
                        self.registers[x] = new_v;
                        self.registers[0xF] = overflow as u8;
                    }
                    5 => {
                        let (new_v, underflow) =
                            self.registers[x].overflowing_sub(self.registers[y]);
                        self.registers[x] = new_v;
                        self.registers[0xF] = (!underflow) as u8;
                    }
                    6 => {
                        // self.registers[x] = self.registers[y]; // TODO: OPTIONAL, MAKE INTO CFG
                        let shift_out = self.registers[x] & 0x1;
                        self.registers[x] >>= 1;

                        self.registers[0xF] = shift_out;
                    }
                    7 => {
                        let (new_v, underflow) =
                            self.registers[y].overflowing_sub(self.registers[x]);
                        self.registers[x] = new_v;
                        self.registers[0xF] = (!underflow) as u8;
                    }
                    0xE => {
                        // self.registers[x] = self.registers[y]; // TODO: OPTIONAL, MAKE INTO CFG
                        let shift_out = self.registers[x] & 0b10000000;
                        self.registers[x] <<= 1;

                        self.registers[0xF] = shift_out;
                    }
                    _ => panic!("No match on ?={n:#03x} for 0x8XY? opcode."),
                }
            }
            0x9 => {
                if self.registers[x] != self.registers[y] {
                    pc += 2
                }
            }
            0xA => self.index_register = nnn as usize,
            0xB => pc = (nnn + self.registers[0x0] as u16) as usize, // TODO: OPTIONAL BXNN, MAKE INTO CFG
            0xC => self.registers[x] = self.rng.gen::<u8>() & nn,
            0xD => {
                self.display_flag = true;

                let row = self.registers[y] as usize % DISPLAY_HEIGHT;
                let col = self.registers[x] as usize % DISPLAY_WIDTH;
                self.registers[0xF as usize] = 0;

                for i in 0..n as usize {
                    let sprite_bits = self.ram.data[self.index_register + i].reverse_bits();
                    let aligned_sprite_bits = (sprite_bits as u64) << col;

                    // check for overwrite
                    if self.display_buffer[row + i] & aligned_sprite_bits > 0 {
                        self.registers[0xF] = 1;
                    }

                    self.display_buffer[row + i] ^= aligned_sprite_bits;
                }
            }
            0xE => match nn {
                _ => todo!("Instruction 0xEXNN not implemented! Waiting on keyboard impl."),
            },
            0xF => match nn {
                0x07 => self.registers[x] = self.delay_timer,
                0x15 => self.delay_timer = self.registers[x],
                0x18 => self.sound_timer = self.registers[x],
                0x1E => {
                    let overflow;
                    (self.index_register, overflow) = self.index_register.overflowing_add(x);
                    self.registers[0xF] = overflow as u8;
                }
                0x0A => todo!("Instruction FX0A not implemented! Waiting on keyboard impl."),
                0x29 => {
                    self.index_register =
                        memory::FONTS_START + self.registers[x] as usize * memory::FONT_HEIGHT
                }
                0x33 => {
                    let mut num = self.registers[x];

                    let mut digits = vec![];
                    while num > 0 {
                        digits.push(num % 10);
                        num /= 10;
                    }
                    digits.reverse();

                    self.ram.set(self.index_register, &digits);
                }
                0x55 => {
                    for i in 0..=x {
                        self.ram.data[self.index_register + i] = self.registers[i];
                    }
                }
                0x65 => {
                    for i in 0..=x {
                        self.registers[i] = self.ram.data[self.index_register + i];
                    }
                }
                _ => panic!("No match on ??={nn:#04x} for 0xFX?? instruction."),
            },
            _ => panic!("No instructions match {instr:#06x}"),
        }

        self.stack.push(pc); // push to stack after modifiers
    }
}

// TODO: move implementation of instruction set into another impl block (perhaps on another module)
