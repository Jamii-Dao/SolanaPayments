use core::fmt;

use crate::{RandomBytes, SolanaPayResult, Utils};

/// A Reference field as defined by the [Solana Pay Spec](https://docs.solanapay.com/spec#reference)
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Reference([u8; 32]);

impl Reference {
    /// Generate a new unique reference in case the reference just needs to be
    /// a random 32 byte value instead on a valid public key with a corresponding private key.
    pub fn new() -> Self {
        let random = RandomBytes::new();

        Self(random.expose_owned())
    }

    /// Generate a Blake3 hash of the reference
    pub fn to_hash(&self) -> blake3::Hash {
        blake3::hash(&self.0)
    }

    /// Get the 32 byte array representation of a [Reference]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    /// Get the 32 byte array representation of a [Reference]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Convert the bytes to a Base58 encoded [String]
    pub fn to_base58(&self) -> String {
        Utils::to_base58(self.0)
    }

    /// Convert a [str] of Base58 encoded characters to a [Reference]
    pub fn from_base58(base58_str: &str) -> SolanaPayResult<Self> {
        let outcome = Utils::from_base58(base58_str)?;

        Ok(Self(outcome))
    }
}

impl fmt::Debug for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reference({})", self.to_hash())
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hash().to_hex().as_str())
    }
}

impl AsRef<[u8]> for Reference {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Default for Reference {
    fn default() -> Self {
        Self::new()
    }
}
