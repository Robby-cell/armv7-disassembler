use crate::prelude::disassemble;

#[test]
fn test_decode_mov_immediate() {
    let bytes = vec![0x56, 0x00, 0xa0, 0xe3]; // mov r0, #0x56
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], "mov r0, #0x56");
}

#[test]
fn test_decode_add_register() {
    let bytes = vec![0x03, 0x10, 0x82, 0xe0]; // add r1, r2, r3
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], "add r1, r2, r3");
}

#[test]
fn test_decode_adds_register() {
    let bytes_s = vec![0x02, 0x00, 0x91, 0xe0]; // adds r0, r1, r2
    let strings_s = disassemble(&bytes_s).unwrap();
    assert_eq!(strings_s[0], "adds r0, r1, r2");
}

#[test]
fn test_decode_mul() {
    let bytes = vec![0x91, 0x02, 0x00, 0xe0]; // mul r0, r1, r2
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], "mul r0, r1, r2");
}
