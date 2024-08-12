use crate::*;

#[test]
fn test_set(){
    let mut mem = memory::Memory::new();

    let p : usize = 0x14;
    let buf = vec![0x11, 0x22, 0x33, 0x44];
    mem.set(p, &buf);

    assert_eq!(mem.data[p..p+buf.len()], buf);
}

#[test]
fn test_get_instruction(){
    let mut mem = memory::Memory::new();

    let p : usize = 0x14;
    let buf = vec![0x11, 0x22, 0x33, 0x44];
    mem.set(p, &buf);

    assert_eq!(mem.get_instruction(p), 0x1122);
    assert_eq!(mem.get_instruction(p + 2), 0x3344);
}