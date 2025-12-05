/**
 * LZ77 Compression Algorithm Implementation
 *
 * A lossless data compression algorithm that uses a sliding window
 * to find and encode repeated sequences of data.
 */

import type { LZ77Token, LZ77Options } from './types.js';
import { DEFAULT_OPTIONS } from './types.js';

/**
 * Finds the longest match in the search buffer for the current position.
 */
function findLongestMatch(
  data: Uint8Array,
  currentPos: number,
  windowSize: number,
  minMatchLength: number,
  maxMatchLength: number,
): { offset: number, length: number } {
  let bestOffset = 0;
  let bestLength = 0;

  const searchStart = Math.max(0, currentPos - windowSize);
  const remainingLength = data.length - currentPos;
  const maxPossibleLength = Math.min(maxMatchLength, remainingLength);

  if (maxPossibleLength < minMatchLength) {
    return { offset: 0, length: 0 };
  }

  for (let searchPos = searchStart; searchPos < currentPos; searchPos++) {
    let matchLength = 0;

    // Compare bytes at searchPos with bytes at currentPos
    while (
      matchLength < maxPossibleLength &&
      data[searchPos + matchLength] === data[currentPos + matchLength]
    ) {
      matchLength++;
    }

    if (matchLength >= minMatchLength && matchLength > bestLength) {
      bestOffset = currentPos - searchPos;
      bestLength = matchLength;

      // Early exit if we found the maximum possible match
      if (bestLength === maxPossibleLength) {
        break;
      }
    }
  }

  return { offset: bestOffset, length: bestLength };
}

/**
 * Compresses data using the LZ77 algorithm.
 *
 * @param input - The input data as a Uint8Array or string
 * @param options - Compression options
 * @returns Array of LZ77 tokens representing the compressed data
 */
export function compress(
  input: Uint8Array | string,
  options: Partial<LZ77Options> = {}
): LZ77Token[] {
  const opts = { ...DEFAULT_OPTIONS, ...options };
  const data =
    typeof input === 'string' ? new TextEncoder().encode(input) : input;

  const tokens: LZ77Token[] = [];
  let pos = 0;

  while (pos < data.length) {
    const { offset, length } = findLongestMatch(
      data,
      pos,
      opts.windowSize,
      opts.minMatchLength,
      opts.maxMatchLength,
    );

    if (length >= opts.minMatchLength) {
      tokens.push({ offset: offset, length: length });
      pos += length;
    } else {
      tokens.push({ literal: data[pos]! });
      pos++;
    }
  }

  return tokens;
}

/**
 * Decompresses LZ77 tokens back to the original data.
 *
 * @param tokens - Array of LZ77 tokens
 * @returns Decompressed data as Uint8Array
 */
export function decompress(tokens: LZ77Token[]): Uint8Array {
  const output: number[] = [];

  for (const token of tokens) {
    if ("literal" in token) {
      output.push(token.literal);
    } else {
      const startPos = output.length - token.offset;
      for (let i = 0; i < token.length; i++) {
        output.push(output[startPos + i]!);
      }
    }
  }

  return new Uint8Array(output);
}

/**
 * Decompresses LZ77 tokens and returns a string.
 *
 * @param tokens - Array of LZ77 tokens
 * @returns Decompressed data as a string
 */
export function decompressToString(tokens: LZ77Token[]): string {
  return new TextDecoder().decode(decompress(tokens));
}

/**
 * Encodes LZ77 tokens into a binary format for storage/transmission.
 *
 * Binary format per token:
 * - Literal: [0x00] [byte]
 * - Reference: [0x01] [offset (2 bytes, big-endian)] [length (2 bytes, big-endian)]
 */
export function encodeTokens(tokens: LZ77Token[]): Uint8Array {
  const chunks: number[] = [];

  for (const token of tokens) {
    if ("literal" in token) {
      chunks.push(0x00, token.literal);
    } else {
      chunks.push(
        0x01,
        (token.offset >> 8) & 0xff,
        token.offset & 0xff,
        (token.length >> 8) & 0xff,
        token.length & 0xff
      );
    }
  }

  return new Uint8Array(chunks);
}

/**
 * Decodes binary-encoded LZ77 tokens.
 */
export function decodeTokens(data: Uint8Array): LZ77Token[] {
  const tokens: LZ77Token[] = [];
  let pos = 0;

  while (pos < data.length) {
    const magic = data[pos++];
    if (magic === 0x00) {
      const literal = data[pos++]!;
      tokens.push({ literal });
    } else if (magic === 0x01) {
      const offset = (data[pos]! << 8) | data[pos + 1]!;
      const length = (data[pos + 2]! << 8) | data[pos + 3]!;
      tokens.push({ offset, length });
      pos += 4;
    } else {
      throw new Error(`Invalid token type: ${magic} at position ${pos - 1}`);
    }
  }

  return tokens;
}

/**
 * Convenience function: compresses and encodes to binary in one step.
 */
export function compressToBinary(
  input: Uint8Array | string,
  options: Partial<LZ77Options> = {}
): Uint8Array {
  return encodeTokens(compress(input, options));
}

/**
 * Convenience function: decodes binary and decompresses in one step.
 */
export function decompressFromBinary(data: Uint8Array): Uint8Array {
  return decompress(decodeTokens(data));
}
