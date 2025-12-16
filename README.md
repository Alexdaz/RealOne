# RealOne

[![License: MIT](https://img.shields.io/badge/License-MIT-orange.svg)](https://opensource.org/licenses/MIT)

A modern, cross-platform hash calculator application built with Rust and Iced. RealOne provides a clean, intuitive interface for calculating and verifying file checksums using multiple cryptographic hash algorithms.

## Features

- **17 Hash Algorithms**: Support for MD4, MD5, SHA-1, SHA-2 (256/384/512), SHA-3 (224/256/384/512), BLAKE2b, BLAKE2s, RIPEMD-160, TIGER192, WHIRLPOOL, GOST, CRC32, and ADLER32
- **Hash Comparison**: Verify file integrity by comparing calculated hashes with expected values
- **Multiple Output Formats**: Display hashes in hexadecimal (lowercase/uppercase) or Base64
- **Variant Support**: 
  - GOST: Choose between CryptoPro S-box and Test S-box variants
  - TIGER: Select between Tiger (original padding) and Tiger2 (alternate padding)
- **Parallel Processing**: Optimized multi-threaded hash calculation for fast performance
- **Memory Efficient**: Uses memory-mapped I/O for large files to minimize memory usage
- **Copy to Clipboard**: One-click copy for any hash result
- **Persistent Settings**: Your algorithm selections and preferences are saved automatically
- **Modern UI**: Dark theme with a clean, user-friendly interface

<p align="center">
<img src="https://raw.githubusercontent.com/Alexdaz/RealOne/main/Assets/SSRealOne.png" width="792" height="411"/>
</p>

## Installation

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/Alexdaz/RealOne.git
cd RealOne
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

The executable will be located at `target/release/realone`.

## Usage

1. **Select a File**: Click "Browse..." to select the file you want to calculate hashes for
2. **Choose Algorithms**: Click "Settings" to select which hash algorithms you want to calculate
3. **Calculate Hashes**: Click "Check" to start the hash calculation process
4. **Verify Hashes** (optional): Paste an expected hash in the "Check" field to compare with calculated results
5. **Copy Results**: Click the "Copy" button next to any hash to copy it to your clipboard

### Settings

- **Result Format**: Choose between hexadecimal (lowercase/uppercase) or Base64 output
- **GOST Variant**: Select the GOST S-box variant (CryptoPro or Test)
- **TIGER Variant**: Choose between Tiger and Tiger2 padding methods
- **Algorithm Selection**: Enable or disable specific hash algorithms

## Supported Algorithms

| Algorithm | Description |
|-----------|-------------|
| MD4 | Message Digest 4 |
| MD5 | Message Digest 5 |
| SHA-1 | Secure Hash Algorithm 1 |
| SHA-256 | SHA-2 with 256-bit output |
| SHA-384 | SHA-2 with 384-bit output |
| SHA-512 | SHA-2 with 512-bit output |
| SHA3-224 | SHA-3 with 224-bit output |
| SHA3-256 | SHA-3 with 256-bit output |
| SHA3-384 | SHA-3 with 384-bit output |
| SHA3-512 | SHA-3 with 512-bit output |
| BLAKE2b | BLAKE2 with 512-bit output |
| BLAKE2s | BLAKE2 with 256-bit output |
| RIPEMD-160 | RACE Integrity Primitives Evaluation Message Digest |
| TIGER192 | Tiger hash with 192-bit output |
| WHIRLPOOL | Whirlpool hash algorithm |
| GOST | GOST R 34.11-94 hash |
| CRC32 | Cyclic Redundancy Check 32-bit |
| ADLER32 | Adler-32 checksum |

## Performance

RealOne is optimized for performance:

- **Parallel Processing**: Multiple hash algorithms are calculated simultaneously using separate threads
- **Memory-Mapped I/O**: Large files are processed using memory-mapped I/O for efficient memory usage
- **Streaming Support**: Files are read in chunks to minimize memory footprint
- **Smart Buffering**: Adaptive buffer sizes based on file size

## Configuration

Settings are automatically saved to a configuration file in your system's configuration directory:
- **Linux**: `~/.config/realone/config.json`
- **macOS**: `~/Library/Application Support/realone/config.json`
- **Windows**: `%APPDATA%\realone\config.json`

## Dependencies

- [Iced](https://github.com/iced-rs/iced) - Cross-platform GUI framework
- [iced-multi-window](https://github.com/iced-rs/iced_multi_window) - Multi-window support for Iced
- Various cryptographic libraries (sha1, sha2, sha3, blake2, etc.)
- [arboard](https://github.com/1Password/arboard) - Cross-platform clipboard access
- [memmap2](https://github.com/RazrFalcon/memmap2) - Memory-mapped file I/O

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## Acknowledgments

- Built with [Iced](https://github.com/iced-rs/iced), an amazing Rust GUI framework
- Inspired by GTKHash

## Roadmap

- [ ] Add more hash algorithms
- [ ] HMAC hash calculation
- [ ] Batch file processing
- [ ] Export results to file
- [ ] Command-line interface (CLI) mode
