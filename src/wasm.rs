//! JS bindings exposed only when the `wasm` feature is enabled.

use js_sys::Array;
use wasm_bindgen::prelude::*;

use crate::disassembler::{
    Decoder as CoreDecoder, DisassemblerOptions, Endian as AV7Endian, disassemble,
};

/// Endianness options natively exposed to WASM.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Endian {
    Little = 0,
    Big = 1 << 30,
}

impl From<Endian> for AV7Endian {
    fn from(e: Endian) -> Self {
        match e {
            Endian::Little => AV7Endian::Little,
            Endian::Big => AV7Endian::Big,
        }
    }
}

/// An object-oriented Disassembler Decoder exposed to JavaScript.
#[wasm_bindgen]
pub struct Decoder {
    start_address: u32,
    endian: Endian,
}

#[wasm_bindgen]
impl Decoder {
    /// Create a new Decoder.
    #[wasm_bindgen(constructor)]
    pub fn new(start_address: Option<u32>, endian: Option<Endian>) -> Self {
        Self {
            start_address: start_address.unwrap_or(0),
            endian: endian.unwrap_or(Endian::Little),
        }
    }

    /// Disassemble the provided ARMv7 machine code bytes using this object's internal state.
    /// Returns a JavaScript `Array` of strings.
    #[wasm_bindgen]
    pub fn disassemble(&self, bytes: &[u8]) -> Result<Array, JsError> {
        let options = DisassemblerOptions {
            start_address: self.start_address,
            endian: self.endian.into(),
        };

        let core_decoder = CoreDecoder::new(options);
        let instrs = core_decoder
            .disassemble(bytes)
            .map_err(|e| JsError::new(&e.to_string()))?;

        let arr = Array::new_with_length(instrs.len() as u32);
        for (i, instr) in instrs.iter().enumerate() {
            arr.set(i as u32, JsValue::from_str(instr));
        }

        Ok(arr)
    }
}

/// Standalone function: Disassemble ARMv7 (A32) bytes into a string array (Little‑Endian).
#[wasm_bindgen]
pub fn disassemble_armv7(bytes: &[u8]) -> Result<Array, JsError> {
    let instrs = disassemble(bytes).map_err(|e| JsError::new(&e.to_string()))?;

    let arr = Array::new_with_length(instrs.len() as u32);
    for (i, instr) in instrs.iter().enumerate() {
        arr.set(i as u32, JsValue::from_str(instr));
    }

    Ok(arr)
}

/// Standalone function: Disassemble ARMv7 (A32) bytes into a string array (Big‑Endian).
#[wasm_bindgen]
pub fn disassemble_armv7_big_endian(bytes: &[u8]) -> Result<Array, JsError> {
    let options = DisassemblerOptions {
        start_address: 0,
        endian: AV7Endian::Big,
    };

    let core_decoder = CoreDecoder::new(options);
    let instrs = core_decoder
        .disassemble(bytes)
        .map_err(|e| JsError::new(&e.to_string()))?;

    let arr = Array::new_with_length(instrs.len() as u32);
    for (i, instr) in instrs.iter().enumerate() {
        arr.set(i as u32, JsValue::from_str(instr));
    }

    Ok(arr)
}
