# LZ77 Compression - TypeScript Implementation

A pure TypeScript implementation of the LZ77 lossless data compression algorithm with no third-party dependencies.

## Overview

LZ77 is a dictionary-based compression algorithm that replaces repeated occurrences of data with references to a single copy existing earlier in the uncompressed data stream. This implementation provides:

- Compression and decompression of strings and binary data
- Configurable sliding window size
- Binary encoding/decoding for storage and transmission
- Full TypeScript type safety

## Installation

```bash
npm install
```

## Usage

### Basic Compression/Decompression

```typescript
import { compress, decompressToString } from "./src";

const original = "abracadabra";
const tokens = compress(original);
const restored = decompressToString(tokens);

console.log(restored); // "abracadabra"
```

### Binary Data

```typescript
import { compress, decompress } from "./src";

const data = new Uint8Array([1, 2, 3, 1, 2, 3, 4, 5]);
const tokens = compress(data);
const restored = decompress(tokens);
```

### Binary Encoding for Storage

```typescript
import { compressToBinary, decompressFromBinary } from "./src";

const original = "Hello, World!";
const binary = compressToBinary(original);

// Store or transmit `binary`...

const restored = new TextDecoder().decode(decompressFromBinary(binary));
```

### Custom Options

```typescript
import { compress } from "./src";

const tokens = compress(data, {
  windowSize: 8192,      // Search buffer size (default: 4096)
  minMatchLength: 4,     // Minimum match to encode as reference (default: 3)
  maxMatchLength: 512,   // Maximum match length (default: 258)
});
```

## API Reference

### Functions

| Function | Description |
|----------|-------------|
| `compress(input, options?)` | Compresses data to LZ77 tokens |
| `decompress(tokens)` | Decompresses tokens to `Uint8Array` |
| `decompressToString(tokens)` | Decompresses tokens to string |
| `encodeTokens(tokens)` | Encodes tokens to binary format |
| `decodeTokens(data)` | Decodes binary format to tokens |
| `compressToBinary(input, options?)` | Compress and encode in one step |
| `decompressFromBinary(data)` | Decode and decompress in one step |

### Types

```typescript
type LZ77Token = LiteralToken | ReferenceToken;

interface LiteralToken {
  literal: number;  // Single byte value
}

interface ReferenceToken {
  offset: number;   // Distance back in buffer
  length: number;   // Number of bytes to copy
}

interface LZ77Options {
  windowSize: number;      // Default: 4096
  minMatchLength: number;  // Default: 3
  maxMatchLength: number;  // Default: 258
}
```

## Development

### Build

```bash
npm run build
```

### Run Tests

```bash
npm test
```

### Type Check

```bash
npm run typecheck
```

## Algorithm Details

The LZ77 algorithm works by maintaining a "sliding window" of recently processed data:

1. **Search Buffer**: Contains previously seen data that can be referenced
2. **Look-ahead Buffer**: Contains data being examined for matches

For each position, the algorithm:
1. Searches the buffer for the longest matching sequence
2. If a match ≥ minimum length is found, outputs a reference token `(offset, length)`
3. Otherwise, outputs the literal byte

### Binary Format

The binary encoding uses a simple format:
- **Literal**: `[0x00] [byte]` (2 bytes)
- **Reference**: `[0x01] [offset-hi] [offset-lo] [length-hi] [length-lo]` (5 bytes)
