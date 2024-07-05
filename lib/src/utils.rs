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
}
