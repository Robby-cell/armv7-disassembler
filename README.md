# ARMv7 Disassembler in Rust

A pure-Rust library that translates ARMv7 (A32) machine code bytes back into readable assembly text.  
**No native dependencies** – compiles natively and to WebAssembly via `wasm-pack`.

<!-- [![Crates.io](https://img.shields.io/crates/v/armv7-disassembler?style=flat-square)](https://crates.io/crates/armv7-disassembler) -->
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## Features

- Full data‑processing decoding (MOV, ADD, SUB, AND, ORR, EOR, BIC, MVN, RSB, RSC, ADC, SBC, CMP, CMN, TST, TEQ)
- Decodes all shifter operands: immediate, register, and shifted registers (LSL, LSR, ASR, ROR, RRX)
- Load/store decoding: LDR, STR, LDRB, STRB including clean formatting for negative offsets (e.g., `#-4`)
- Branch resolution: calculates absolute PC-relative target addresses for B and BL based on the provided start address
- Multiply: MUL
- Stack operations: PUSH, POP
- Smart Aliasing: Automatically unwinds optimized single-register stack operations (e.g., `STR LR, [SP, #-4]!`) back into readable `push {lr}` / `pop {pc}` pseudo-instructions
- Supervisor call: SVC
- Hint instructions: NOP, YIELD, WFE, WFI, SEV
- Breakpoint: BKPT (always unconditional)
- Condition codes and `s` flags gracefully appended (e.g., `addeq`, `movs`)
- Safe fallback: Unrecognized or invalid data bytes are safely decoded as `.word 0x...` directives instead of crashing
- Endianness selection (Little/Big)
- Custom start address offset
- Clear error reporting for unaligned instruction streams

---

## Quick Start

Add the library to your `Cargo.toml`:

```toml
[dependencies]
armv7-disassembler = { git = "https://github.com/Robby-cell/armv7-disassembler.git", tag = "0.3.0" }
```

Then disassemble some bytes:

```rust
use armv7_disassembler::prelude::*;

fn main() {
    // Machine code for:
    // mov r0, #42
    // bx lr
    let machine_code: [u8; 8] =[0x2a, 0x00, 0xa0, 0xe3, 0x1e, 0xff, 0x2f, 0xe1];

    // disassemble() automatically uses DisassemblerOptions::default()
    let instructions = disassemble(&machine_code).unwrap();
    
    for instr in instructions {
        println!("{}", instr);
    }
}
```

---

## Disassembler Options

You can control the decoding behavior, starting PC offset, and byte reading order via `DisassemblerOptions`. Both the types and the `disassemble_with_options` function are available in the prelude.

```rust
use armv7_disassembler::prelude::*;

let options = DisassemblerOptions {
    start_address: 0x8000,
    endian: Endian::Big,
};

let instructions = disassemble_with_options(&bytes, options).unwrap();
```

- `start_address` – Base address (PC) used to calculate absolute target addresses for branching instructions (`B`, `BL`).
- `endian` – `Endian::Little` (default) or `Endian::Big`. Determines how the 4-byte chunks are packed into 32-bit instruction words before decoding.

---

## Supported Instructions & Syntax

| Category             | Mnemonics                                                       |
|----------------------|-----------------------------------------------------------------|
| Data processing      | MOV, MVN, ADD, SUB, RSB, ADC, SBC, RSC, AND, ORR, EOR, BIC      |
| Comparisons          | CMP, CMN, TST, TEQ                                              |
| Load/Store           | LDR, STR, LDRB, STRB                                            |
| Multiply             | MUL                                                             |
| Stack                | PUSH, POP (Including implicit LDR/STR unwinding)                |
| Branch               | B, BL, BX                                                       |
| Supervisor           | SVC                                                             |
| Hints & Breakpoint   | NOP, YIELD, WFE, WFI, SEV, BKPT                                 |
| Misc                 | NOP, `.word` (Fallback for arbitrary data blocks)               |

### Disassembly Formatting Output

- **Immediate**: Formats numbers cleanly (`#4` for small numbers, `#0x20` for larger HEX).
- **Register**: Outputs standard ABI lowercase aliases (`r0`-`r12`, `sp`, `lr`, `pc`).
- **Shifts**: Correctly unpacks `r1, lsl #2`, `r2, ror r3`, `r4, rrx`.
- **Memory**: Resolves offset signs cleanly: `[r1, #-4]` vs `[r1, #4]`.
- **Hints & BKPT** – Hint instructions (NOP, YIELD, WFE, WFI, SEV) may carry a condition code; `BKPT` is always unconditional and printed as `bkpt #0x…`.

---

## Error Handling

The library returns a `DecodeError` enum implementing `Display + Error`. Example match:

```rust
match disassemble(&bytes) {
    Err(DecodeError::UnalignedInput) => println!("Bytes slice length must be a multiple of 4"),
    Err(DecodeError::InvalidRegisterCode { code }) => println!("Invalid register: {}", code),
    Err(DecodeError::UnknownInstruction { word }) => println!("Failed to decode word: 0x{:08x}", word),
    Ok(insts) => println!("{:#?}", insts),
}
```
*(Note: Most unrecognized words are caught cleanly and output as `.word 0x...` rather than throwing a hard error).*

---

## WebAssembly Usage

Build with `wasm-pack`:

```bash
wasm-pack build --target web --features wasm
```

Then use in JavaScript:

```javascript
import {Decoder, Endian} from './pkg/armv7_disassembler.js'

const d = new Decoder(0, Endian.Little)
const code = d.disassemble(new Uint8Array([0x04, 0xf0, 0x9d, 0xe5]))
console.log(code)
```

---

## Testing

Run the full test suite:

```bash
cargo test
```

Extensive unit tests cover decoding instruction bytes back to strings, handling memory offset edge cases, and verifying endianness compatibility.

---

## License

This project is licensed under the MIT license. See [LICENSE](LICENSE) for details.

---

## Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.
