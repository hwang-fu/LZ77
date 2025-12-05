# LZ77 Compression - C Implementation

A minimal, portable C99 implementation of the LZ77 compression algorithm for UNIX systems.

## Building

```bash
make
```

This produces `bin/lz77`.

## Usage

```
Usage: lz77 [OPTIONS]

Options:
  -c          Compress (default)
  -d          Decompress
  -i FILE     Read input from FILE
  -s STRING   Use STRING as input
  -o FILE     Write output to FILE (default: stdout)
  -h          Show this help
```

### Examples

Compress a file:
```bash
./bin/lz77 -i input.txt -o compressed.lz77
```

Decompress a file:
```bash
./bin/lz77 -d -i compressed.lz77 -o output.txt
```

Compress a string:
```bash
./bin/lz77 -s "hello world hello world" -o out.lz77
```

Compress to stdout:
```bash
./bin/lz77 -s "test data" > compressed.lz77
```

## Testing

```bash
make test
```

## Binary Format

The compressed output uses a simple token format:

| Token Type | Format | Size |
|------------|--------|------|
| Literal    | `[0x00][byte]` | 2 bytes |
| Reference  | `[0x01][offset:2][length:2]` | 5 bytes |

Offset and length are stored as big-endian 16-bit integers.

## Files

```
C/
├── Makefile
├── README.md
└── include/
    ├── lz77.h      # Public API
└── src/
    ├── lz77.c      # Compression/decompression logic
    └── main.c      # CLI interface
```

## License

MIT

