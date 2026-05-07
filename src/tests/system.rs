use crate::prelude::disassemble;

#[test]
fn test_decode_svc() {
    let bytes = vec![0x80, 0x00, 0x00, 0xef]; // svc #0x80
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], "svc #0x80");
}

#[test]
fn test_unknown_instruction() {
    let bytes = vec![0xef, 0xbe, 0xad, 0xde]; // .word 0xDEADBEEF (unknown instruction)
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], ".word 0xdeadbeef");
}
