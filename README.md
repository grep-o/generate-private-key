# Ethereum Address Generator

A secure Rust implementation for generating Ethereum private keys and addresses. Implements the official Ethereum address algorithm with EIP-55 checksum encoding.

## Features

- Cryptographically secure private key generation
- Secp256k1 public key derivation
- Keccak-256 hashing for address generation
- EIP-55 compliant checksum addresses
- Support for importing existing private keys

## Dependencies

```toml
[dependencies]
rand = "0.8"
secp256k1 = "0.28"
tiny-keccak = { version = "2.0", features = ["keccak"] }
hex = "0.4.3"
```

## Usage

```rust
// Generate new address
let (private_key, address) = generate_ethereum_address(None)?;
println!("Private Key: {}", private_key);
println!("Ethereum Address: {}", address);

// Use existing private key
let my_private_key = [/* your 32 bytes */];
let (private_key, address) = generate_ethereum_address(Some(my_private_key))?;
```

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