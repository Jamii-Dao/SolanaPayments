use core::fmt;
use std::hash::Hash;

use constant_time_eq::constant_time_eq_n;

use crate::{RandomBytes, SolanaPayResult, Utils};

#[derive(Clone, Copy)]
pub struct Reference([u8; 32]);

impl Reference {
    pub fn new() -> Self {
        let random = RandomBytes::new();

        Self(random.expose_owned())
    }

    pub fn to_hash(&self) -> blake3::Hash {
        blake3::hash(&self.0)
    }

    pub fn expose_owned(&self) -> [u8; 32] {
        self.0
    }

    pub fn expose(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_base58(&self) -> String {
        Utils::to_base58(&self.0)
    }

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

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        constant_time_eq_n(self.expose(), other.expose())
    }
}

impl Eq for Reference {}

impl AsRef<[u8]> for Reference {
    fn as_ref(&self) -> &[u8] {
        self.expose()
    }
}

impl Default for Reference {
    fn default() -> Self {
        Self::new()
    }
}

impl Hash for Reference {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.expose().iter().for_each(|byte| {
            state.write_u8(*byte);
        });
    }
}
