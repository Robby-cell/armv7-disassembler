use crate::disassembler::Decoder;

#[test]
fn test_decode_branch() {
    let bytes = vec![0xfe, 0xff, 0xff, 0xea]; // b start (offset -2 words)

    let strings = Decoder::new()
        .start_address(0x1008)
        .disassemble(&bytes)
        .unwrap();

    // PC = 0x1008 + 8 = 0x1010
    // Offset = -2 words = -8 bytes
    // Target = 0x1010 - 8 = 0x1008
    assert_eq!(strings[0], "b 0x00001008");
}
