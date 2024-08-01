use core::fmt;

use crate::{SolanaPayError, SolanaPayResult};

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct PublicKey(pub [u8; 32]);

impl PublicKey {
    pub fn from_base58(base58_str: &str) -> SolanaPayResult<Self> {
        let mut buffer = [0u8; 32];
        bs58::decode(base58_str)
            .onto(&mut buffer)
            .map_err(|_| SolanaPayError::InvalidBase58Str)?;

        Ok(Self(buffer))
    }

    pub fn to_base58(&self) -> String {
        bs58::encode(&self.0).into_string()
    }

    pub fn is_on_ed25519_curve(&self) -> SolanaPayResult<bool> {
        Ok(
            curve25519_dalek::edwards::CompressedEdwardsY::from_slice(&self.0)
                .map_err(|_| SolanaPayError::InvalidEd25519PublicKey)?
                .decompress()
                .is_some(),
        )
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey({})", &self.to_base58())
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_base58())
    }
}

#[cfg(test)]
mod test_pubkey {
    use crate::PublicKey;

    #[test]
    fn test_valid_base58() {
        let address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
        assert!(PublicKey::from_base58(address).is_ok());
    }

    #[test]
    fn test_invalid_base58() {
        let address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DAA";
        assert!(PublicKey::from_base58(address).is_err());
    }

    #[test]
    fn valid_point_on_curve() {
        let address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
        let public_key = PublicKey::from_base58(address).unwrap();

        assert!(PublicKey::is_on_ed25519_curve(&public_key).is_ok());
        assert!(PublicKey::is_on_ed25519_curve(&public_key).unwrap());
    }

    #[test]
    fn invalid_point_not_on_curve() {
        let address = "HqAi1JjEEVS6QRvNe7gC4z8pYTuKbWkdZqCuuDpZxxQW";
        let public_key = PublicKey::from_base58(address).unwrap();

        assert!(PublicKey::is_on_ed25519_curve(&public_key).is_ok());
        assert!(!PublicKey::is_on_ed25519_curve(&public_key).unwrap());
    }
}
