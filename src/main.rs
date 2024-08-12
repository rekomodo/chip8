mod display;

use display::Display;
use komchip::*;
use std::{env, fs::File, io::Read};

fn main() {
    println!("CHIP8 Interpreter by @rekomodo");

    let mut argv = env::args();
    let rom_path = argv.nth(1).unwrap();
    let rom = get_rom_bytes(&rom_path).unwrap();

    let mut interpreter = chip8::Interpreter::new();
    interpreter.load_rom(&rom);

    let mut display = Display::new(chip8::DISPLAY_WIDTH, chip8::DISPLAY_HEIGHT);

    loop {
        interpreter.step();

        let mut buffer = vec![];
        for buffer_line in interpreter.display_buffer {
            for j in 0..chip8::DISPLAY_WIDTH {
                buffer.push((buffer_line >> j & 1 > 0) as u32);
            }
        }

        if interpreter.display_flag {
            display.update_display(&buffer);
        }
    }
}

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
