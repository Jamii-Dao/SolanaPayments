use core::fmt;

use crate::{SolanaPayResult, Utils};

/// An Ed25519 Public key that may or may not be on the curve defined by Curve25519
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct PublicKey(pub [u8; 32]);

impl PublicKey {
    /// Convert a Base58 encoded [str] to a [PublicKey]
    pub fn from_base58(base58_str: &str) -> SolanaPayResult<Self> {
        let outcome = Utils::from_base58(base58_str)?;

        Ok(Self(outcome))
    }

    /// Convert a [PublicKey] to Base58 encoded [String]
    pub fn to_base58(&self) -> String {
        Utils::to_base58(self.0)
    }

    /// Check whether the [PublicKey] lies on the curve defined by Curve25519
    pub fn is_on_ed25519_curve(&self) -> SolanaPayResult<bool> {
        Utils::is_on_curve25519(&self.0)
    }

    /// Convert [PublicKey] to a 32 byte array
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    /// Convert [PublicKey] to a 32 byte array and return a slice
    pub fn as_slice(&self) -> &[u8] {
        self.as_ref()
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

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
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
