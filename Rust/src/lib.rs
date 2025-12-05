//! lz77_r - A Simple LZ77 Compressor Library

pub mod lz77;

// Re-export main functions for convenience
pub use lz77::{compress_str, compress_bytes};
