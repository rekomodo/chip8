use std::fs::File;
use std::io::BufReader;
use std::io::Read;
pub struct InstructionBuffer {
    bytes_iterator: Box<dyn Iterator<Item = Result<u8, std::io::Error>>>,
}

pub fn get_nibbles(byte : u8) -> (u8, u8) {
    (byte >> 4, byte & 0x0F)
}

pub fn join_nibbles(nibbles : &[u8]) -> u16 {
    // use u32 to avoid overflow when shifting extra 4
    let mut res : u32 = 0;

    // nibbles are four bites long
    for nibb in nibbles{
        res |= *nibb as u32;
        res <<= 4; // extra 4 shift on last iteration
    }
    res >>= 4; // undo extra shift

    res as u16
}

impl InstructionBuffer {
    pub fn new(path: &str) -> InstructionBuffer {
        let file = File::open(path).unwrap();
        let bytes_iterator = Box::new(BufReader::new(file).bytes());

        InstructionBuffer { bytes_iterator }
    }

    ///
    /// Returns the next two bytes in the buffer.
    /// Returns None when either byte is missing.
    /// 
    pub fn next_instruction(&mut self) -> Option<[u8;4]> {
        let byte1: u8;
        let byte2: u8;

        match self.bytes_iterator.next() {
            Some(x) => byte1 = x.unwrap(),
            None => return None,
        }

        match self.bytes_iterator.next() {
            Some(x) => byte2 = x.unwrap(),
            None => return None,
        }

        let (nb0, nb1) = get_nibbles(byte1);
        let (nb2, nb3) = get_nibbles(byte2);

        Some([nb0, nb1, nb2, nb3])
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
    fn test_join_niblbes(){
        assert_eq!(join_nibbles(&[0xE, 0xF]), 0x00EF);
    }

    #[test]
    fn test_instruction_buffer(){
        let mut bytes = InstructionBuffer::new("./roms/ibm_logo.ch8");
        let instr = bytes.next_instruction().unwrap();

        assert_eq!(join_nibbles(&instr[0..4]), 0x00EF);
    }
}