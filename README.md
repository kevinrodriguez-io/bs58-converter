# bs58-converter

A command-line tool that converts Base58 strings to byte arrays and saves them as JSON files.

I made this as a little helper to grab my id.json file from my Phantom Solana wallet.

## Features

- Convert Base58 strings to byte arrays
- Asynchronous file operations
- Progress indicators for each step
- Customizable output file path

## Installation

### From Source

1. Make sure you have Rust and Cargo installed. If not, install them from [rustup.rs](https://rustup.rs/)

2. Clone the repository and build:
```bash
git clone https://github.com/kevinrodriguez-io/bs58-converter.git
cd bs58-converter
cargo install --path .
```

This will install the `bs58-converter` binary in your system.

## Usage

Basic usage:
```bash
bs58-converter -i <BASE58_STRING> -o ~/.config/solana/id.json
```

This will create an `output.json` file in the current directory.

### Options

- `-i, --input`: The Base58 string to convert (required)
- `-f, --input-file`: Input file containing the Base58 string (optional)
- `-o, --output`: Output JSON file path (optional, defaults to "output.json")
- `-h, --help`: Show help information
- `-V, --version`: Show version information

### Examples

Convert a Base58 string to a JSON file:
```bash
bs58-converter --input "5Q6qHRLW2wntVBwRWFyqj5eXKFqHN6"
```

Specify a custom output file:
```bash
bs58-converter --input "5Q6qHRLW2wntVBwRWFyqj5eXKFqHN6" --output my-bytes.json
```

## Output Format

The tool generates a JSON file with the following structure:
```json
[1,2,3,...]  // Array of decoded bytes
```
