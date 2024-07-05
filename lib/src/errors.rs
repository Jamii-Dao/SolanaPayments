/// The result type with [SolanaPayError] as the error type
pub type SolanaPayResult<T> = Result<T, SolanaPayError>;

/// The errors returned from operations of the crate.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SolanaPayError {
    /// The `Amount` from the URL is invalid
    InvalidRecipientAmount,
    /// The Base58 str provided is invalid
    InvalidBase58Str,
}
