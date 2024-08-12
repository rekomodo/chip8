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