use crate::*;

#[test]
fn test_get_nibbles(){
    assert_eq!([0x1, 0xE, 0xF, 0xB], nibbles::get_nibbles(0x1EFB));
}

#[test]
fn test_join_nibbles(){
    assert_eq!(0x1EFB, nibbles::join_nibbles(&[0x1, 0xE, 0xF, 0xB]));
}