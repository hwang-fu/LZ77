//! Core LZ77 Compression Implementation

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

/// Magic bytes identifying the LZ77R format.
const MAGIC: &[u8; 5] = b"LZ77R";

const MIN_MATCH_LEN: usize = 3;

/// Token type maic byte for a literal.
const LZ77_TOKEN_LITERAL: u8 = 0x00;

/// Token type magic byte for a reference.
const LZ77_TOKEN_REFERENCE: u8 =  0x01;
