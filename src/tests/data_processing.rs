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

#[test]
fn test_decode_mla() {
    let bytes = vec![0x91, 0x32, 0x20, 0xe0]; // mla r0, r1, r2, r3
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], "mla r0, r1, r2, r3");
}

#[test]
fn test_decode_mls() {
    let bytes = vec![0x91, 0x32, 0x60, 0xe0]; // mls r0, r1, r2, r3
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], "mls r0, r1, r2, r3");
}

#[test]
fn test_decode_sdiv() {
    let bytes = vec![0x11, 0xf2, 0x10, 0xe7]; // sdiv r0, r1, r2
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], "sdiv r0, r1, r2");
}

#[test]
fn test_decode_udiv() {
    let bytes = vec![0x11, 0xf2, 0x30, 0xe7]; // udiv r0, r1, r2
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], "udiv r0, r1, r2");
}

#[test]
fn test_movw_movt() {
    // movw r0, #0x5678 (E3050678)
    let bytes = vec![0x78, 0x06, 0x05, 0xe3];
    let insts = disassemble(&bytes).unwrap();
    assert_eq!(insts[0], "movw r0, #0x5678");

    // movt r0, #0x1234 (E3410234)
    let bytes = vec![0x34, 0x02, 0x41, 0xe3];
    let insts = disassemble(&bytes).unwrap();
    assert_eq!(insts[0], "movt r0, #0x1234");
}

#[test]
fn test_ldm_stm() {
    // ldmia r0!, {r1, r2} (E8B00006)
    let bytes = vec![0x06, 0x00, 0xb0, 0xe8];
    let insts = disassemble(&bytes).unwrap();
    assert_eq!(insts[0], "ldmia r0!, {r1, r2}");

    // stmdb r0!, {r1, r2} (E9200006)
    let bytes = vec![0x06, 0x00, 0x20, 0xe9];
    let insts = disassemble(&bytes).unwrap();
    assert_eq!(insts[0], "stmdb r0!, {r1, r2}");
}

#[test]
fn test_extend() {
    // sxtb r0, r1 (E6AF0071)
    let bytes = vec![0x71, 0x00, 0xaf, 0xe6];
    let insts = disassemble(&bytes).unwrap();
    assert_eq!(insts[0], "sxtb r0, r1");

    // uxth r0, r1, ror #8 (E6FF0471)
    let bytes = vec![0x71, 0x04, 0xff, 0xe6];
    let insts = disassemble(&bytes).unwrap();
    assert_eq!(insts[0], "uxth r0, r1, ror #8");
}

#[test]
fn test_reverse() {
    // rev r0, r1 (E6BF0F31)
    let bytes = vec![0x31, 0x0f, 0xbf, 0xe6];
    let insts = disassemble(&bytes).unwrap();
    assert_eq!(insts[0], "rev r0, r1");

    // rev16 r0, r1 (E6BF0FB1)
    let bytes = vec![0xb1, 0x0f, 0xbf, 0xe6];
    let insts = disassemble(&bytes).unwrap();
    assert_eq!(insts[0], "rev16 r0, r1");

    // revsh r0, r1 (E6FF0FB1)
    let bytes = vec![0xb1, 0x0f, 0xff, 0xe6];
    let insts = disassemble(&bytes).unwrap();
    assert_eq!(insts[0], "revsh r0, r1");
}
