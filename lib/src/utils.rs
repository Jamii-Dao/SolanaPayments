use std::borrow::Cow;

use crate::{SolanaPayError, SolanaPayResult};

pub const NATIVE_SOL_DECIMAL_COUNT: u8 = 9;

pub struct Utils;

impl Utils {
    pub fn from_base58(base58_str: &str) -> SolanaPayResult<[u8; 32]> {
        let mut buffer = [0u8; 32];
        bs58::decode(base58_str)
            .onto(&mut buffer)
            .map_err(|_| SolanaPayError::InvalidBase58Str)?;

        Ok(buffer)
    }

    pub fn to_base58(bytes: impl AsRef<[u8]>) -> String {
        bs58::encode(bytes.as_ref()).into_string()
    }

    pub fn is_on_curve25519(bytes: &[u8; 32]) -> SolanaPayResult<bool> {
        Ok(
            curve25519_dalek::edwards::CompressedEdwardsY::from_slice(bytes)
                .map_err(|_| SolanaPayError::InvalidEd25519PublicKey)?
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
}

pub struct RandomBytes<const N: usize>([u8; N]);

impl<const N: usize> RandomBytes<N> {
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

    pub fn expose(&self) -> &[u8; N] {
        &self.0
    }

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
