use std::env;
mod reader;

fn main() {
    println!("CHIP8 Interpreter by @rekomodo");

    let mut argv = env::args();
    let rom_path: String = argv.nth(1).unwrap();

    println!("{rom_path}");
}