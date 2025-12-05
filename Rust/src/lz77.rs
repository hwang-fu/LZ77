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
fn write_header(out: &mut impl Write, window_szie: usize, max_match_len: usize) -> io::Result<u64> {
    out.write_all(LZ77_MAGIC)?;
    out.write_all(&window_szie.to_le_bytes())?;
    out.write_all(&max_match_len.to_le_bytes())?;
    Ok(10)
}

/// Emits a 2-byte literal token: [0x00][byte_value]
#[inline]
fn emit_literal_token(out: &mut impl Write, literal: u8) -> io::Result<u64> {
    out.write_all(&[LZ77_TOKEN_LITERAL_MAGIC, literal])?;
    Ok(2)
}

/// Emits a 5-byte reference token: [0x01][offset as u16 LE][length as u16 LE]
#[inline]
fn emit_reference_token(out: &mut impl Write, offset: u16, length: u16) -> io::Result<u64> {
    out.write_all(&[LZ77_TOKEN_REFERENCE_MAGIC])?;
    out.write_all(&offset.to_le_bytes())?;
    out.write_all(&length.to_le_bytes())?;
    Ok(5)
}

// -----------------------------------------------------------------------------
// Match Finding
// -----------------------------------------------------------------------------

/// Computes how many bytes match between two positions in the input.
#[inline]
fn compute_match_length(
    input: &[u8],
    candidate: usize,
    pos: usize,
    max_len: usize,
) -> usize {
    let mut length = 0;
    let max_possible = max_len.min(input.len() - pos);

    while length < max_possible && input[candidate + length] == input[pos + length] {
        length += 1;
    }

    length
}

/// Searches backward in the sliding window for the longest match.
///
/// This is a naive O(window_size) search for each position. It scans every
/// position in the window and keeps track of the longest match found.
///
/// # Returns
/// (offset, length) where offset is the distance backward from `pos`.
/// Returns (0, 0) if no match of at least MIN_MATCH_LEN is found.
fn find_longest_match(
    input: &[u8],
    pos: usize,
    window_size: usize,
    max_match_len: usize,
) -> (usize, usize) {
    let mut best_offset: usize = 0;
    let mut best_length: usize = 0;

    // Window spans from max(0, pos - window_size) to pos (exclusive)
    let window_start = pos.saturating_sub(window_size);

    // Try each candidate position in the window
    for candidate in window_start..pos {
        let length = compute_match_length(input, candidate, pos, max_match_len);
        if length > best_length {
            best_length = length;
            best_offset = pos - candidate;
        }
    }

    (best_offset, best_length)
}

// -----------------------------------------------------------------------------
// Core Compression Logic
// -----------------------------------------------------------------------------

fn compress(
    input: &[u8],
    out: &mut impl Write,
    window_size: usize,
    max_match_len: usize,
) -> io::Result<u64> {
    let mut bytes_written: u64 = 0;
    let mut pos: usize = 0;

    while pos < input.len() {
        let (match_offset, match_length) = find_longest_match(input, pos, window_size, max_match_len);
        if match_length >= LZ77_MIN_MATCH_LEN {
            bytes_written += emit_reference_token(out, match_offset as u16, match_length as u16)?;
            pos += match_length;

            // Classic LZ77: emit the next byte as a literal (if any remain)
            if pos < input.len() {
                bytes_written += emit_literal_token(out, input[pos])?;
                pos += 1;
            }
        } else {
            bytes_written += emit_literal_token(out, input[pos])?;
            pos += 1;
        }
    }

    Ok(bytes_written)
}

// -----------------------------------------------------------------------------
// Public API
// -----------------------------------------------------------------------------

pub fn compress_bytes(
    input: &[u8],
    out: &mut impl Write,
    window_size: usize,
    max_match_len: usize,
) -> io::Result<u64> {
    let mut bytes_written: u64 = 0;
    bytes_written += write_header(out, window_size, max_match_len)?;
    bytes_written += compress(input, out, window_size, max_match_len)?;
    Ok(bytes_written)
}

pub fn compress_str(
    s: &str,
    out: &mut impl Write,
    window_size: usize,
    max_match_len: usize,
) -> io::Result<u64> {
    compress(s.as_bytes(), out, window_size, max_match_len)
}
