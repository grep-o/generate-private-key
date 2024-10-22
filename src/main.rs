use rand::rngs::OsRng;
use rand::RngCore;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tiny_keccak::{Hasher, Keccak};

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn to_checksum_address(address: &str) -> String {
    let address = address.trim_start_matches("0x").to_lowercase();

    let mut hasher = Keccak::v256();
    hasher.update(address.as_bytes());
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);

    let hash_hex = bytes_to_hex(&hash);

    let mut checksum_address = String::from("0x");
    for (i, c) in address.chars().enumerate() {
        let hash_digit = u8::from_str_radix(&hash_hex[i..i + 1], 16).unwrap();
        if hash_digit >= 8 {
            checksum_address.push(c.to_ascii_uppercase());
        } else {
            checksum_address.push(c);
        }
    }

    checksum_address
}

fn generate_ethereum_address(
    private_key: Option<[u8; 32]>,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();

    // Use provided private key or generate new one
    let secret_key_bytes = match private_key {
        Some(key) => key,
        None => {
            let mut bytes = [0u8; 32];
            OsRng.fill_bytes(&mut bytes);
            bytes
        }
    };

    let secret_key = SecretKey::from_slice(&secret_key_bytes)?;
    let private_key_hex = format!("0x{}", bytes_to_hex(&secret_key.secret_bytes()));

    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let public_key_serialized = public_key.serialize_uncompressed();
    let public_key_bytes = &public_key_serialized[1..];

    let mut hasher = Keccak::v256();
    hasher.update(public_key_bytes);
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);

    let address = &hash[12..];
    let hex_address = format!("0x{}", bytes_to_hex(address));
    let checksum_address = to_checksum_address(&hex_address);

    Ok((private_key_hex, checksum_address))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_known_address() -> Result<(), Box<dyn std::error::Error>> {
        // Test vector from https://github.com/ethereum/EIPs/blob/master/EIPS/eip-55.md
        assert_eq!(
            to_checksum_address("0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed"),
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"
        );

        // Test with known private key
        let private_key =
            hex::decode("f8f8a2f43c8376ccb0871305060d7b27b0554d2cc72bccf41b2705608452f315")?;
        let mut private_key_bytes = [0u8; 32];
        private_key_bytes.copy_from_slice(&private_key);

        let (_, address) = generate_ethereum_address(Some(private_key_bytes))?;
        assert_eq!(address, "0x001d3F1ef827552Ae1114027BD3ECF1f086bA0F9");

        Ok(())
    }

    #[test]
    fn test_address_generation() -> Result<(), Box<dyn std::error::Error>> {
        let (priv_key, address) = generate_ethereum_address(None)?;

        // Basic format checks
        assert!(priv_key.starts_with("0x"));
        assert_eq!(priv_key.len(), 66); // 0x + 64 hex chars

        assert!(address.starts_with("0x"));
        assert_eq!(address.len(), 42); // 0x + 40 hex chars

        // Generate another address - should be different
        let (priv_key2, address2) = generate_ethereum_address(None)?;
        assert_ne!(priv_key, priv_key2);
        assert_ne!(address, address2);

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (private_key, address) = generate_ethereum_address(None)?;

    println!("Private Key: {private_key}");
    println!("Ethereum Address: {address}");

    Ok(())
}
