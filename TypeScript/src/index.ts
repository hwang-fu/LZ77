/**
 * LZ77 Compression Algorithm
 *
 * A TypeScript implementation of the LZ77 lossless data compression algorithm.
 *
 * @example
 * ```typescript
 * import { compress, decompress, decompressToString } from './lz77';
 *
 * const original = "abracadabra";
 * const tokens = compress(original);
 * const restored = decompressToString(tokens);
 * console.log(restored); // "abracadabra"
 * ```
 *
 * @example
 * ```typescript
 * // Binary encoding for storage/transmission
 * import { compressToBinary, decompressFromBinary } from './lz77';
 *
 * const binary = compressToBinary("Hello, World!");
 * const restored = new TextDecoder().decode(decompressFromBinary(binary));
 * ```
 */

export {
  compress,
  decompress,
  decompressToString,
  encodeTokens,
  decodeTokens,
  compressToBinary,
  decompressFromBinary,
} from './lz77';

export type {
  LZ77Token,
  LiteralToken,
  ReferenceToken,
  LZ77Options,
} from "./types";

export {
  DEFAULT_OPTIONS,
  isLiteral,
  isReference,
} from "./types";

