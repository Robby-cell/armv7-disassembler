//! JS bindings exposed only when the `wasm` feature is enabled.

use wasm_bindgen::prelude::*;

use crate::disassembler::disassemble;

/// Disassemble ARMv7 (A32) machine code into assembly text.
/// Returns a JavaScript `Array` of strings on success, or throws an error string on failure.
#[wasm_bindgen]
pub fn disassemble_armv7(bytes: &[u8]) -> Result<Vec<String>, JsError> {
    disassemble(bytes).map_err(|e| JsError::new(&e.to_string()))
}
