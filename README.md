<div align="center">

# lonesha256

[![Crates.io](https://img.shields.io/crates/v/lonesha256)](https://crates.io/crates/lonesha256)
[![Crates.io](https://img.shields.io/crates/d/lonesha256)](https://crates.io/crates/lonesha256)

Rust bindings for the lonesha256 C library - a portable, endian-proof, single-file SHA256 implementation

</div>

## About

This crate provides Rust bindings for [lonesha256](https://github.com/BareRose/lonesha256), making it easy to use this lightweight SHA256 implementation in your Rust projects.

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
lonesha256 = "1.1.0"
```
or run this command in your terminal:
```sh
cargo add lonesha256
```

## Usage

```rust
use lonesha256::lonesha256;

fn main() {
    let input = b"Hello, world!";
    let mut hash = [0u8; 32];
    
    lonesha256(&mut hash, input).expect("Failed to compute hash");
    
    println!("Hash: {}", 
        hash.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>());
}
```

## Documentation

Library documentation can be found [here](https://docs.rs/lonesha256/latest/lonesha256/)

## Features

- Lightweight SHA256 implementation
- No external dependencies
- Safe Rust wrapper around C code
- Comprehensive test suite
- Multiple usage examples

## Building from Source

1. Install `clang`, as said [here](https://rust-lang.github.io/rust-bindgen/requirements.html)
2. Clone the repository:
```
git clone https://github.com/ankddev/lonesha256
cd lonesha256
```

3. Build:
```
cargo build --release
```

## Examples

Run the included examples:
```sh
cargo run --example sample       # Basic usage
cargo run --example file_hash   # File hashing
cargo run --example streaming   # Process data in chunks
cargo run --example hex_output # Different output formats
cargo run --example benchmark  # Performance testing
```

## Testing

Run the test suite:
```
cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

[ANKDDEV](https://github.com/ankddev)

## Acknowledgments

- [lonesha256](https://github.com/BareRose/lonesha256) - Original C implementation by [BareRose](https://github.com/BareRose)
