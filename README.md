# Ethereum Address Generator

A secure Rust implementation for generating Ethereum private keys and addresses. Implements the official Ethereum address algorithm with EIP-55 checksum encoding.

## Quick Start

### Pre-built Binary (Mac M2)
A pre-built binary for Apple Silicon (M1/M2) is available in the `bin` directory. To use it:

1. Clone this repository
2. Navigate to the `bin` directory
3. Make the binary executable and run:
```bash
chmod +x generate-private-key
./generate-private-key
```

### Building from Source

If you need to build for a different platform or want to compile from source:

#### Dependencies
```toml
[dependencies]
rand = "0.8"
secp256k1 = "0.28"
tiny-keccak = { version = "2.0", features = ["keccak"] }
hex = "0.4.3"
```

#### Build
```bash
cargo build --release
```
The executable will be available at `target/release/generate-private-key`

## Features

- Cryptographically secure private key generation
- Secp256k1 public key derivation
- Keccak-256 hashing for address generation
- EIP-55 compliant checksum addresses
- Support for importing existing private keys

## Usage

The program will output:
- A new private key (keep this secret!)
- The corresponding Ethereum address

## Security

- Uses `OsRng` for secure random number generation
- Validates private keys against the secp256k1 curve order
- Implements address checksum for error detection
- Full test coverage including official test vectors

## Testing

```bash
cargo test
```

## License

MIT