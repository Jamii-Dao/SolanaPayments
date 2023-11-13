use crate::{PayError, PayResult};
use curve25519_dalek::edwards::CompressedEdwardsY;

/// Utilities used in this crate
pub struct PayUtils;

impl PayUtils {
    /// Check if bytes gives lie on the Twisted Edwards Curve returning `Ok(true)`
    /// if the bytes lie on the curve, `Ok(false)` if they don't or
    /// `Err(PayError::ExpectedLengthOf32Bytes)` if the length of the slice is not 32 bytes
    pub fn on_edwards_curve(bytes: &[u8]) -> PayResult<bool> {
        match CompressedEdwardsY::from_slice(&bytes) {
            Ok(outcome) => Ok(outcome.decompress().is_some()),
            Err(_) => Err(PayError::ExpectedLengthOf32Bytes),
        }
    }
}
