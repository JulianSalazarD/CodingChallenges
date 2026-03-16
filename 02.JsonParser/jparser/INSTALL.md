# Installation

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (includes `cargo`)

## Build

```bash
cargo build --release
```

The binary will be available at `./target/release/jparser`.

## Install globally

```bash
cargo install --path .
```

After installing, `jparser` will be available system-wide.

## Usage

```bash
jparser [FILE]
```

Validates a JSON file and outputs:
- `0` if the JSON is valid
- `1` if the JSON is invalid
