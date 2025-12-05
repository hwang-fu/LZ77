//! Core LZ77 Compression Implementation

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

use std::io::{self, Write};

/// Magic bytes identifying the LZ77R format.
const LZ77_MAGIC: &[u8; 5] = b"LZ77R";

const LZ77_MIN_MATCH_LEN: usize = 3;

/// Token type maic byte for a literal.
const LZ77_TOKEN_LITERAL_MAGIC: u8 = 0x00;

/// Token type magic byte for a reference.
const LZ77_TOKEN_REFERENCE_MAGIC: u8 =  0x01;

// -----------------------------------------------------------------------------
// Header and Token Emission
// -----------------------------------------------------------------------------

/// Writes the file header.
///
/// Format (10 bytes total):
/// - Bytes 0-5: Magic "LZ77R1"
/// - Bytes 6-7: window_size as u16 little-endian
/// - Bytes 8-9: max_match_len as u16 little-endian
fn lz77_write_header(out: &mut impl Write, window_szie: u16, max_match_len: u16) -> io::Result<u64> {
    out.write_all(LZ77_MAGIC)?;
    out.write_all(&window_szie.to_le_bytes())?;
    out.write_all(&max_match_len.to_le_bytes())?;
    Ok(10)
}

/// Emits a 2-byte literal token: [0x00][byte_value]
#[inline]
fn lz77_emit_literal_token(out: &mut impl Write, literal: u8) -> io::Result<u64> {
    out.write_all(&[LZ77_TOKEN_LITERAL_MAGIC, literal])?;
    Ok(2)
}

/// Emits a 5-byte reference token: [0x01][offset as u16 LE][length as u16 LE]
#[inline]
fn lz77_emit_reference_token(out: &mut impl Write, offset: u16, length: u16) -> io::Result<u64> {
    out.write_all(&[LZ77_TOKEN_REFERENCE_MAGIC])?;
    out.write_all(&offset.to_le_bytes())?;
    out.write_all(&length.to_le_bytes())?;
    Ok(5)
}
