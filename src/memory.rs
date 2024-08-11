const FONT_HEIGHT: usize = 5;
const FONT_COUNT: usize = 16;

const FONTS: [u8; FONT_HEIGHT * FONT_COUNT] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const MEMORY_SIZE: usize = 4096;
const FONTS_START: usize = 0x050;

pub struct Memory {
    data : [u8; MEMORY_SIZE]
}

impl Memory {
    pub fn new() -> Memory {
        let mut data: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    
        // copy fonts to memory
        let data_slice = &mut data[FONTS_START..FONTS_START + FONTS.len()];
        let fonts_slice = &FONTS[0..FONTS.len()];
        data_slice.copy_from_slice(fonts_slice);

        Memory { data }
    }

    pub fn set(&mut self, p : usize, buff : &Vec<u8>) {
        assert!((0..MEMORY_SIZE).contains(&p));
    
        let memory_slice = &mut self.data[p..p + buff.len()];
        memory_slice.copy_from_slice(buff);
    }

    pub fn get_instruction(&self, p : usize) -> u16 {
        assert!((0..MEMORY_SIZE).contains(&p));
        assert!((0..MEMORY_SIZE).contains(&(p+1)));

        let byte_left : u16 = self.data[p] as u16;
        let byte_right : u16 = self.data[p + 1] as u16;

        (byte_left << 8) + byte_right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set(){
        let mut mem = Memory::new();

        let p : usize = 0x14;
        let buf = vec![0x11, 0x22, 0x33, 0x44];
        mem.set(p, &buf);

        assert_eq!(mem.data[p..p+buf.len()], buf);
    }

    #[test]
    fn test_get_instruction(){
        let mut mem = Memory::new();

        let p : usize = 0x14;
        let buf = vec![0x11, 0x22, 0x33, 0x44];
        mem.set(p, &buf);

        assert_eq!(mem.get_instruction(p), 0x1122);
        assert_eq!(mem.get_instruction(p + 2), 0x3344);
    }
}