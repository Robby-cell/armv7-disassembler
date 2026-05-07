use crate::prelude::{DisassemblerOptions, Endian, disassemble_with_options};

#[test]
fn test_decode_branch() {
    let bytes = vec![0xfe, 0xff, 0xff, 0xea]; // b start (offset -2 words)
    let options = DisassemblerOptions {
        start_address: 0x1008,
        endian: Endian::Little,
    };
    let strings = disassemble_with_options(&bytes, options).unwrap();

    // PC = 0x1008 + 8 = 0x1010
    // Offset = -2 words = -8 bytes
    // Target = 0x1010 - 8 = 0x1008
    assert_eq!(strings[0], "b 0x00001008");
}
