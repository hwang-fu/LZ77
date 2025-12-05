# lz77r

A naive LZ77 compressor implemented in Rust using only the standard library.

## Features

- Sliding-window compression with configurable parameters
- Multiple input sources: file, string, or stdin
- Output to file or stdout
- Self-describing binary format with header
- Zero external dependencies
- Library API for programmatic use

## Building
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

## Usage
```
lz77r [OPTIONS]

OPTIONS:
    -f <path>     Read input from file (binary mode)
    -s <string>   Compress the given string directly
    -o <path>     Write output to file (default: stdout)
    -w <size>     Sliding window size in bytes (default: 4096)
    -m <length>   Maximum match length in bytes (default: 258)
    -h, --help    Show help message
```

### Examples
```bash
# Compress a file to stdout
cargo run -- -f input.bin > output.lz77

# Compress a file to a specific output path
cargo run -- -f input.bin -o output.lz77

# Compress a string
cargo run -- -s "hello hello hello" -o hello.lz77

# Compress with custom parameters
cargo run -- -f input.bin -w 8192 -m 128 -o out.lz77

# Compress from stdin
echo "test data" | cargo run > test.lz77
```

## Binary Format

### Header (9 bytes)

| Offset | Size | Description                      |
|--------|------|----------------------------------|
| 0-4    | 5    | Magic bytes: `LZ77R`            |
| 5-6    | 2    | Window size (u16 little-endian)  |
| 7-8    | 2    | Max match length (u16 little-endian) |

### Token Stream

| Type    | Format                              | Size    |
|---------|-------------------------------------|---------|
| Literal | `0x00` `<byte>`                     | 2 bytes |
| Match   | `0x01` `<offset:u16>` `<length:u16>` | 5 bytes |

After emitting a match, the next byte is emitted as a literal (classic LZ77 behavior).

## Library API

You can use the compression functions programmatically:
```rust
use lz77r::{compress_bytes, compress_str};
use std::io::Cursor;

fn main() -> std::io::Result<()> {
    let mut output = Vec::new();

    // Compress bytes
    let bytes_written = compress_bytes(
        b"hello hello hello",
        &mut output,
        4096,  // window_size
        258,   // max_match_len
    )?;

    println!("Wrote {} bytes", bytes_written);

    // Or compress a string
    let mut output2 = Vec::new();
    compress_str("aaabaaab", &mut output2, 4096, 258)?;

    Ok(())
}
```

## Algorithm

This implementation uses a naive O(n × window_size) sliding-window approach:

1. For each position in the input, search backward through the window
2. Find the longest match (minimum 3 bytes to be worthwhile)
3. Emit either a match token (offset + length) or a literal byte
4. After a match, emit the next byte as a literal

## Limitations

- **Compressor only**: No decompressor included
- **Naive complexity**: O(n × window_size) — not optimized for large files
- **Single-threaded**: No parallel processing
- **Minimum match**: Fixed at 3 bytes

