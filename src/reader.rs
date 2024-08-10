use std::fs::File;
use std::io::BufReader;
use std::io::Read;
struct InstructionBuffer {
    bytes_iterator: Box<dyn Iterator<Item = Result<u8, std::io::Error>>>,
}

fn get_nibbles(byte : u8) -> (u8, u8) {
    (byte >> 4, byte & 0x0F)
}

impl InstructionBuffer {
    fn new(path: &str) -> InstructionBuffer {
        let file = File::open(path).unwrap();
        let bytes_iterator = Box::new(BufReader::new(file).bytes());

        InstructionBuffer { bytes_iterator }
    }

    fn next_instruction(&mut self) -> Option<(u8, u8)> {
        let byte1: u8;
        let byte2: u8;

        match self.bytes_iterator.next() {
            Some(x) => byte1 = x.unwrap(),
            None => return None,
        }

        match self.bytes_iterator.next() {
            Some(x) => byte2 = x.unwrap(),
            None => panic!("Did not find second byte. Improper ROM?"),
        }

        Some((byte1, byte2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nibbles(){
        assert_eq!((0x1, 0xE), get_nibbles(0x1E));
    }

    #[test]
    fn test_instruction_buffer(){
        let mut bytes = InstructionBuffer::new("./roms/ibm_logo.ch8");
        let (byte1, byte2) = bytes.next_instruction().unwrap();

        assert_eq!((byte1 << 4) | byte2, 0x00EF);
    }
}