use crate::prelude::disassemble;

#[test]
fn test_decode_svc() {
    let bytes = vec![0x80, 0x00, 0x00, 0xef]; // svc #0x80
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], "svc #0x80");
}

#[test]
fn test_decode_hints() {
    let bytes = vec![
        0x00, 0xf0, 0x20, 0xe3, // nop
        0x01, 0xf0, 0x20, 0xe3, // yield
        0x02, 0xf0, 0x20, 0xe3, // wfe
        0x03, 0xf0, 0x20, 0xe3, // wfi
        0x04, 0xf0, 0x20, 0xe3, // sev
    ];
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], "nop");
    assert_eq!(strings[1], "yield");
    assert_eq!(strings[2], "wfe");
    assert_eq!(strings[3], "wfi");
    assert_eq!(strings[4], "sev");
}

#[test]
fn test_decode_bkpt() {
    // bkpt #0xab
    let bytes = vec![0x7b, 0x0a, 0x20, 0xe1];
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], "bkpt #0xab");
}

#[test]
fn test_unknown_instruction() {
    let bytes = vec![0xef, 0xbe, 0xad, 0xde]; // .word 0xDEADBEEF (unknown instruction)
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], ".word 0xdeadbeef");
}
