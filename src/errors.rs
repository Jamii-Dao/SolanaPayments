use bs58::decode::Error as Base58DecodeError;

/// The common `Result` type using `PayError` as it's error field
pub type PayResult<T> = Result<T, PayError>;

/// Errors encountered while using this crate
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PayError {
    /// The string provided to convert to Ed25519 Verifying Key is invalid
    InvalidEd25519Base58Key,
    /// The Ed25519 Key provided does not lie on the curve
    Ed25519KeyMustLieOnCurve,
    /// The Ed25519 Key provided lies on the curve but the required
    /// one must not lie on the curve eg for a PDA address
    Ed25519KeyMustNotLieOnCurve,
    /// The base58 string provided is invalid
    Base58(Base58DecodeError),
    /// Expected an array or slice of length 32 bytes
    ExpectedLengthOf32Bytes,
}

impl From<Base58DecodeError> for PayError {
    fn from(value: Base58DecodeError) -> Self {
        PayError::Base58(value)
    }
}
