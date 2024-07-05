use crate::{SolanaPayError, SolanaPayResult};

pub struct Utils;

impl Utils {
    pub fn parse_public_key(base58_str: &str) -> SolanaPayResult<[u8; 32]> {
        let mut buffer = [0u8; 32];
        bs58::decode(base58_str)
            .onto(&mut buffer)
            .map_err(|_| SolanaPayError::InvalidBase58Str)?;

        Ok(buffer)
    }

    pub fn is_on_ed25519_curve(bytes: &[u8; 32]) -> SolanaPayResult<bool> {
        Ok(
            curve25519_dalek::edwards::CompressedEdwardsY::from_slice(bytes)
                .map_err(|_| SolanaPayError::InvalidEd25519PublicKey)?
                .decompress()
                .is_some(),
        )
    }
}

#[cfg(test)]
mod test_utils {
    use crate::Utils;

    #[test]
    fn test_valid_base58() {
        let address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
        assert!(Utils::parse_public_key(address).is_ok());
    }

    #[test]
    fn test_invalid_base58() {
        let address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DAA";
        assert!(Utils::parse_public_key(address).is_err());
    }

    #[test]
    fn valid_point_on_curve() {
        let address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
        let public_key = Utils::parse_public_key(address).unwrap();

        assert!(Utils::is_on_ed25519_curve(&public_key).is_ok());
        assert!(Utils::is_on_ed25519_curve(&public_key).unwrap());
    }

    #[test]
    fn invalid_point_not_on_curve() {
        let address = "HqAi1JjEEVS6QRvNe7gC4z8pYTuKbWkdZqCuuDpZxxQW";
        let public_key = Utils::parse_public_key(address).unwrap();

        assert!(Utils::is_on_ed25519_curve(&public_key).is_ok());
        assert!(!Utils::is_on_ed25519_curve(&public_key).unwrap());
    }
}
