mod display;

use display::Display;
use komchip::*;
use std::{env, fs::File, io::Read, thread::sleep, time::Duration};

const TPS: u32 = 60; // used for delay_timer, sound_timer
const IPT: u32 = 12;
const IPS: u32 = TPS * IPT;

fn main() {
    println!("CHIP8 Interpreter by @rekomodo");

    let mut argv = env::args();
    let rom_path = argv.nth(1).unwrap();
    let rom = get_rom_bytes(&rom_path).unwrap();

    let mut interpreter = chip8::Interpreter::new();
    interpreter.load_rom(&rom);

    let mut display = Display::new(chip8::DISPLAY_WIDTH, chip8::DISPLAY_HEIGHT);

    let mut instructions_since_tick = 0;
    loop {
        interpreter.keyboard = display.get_inputs();
        interpreter.step();

        let mut buffer = vec![];
        for buffer_line in interpreter.display_buffer {
            for j in 0..chip8::DISPLAY_WIDTH {
                buffer.push((buffer_line >> j & 1 > 0) as u32);
            }
        }

        if interpreter.display_flag {
            display.update_display(&buffer);
            interpreter.display_flag = false;
        }

        if instructions_since_tick == 0 {
            interpreter.tick_timers();
        }

        if display.exit {
            break;
        }

        instructions_since_tick = (instructions_since_tick + 1) % IPT;
        sleep(Duration::from_secs_f64(1f64 / IPS as f64));
    }

    std::fs::write("debug/memory_dump.txt", format!("{:#06x?}", interpreter.ram.data)).unwrap();
}                               

// TODO: make into iterator with ReadBuffer for memory reasons
fn get_rom_bytes(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let file = File::open(path).unwrap();
    return file.bytes().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rom_bytes() {
        let bytes = get_rom_bytes("./roms/ibm_logo.ch8").unwrap();

        assert_eq!(bytes[1], 0xE0);
    }
}
