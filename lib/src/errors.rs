use thiserror::Error;

/// The result type with [SolanaPayError] as the error type
pub type SolanaPayResult<T> = Result<T, SolanaPayError>;

/// The errors returned from operations of the crate.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum SolanaPayError {
    /// The `Amount` from the Solana Pay URL is invalid
    #[error("The `Amount` from the Solana Pay URL is invalid")]
    InvalidNumber,
    /// The Base58 str provided is invalid
    #[error("he Base58 str provided is invalid")]
    InvalidBase58Str,
    /// Invalid Ed25519 public key
    #[error("Invalid Ed25519 public key")]
    InvalidEd25519PublicKey,
    /// The recipient is expected to be on curve
    /// to prevent sending to a PDA without user's
    /// knowledge. Use `new_all_accounts()` if you
    /// want to support all types of recipients
    #[error("The recipient is expected to be on curve to prevent sending to a PDA without user's knowledge. Use `new_any_public_key()` if you want to support all types of recipients")]
    ExpectedRecipientPublicKeyOnCurve,
    /// The number of decimals in a number
    /// exceeds those of Native SOL (9 decimals)
    #[error("The number of decimals in a number exceeds those of Native SOL (9 decimals)")]
    NumberOfDecimalsExceeds9,
    /// The number of decimals in a number
    /// exceeds those configured by the mint
    #[error("The number of decimals in a number exceeds those configured by the mint")]
    NumberOfDecimalsExceedsMintConfiguration,
    /// The capacity left in the references container
    /// is smaller than the references provided
    /// as arguments
    #[error("The capacity left in the references container is smaller than the references provided as arguments")]
    TooManyReferences,
    /// The characters contain Invalid UTF8
    #[error("The characters contain Invalid UTF8")]
    InvalidUrlEncodedString,
    /// Invalid Parameter of a Solana Pay URL
    #[error("Invalid Parameter of a Solana Pay URL")]
    InvalidQueryParam,
    /// Found duplicate amount in a Solana Pay URL
    #[error("Found duplicate amount in a Solana Pay URL")]
    AmountAlreadyExists,
    /// Found duplicate spl-token in a Solana Pay URL
    #[error("Found duplicate spl-token in a Solana Pay URL")]
    SplTokenAlreadyExists,
    /// Found duplicate label in a Solana Pay URL
    #[error("Found duplicate label in a Solana Pay URL")]
    LabelAlreadyExists,
    /// Found duplicate message in a Solana Pay URL
    #[error("Found duplicate message in a Solana Pay URL")]
    MessageAlreadyExists,
    /// Found duplicate memo in a Solana Pay URL
    #[error("Found duplicate memo in a Solana Pay URL")]
    MemoAlreadyExists,
}
