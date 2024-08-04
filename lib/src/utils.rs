use std::borrow::Cow;

use crate::{SolanaPayError, SolanaPayResult};

pub(crate) struct Utils;

impl Utils {
    pub(crate) fn from_base58(base58_str: &str) -> SolanaPayResult<[u8; 32]> {
        let mut buffer = [0u8; 32];
        bs58::decode(base58_str)
            .onto(&mut buffer)
            .map_err(|_| SolanaPayError::InvalidBase58Str)?;

        Ok(buffer)
    }

    pub(crate) fn to_base58(bytes: impl AsRef<[u8]>) -> String {
        bs58::encode(bytes.as_ref()).into_string()
    }

    pub(crate) fn is_on_curve25519(bytes: &[u8; 32]) -> SolanaPayResult<bool> {
        Ok(
            curve25519_dalek::edwards::CompressedEdwardsY::from_slice(bytes)
                .decompress()
                .is_some(),
        )
    }

    pub fn url_decode(value: &str) -> SolanaPayResult<Cow<str>> {
        percent_encoding::percent_decode_str(value)
            .decode_utf8()
            .map_err(|_| SolanaPayError::InvalidUrlEncodedString)
    }

    pub fn url_encode(value: &str) -> String {
        percent_encoding::utf8_percent_encode(value, percent_encoding::NON_ALPHANUMERIC).to_string()
    }

    pub async fn native_sol(_value: [u8; 32]) -> u8 {
        9
    }
}

/// Random bytes generator that generates an array of bytes of length defined by
/// `const N: usize` where `N` is the number of bytes to generate.
/// This struct implements `Zeroize` so the memory region is automatically
/// zeroes out when the [RandomBytes] goes out of scope.
/// The random number generator is a CSPRNG generating entropy from the OS random number generator.
pub struct RandomBytes<const N: usize>([u8; N]);

impl<const N: usize> RandomBytes<N> {
    /// Generate random bytes of length defined by `N`
    pub fn new() -> Self {
        use rand_chacha::ChaCha20Rng;
        use rand_core::{RngCore, SeedableRng};

        let mut rng = ChaCha20Rng::from_entropy();
        let mut buffer = [0u8; N];
        rng.fill_bytes(&mut buffer);

        let outcome = Self(buffer);

        buffer.fill(0);

        outcome
    }

    /// Expose the bytes as a byte array
    pub fn expose(&self) -> &[u8; N] {
        &self.0
    }

    /// Clone the bytes and return the byte array
    pub fn expose_owned(&self) -> [u8; N] {
        self.0
    }
}

impl<const N: usize> core::fmt::Debug for RandomBytes<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RandomBytes(REDACTED)").finish()
    }
}

impl<const N: usize> core::fmt::Display for RandomBytes<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RandomBytes(REDACTED)").finish()
    }
}

impl<const N: usize> Drop for RandomBytes<N> {
    fn drop(&mut self) {
        use zeroize::Zeroize;

        self.zeroize()
    }
}

impl<const N: usize> zeroize::Zeroize for RandomBytes<N> {
    fn zeroize(&mut self) {
        self.0.fill(0);

        assert_eq!(self.0, [0u8; N]); //Must panic if memory cannot be zeroized
    }
}

impl<const N: usize> Default for RandomBytes<N> {
    fn default() -> Self {
        Self::new()
    }
}
