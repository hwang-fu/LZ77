/**
 * Type definitions for the LZ77 compression algorithm.
 */

/**
 * Represents a literal byte in the compressed output.
 */
export type LiteralToken = {
  literal: number;
};

/**
 * Represents a back-reference to previously seen data.
 */
export type ReferenceToken = {
  offset: number;
  length: number;
};

/**
 * A token is either a literal byte or a back-reference.
 */
export type LZ77Token = LiteralToken | ReferenceToken;

/**
 * Configuration options for the LZ77 compressor.
 */
export type LZ77Options = {
  /**
   * Size of the sliding window (search buffer).
   * Larger values can find longer matches but use more memory.
   * Default: 4096
   */
  windowSize: number;

  /**
   * Minimum match length to encode as a reference.
   * Matches shorter than this are stored as literals.
   * Default: 3
   */
  minMatchLength: number;

  /**
   * Maximum match length to encode.
   * Default: 258
   */
  maxMatchLength: number;
};

/**
 * Default compression options.
 */
export const DEFAULT_OPTIONS: LZ77Options = {
  windowSize: 4096,
  minMatchLength: 3,
  maxMatchLength: 258,
};

/**
 * Type guard to check if a token is a literal.
 */
export function isLiteral(token: LZ77Token): token is LiteralToken {
  return "literal" in token;
}

/**
 * Type guard to check if a token is a reference.
 */
export function isReference(token: LZ77Token): token is ReferenceToken {
  return "offset" in token && "length" in token;
};
