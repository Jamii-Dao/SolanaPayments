use std::{borrow::Cow, future::Future};

use arrayvec::ArrayVec;

use crate::{
    Number, PublicKey, Reference, SolanaPayError, SolanaPayResult, Utils, NATIVE_SOL_DECIMAL_COUNT,
};

// TODO Create program derived addresses

/// A Solana Payment URL struct representation allowing
/// conversion to and from a Solana Pay URL
/// #### Structure
/// ```rust
/// #[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
/// pub struct SolanaPayment<'a, const N: usize> {
///     recipient: PublicKey,
///     amount: Option<Number>,
///     spl_token: Option<PublicKey>,
///     references: ArrayVec<Reference, N>,
///     label: Option<Cow<'a, str>>,
///     message: Option<Cow<'a, str>>,
///     spl_memo: Option<Cow<'a, str>>,
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub struct SolanaPayment<'a, const N: usize> {
    /// An Ed25519 public key of a recipient as defined by [Solana Pay Spec](https://docs.solanapay.com/spec#recipient)
    pub recipient: PublicKey,
    /// An amount as defined by [Solana Pay Spec](https://docs.solanapay.com/spec#amount)
    pub amount: Option<Number>,
    /// A SPL Token Public Key as defined by [Solana Pay Spec](https://docs.solanapay.com/spec#spl-token)
    pub spl_token: Option<PublicKey>,
    /// One or multiple references as defined by [Solana Pay Spec](https://docs.solanapay.com/spec#reference)
    pub references: ArrayVec<Reference, N>,
    /// A label as defined by [Solana Pay Spec](https://docs.solanapay.com/spec#label)
    pub label: Option<Cow<'a, str>>,
    /// A Message as defined by [Solana Pay Spec](https://docs.solanapay.com/spec#message)
    pub message: Option<Cow<'a, str>>,
    /// A SPL memo as defined by [Solana Pay Spec](https://docs.solanapay.com/spec#memo)
    pub spl_memo: Option<Cow<'a, str>>,
}

impl<'a, const N: usize> SolanaPayment<'a, N> {
    /// Instantiate a new struct with a Base58 encoded public key of the recipient.
    /// This returns an error if the Base58 encoded public key does not lie
    /// on the Curve25519 curve, i.e. The Public Key does not have a corresponding
    /// private key. To use any public key use the method [Self::new_any_public_key] instead
    pub fn new(base58_public_key: &str) -> SolanaPayResult<Self> {
        let recipient = PublicKey::from_base58(base58_public_key)?;

        if !recipient.is_on_ed25519_curve()? {
            return Err(SolanaPayError::ExpectedRecipientPublicKeyOnCurve);
        }

        Ok(Self {
            recipient,
            ..Default::default()
        })
    }

    /// Instantiate a new struct with a Base58 encoded public key of the recipient.
    /// The Base58 encoded public key may or may not lie on the curve defined by Curve25519
    /// which means this can be a Solana PDA account. To enforce that the recipient
    /// is a public key that lies on the curve or has a corresponding private key use [Self::new]
    pub fn new_any_public_key(base58_public_key: &str) -> SolanaPayResult<Self> {
        let recipient = PublicKey::from_base58(base58_public_key)?;

        Ok(Self {
            recipient,
            ..Default::default()
        })
    }

    /// Add native SOL amount
    pub fn add_amount(&mut self, amount: Number) -> SolanaPayResult<&mut Self> {
        if amount.fractional_count > NATIVE_SOL_DECIMAL_COUNT as usize {
            return Err(SolanaPayError::NumberOfDecimalsExceeds9);
        }

        self.amount.replace(amount);

        Ok(self)
    }

    /// Add amount of an SPL token
    pub fn add_spl_token_amount_sync(
        &mut self,
        amount: Number,
        lookup_fn: fn([u8; 32]) -> usize,
    ) -> SolanaPayResult<&mut Self> {
        let mint_decimals = lookup_fn(self.recipient.to_bytes());

        if amount.fractional_count > mint_decimals {
            return Err(SolanaPayError::NumberOfDecimalsExceedsMintConfiguration);
        }

        self.amount.replace(amount);

        Ok(self)
    }

    /// Add amount of an SPL token but using an async
    /// lookup function
    pub async fn add_spl_token_amount<
        F: Fn([u8; 32]) -> Fut,
        Fut: Future<Output = usize> + Send + 'static + Sync,
    >(
        &mut self,
        amount: Number,
        lookup_fn: F,
    ) -> SolanaPayResult<&mut Self> {
        let mint_decimals = lookup_fn(self.recipient.to_bytes()).await;

        if amount.fractional_count > mint_decimals {
            return Err(SolanaPayError::NumberOfDecimalsExceedsMintConfiguration);
        }

        self.amount.replace(amount);

        Ok(self)
    }

    /// Add a [Reference]
    pub fn add_reference(&mut self, reference: Reference) -> SolanaPayResult<&mut Self> {
        self.references
            .try_push(reference)
            .map_err(|_| SolanaPayError::TooManyReferences)?;

        Ok(self)
    }

    /// Add multiple [Reference]s
    pub fn add_multiple_references(
        &mut self,
        references: &[Reference],
    ) -> SolanaPayResult<&mut Self> {
        self.references
            .try_extend_from_slice(references)
            .map_err(|_| SolanaPayError::TooManyReferences)?;

        Ok(self)
    }

    /// Add a UTF-8 encoded label
    pub fn add_label(&mut self, label: &'a str) -> SolanaPayResult<&mut Self> {
        let label = Utils::url_decode(label)?;
        self.label.replace(label);

        Ok(self)
    }

    /// Add a UTF-8 encoded message
    pub fn add_message(&mut self, message: &'a str) -> SolanaPayResult<&mut Self> {
        let message = Utils::url_decode(message)?;
        self.message.replace(message);

        Ok(self)
    }

    /// Add a UTF-8 encoded `Memo` for an SPL Token transfer
    /// This is recorded in the SPL memo part of the transaction which is recorded onchain
    /// and therefore SHOULD NOT contain any sensitive information.
    pub fn add_spl_memo(&mut self, spl_memo: &'a str) -> SolanaPayResult<&mut Self> {
        let spl_memo = Utils::url_decode(spl_memo)?;
        self.spl_memo.replace(spl_memo);

        Ok(self)
    }

    /// Check if an associated token account should be generated as the destination
    pub fn is_associated_account(&self) -> bool {
        self.spl_token.is_some()
    }
}
