use crate::prelude::disassemble;

#[test]
fn test_decode_memory_positive_offset() {
    let bytes = vec![0x04, 0x00, 0x91, 0xe5]; // ldr r0,[r1, #4]
    let strings = disassemble(&bytes).unwrap();
    assert_eq!(strings[0], "ldr r0, [r1, #4]");
}

#[test]
fn test_decode_memory_negative_offset() {
    let bytes_sub = vec![0x04, 0x00, 0x11, 0xe5]; // ldr r0, [r1, #-4]
    let strings_sub = disassemble(&bytes_sub).unwrap();
    // Fixed: Assertion correctly expects #-4 instead of #-0x4 for numbers <= 9
    assert_eq!(strings_sub[0], "ldr r0, [r1, #-4]");
}

#[test]
fn test_decode_push_pop_single() {
    let push_lr = vec![0x04, 0xe0, 0x2d, 0xe5]; // PUSH {LR} optimized as STR LR, [SP, #-4]!
    let strings = disassemble(&push_lr).unwrap();
    assert_eq!(strings[0], "push {lr}");

    let pop_pc = vec![0x04, 0xf0, 0x9d, 0xe4]; // POP {PC} optimized as LDR PC,[SP], #4
    let strings = disassemble(&pop_pc).unwrap();
    assert_eq!(strings[0], "pop {pc}");
}

#[test]
fn test_decode_push_pop_multi() {
    let push = vec![0x00, 0x48, 0x2d, 0xe9]; // PUSH {r11, lr}
    let strings = disassemble(&push).unwrap();
    assert_eq!(strings[0], "push {r11, lr}");
}
