//! CLI for the lz77r compressor


// -----------------------------------------------------------------------------
// Default Configuration
// -----------------------------------------------------------------------------

use std::io::{BufWriter, Read, Write};
use std::{env, io};
use std::fs::File;
use std::process;

use lz77r::lz77;

const DEFAULT_LZ77_WINDOW_SIZE: u16 = 4096;
const DEFAULT_LZ77_MAX_MATCH_LEN: u16 = 258;

// -----------------------------------------------------------------------------
// Argument Parsing
// -----------------------------------------------------------------------------

/// Holds parsed command-line arguments.
struct Args {
    input_string: Option<String>,
    input_filename: Option<String>,
    output_filename: Option<String>,
    window_size: u16,
    max_match_len: u16,
    show_help: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            input_string: None,
            input_filename: None,
            output_filename: None,
            window_size: DEFAULT_LZ77_WINDOW_SIZE,
            max_match_len: DEFAULT_LZ77_MAX_MATCH_LEN,
            show_help: false,
        }
    }
}

fn parse_args() -> Result<Args, String> {
    let argv: Vec<String> = env::args().collect();
    let mut args = Args::default();
    let mut i = 1;

    while i < argv.len() {
        match argv[i].as_str() {
            "-h" | "--help" => {
                args.show_help = true;
                return Ok(args);
            }

            "-f" => {
                i += 1;
                if i >= argv.len() {
                    return Err("-f requires a file path argument".into());
                }
                args.input_filename = Some(argv[i].clone());
            }

            "-s" => {
                i += 1;
                if i >= argv.len() {
                    return Err("-s requires a string argument".into());
                }
                args.input_string = Some(argv[i].clone());
            }

            "-o" => {
                i += 1;
                if i >= argv.len() {
                    return Err("-o requires an output path argument".into());
                }
                args.output_filename = Some(argv[i].clone());
            }

            "-w" => {
                i += 1;
                if i >= argv.len() {
                    return Err("-w requires a numeric argument".into());
                }
                args.window_size = argv[i]
                    .parse()
                    .map_err(|_| format!("Invalid window size: '{}'", argv[i]))?;
            }

            "-m" => {
                i += 1;
                if i >= argv.len() {
                    return Err("-m requires a numeric argument".into());
                }
                args.max_match_len = argv[i]
                    .parse()
                    .map_err(|_| format!("Invalid max match length: '{}'", argv[i]))?;
            }

            other => {
                return Err(format!("Unknown argument: '{}'", other));
            }
        }
        i += 1;
    }

    if args.input_filename.is_some() && args.input_string.is_some() {
        return Err("Cannot specify both -f (file) and -s (string)".into());
    }

    Ok(args)
}

// -----------------------------------------------------------------------------
// Input/Output Handling
// -----------------------------------------------------------------------------

/// Prints usage information to stdout.
fn print_help() {
    let help = r#"lz77r - LZ77 compressor (Rust, std only)

USAGE:
    lz77r [OPTIONS]

OPTIONS:
    -f <path>     Read input from file (binary mode)
    -s <string>   Compress the given string directly
    -o <path>     Write output to file (default: stdout)
    -w <size>     Sliding window size in bytes (default: 4096)
    -m <length>   Maximum match length in bytes (default: 258)
    -h, --help    Show this help message

INPUT:
    If neither -f nor -s is provided, input is read from stdin.
    You cannot specify both -f and -s simultaneously.

OUTPUT FORMAT:
    Header (10 bytes):
        - Magic: "LZ77R1" (6 bytes)
        - Window size: u16 little-endian (2 bytes)
        - Max match length: u16 little-endian (2 bytes)

    Tokens (variable):
        - Literal: 0x00 <byte>
        - Match:   0x01 <offset:u16_le> <length:u16_le>

EXAMPLES:
    lz77r -f input.bin > output.lz77
    lz77r -f input.bin -o output.lz77
    lz77r -s "hello hello hello" -o hello.lz77
    echo "test data" | lz77r > test.lz77
"#;
    print!("{}", help);
}

fn read_input(args: &Args) -> io::Result<Vec<u8>> {
    if let Some(ref path) = args.input_filename {
        let mut file = File::open(path)
            .map_err(|e| io::Error::new(e.kind(), format!("Cannot open '{}': {}", path, e)))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    } else if let Some(ref s) = args.input_string {
        Ok(s.as_bytes().to_vec())
    } else {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

fn create_output(args: &Args) -> io::Result<Box<dyn Write>> {
    match &args.output_filename {
        Some(path) => {
            let file = File::create(path)
                .map_err(|e| io::Error::new(e.kind(), format!("Cannot open '{}': {}", path, e)))?;
            Ok(Box::new(BufWriter::new(file)))
        }
        None => {
            Ok(Box::new(BufWriter::new(io::stdout().lock())))
        }
    }
}

fn run(args: &Args) -> io::Result<()> {
    let input_data = read_input(args)?;
    let input_size = input_data.len();

    let mut output = create_output(args)?;

    let bytes_written = lz77::compress_bytes(
        &input_data,
        &mut output,
        args.window_size as usize,
        args.max_match_len as usize,
    )?;

    output.flush()?;

    if args.output_filename.is_some() {
        eprintln!(
            "Compressed {} bytes -> {} bytes ({:.1}%)",
            input_size,
            bytes_written,
            if input_size > 0 {
                (bytes_written as f64 / input_size as f64) * 100.0
            } else {
                0.0
            }
        );
    }

    Ok(())
}

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Use -h for usage information.");
            process::exit(1);
        }
    };

    if args.show_help {
        print_help();
        return;
    }

    if let Err(e) = run(&args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
