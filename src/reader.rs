use std::fs::File;
use std::io::Read;

pub fn get_nibbles(instr : u16) -> [u16;4] {
    [
        (instr & 0xF000) >> 12,
        (instr & 0x0F00) >> 8,
        (instr & 0x00F0) >> 4,
        (instr & 0x000F)
    ]
}

pub fn join_nibbles(nibbles : &[u16]) -> u16 {
    let mut res = nibbles[0];

    for nibble in &nibbles[1..]{
        res <<= 4;
        res += nibble;
    }

    res
}

pub fn get_program_bytes(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let file = File::open(path).unwrap();
    return file.bytes().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nibbles(){
        assert_eq!([0x1, 0xE, 0xF, 0xB], get_nibbles(0x1EFB));
    }

    #[test]
    fn test_join_nibbles(){
        assert_eq!(0x1EFB, join_nibbles(&[0x1, 0xE, 0xF, 0xB]));
    }

    #[test]
    fn test_get_program_bytes(){
        let bytes = get_program_bytes("./roms/ibm_logo.ch8").unwrap();

        assert_eq!(bytes[1], 0xE0);
    }
}