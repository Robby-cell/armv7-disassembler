pub mod decoder;
pub mod disassembler;
pub mod error;
pub mod types;

pub mod prelude;

// Only include the WASM bindings when the feature is active
#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(test)]
mod tests;
