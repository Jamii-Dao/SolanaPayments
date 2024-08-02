use std::{borrow::Cow, future::Future};

use arrayvec::ArrayVec;

use crate::{
    Number, PublicKey, Reference, SolanaPayError, SolanaPayResult, Utils, NATIVE_SOL_DECIMAL_COUNT,
};

// TODO Ensure URL size is not greater that a few KBs or 1MiB or add config
// TODO Create program derived addresses
#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub struct SolanaPayment<'a, const N: usize> {
    recipient: PublicKey,
    amount: Option<Number>,
    spl_token: Option<PublicKey>,
    references: ArrayVec<Reference, N>,
    label: Option<Cow<'a, str>>,
    message: Option<Cow<'a, str>>,
    spl_memo: Option<Cow<'a, str>>,
}

impl<'a, const N: usize> SolanaPayment<'a, N> {
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
    pub fn new_all_accounts(base58_public_key: &str) -> SolanaPayResult<Self> {
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

    pub fn add_reference(&mut self, reference: Reference) -> SolanaPayResult<&mut Self> {
        self.references
            .try_push(reference)
            .map_err(|_| SolanaPayError::TooManyReferences)?;

        Ok(self)
    }

    pub fn add_multiple_references(
        &mut self,
        references: &[Reference],
    ) -> SolanaPayResult<&mut Self> {
        self.references
            .try_extend_from_slice(references)
            .map_err(|_| SolanaPayError::TooManyReferences)?;

        Ok(self)
    }

    pub fn add_label(&mut self, label: &'a str) -> SolanaPayResult<&mut Self> {
        let label = Utils::url_decode(label)?;
        self.label.replace(label);

        Ok(self)
    }

    pub fn add_message(&mut self, message: &'a str) -> SolanaPayResult<&mut Self> {
        let message = Utils::url_decode(message)?;
        self.message.replace(message);

        Ok(self)
    }

    /// This is recorded in the SPL memo part of the transaction which is recorded onchain
    /// and therefore SHOULD NOT contain any sensitive information
    pub fn add_spl_memo(&mut self, spl_memo: &'a str) -> SolanaPayResult<&mut Self> {
        let spl_memo = Utils::url_decode(spl_memo)?;
        self.spl_memo.replace(spl_memo);

        Ok(self)
    }

    pub fn is_associated_account(&self) -> bool {
        self.spl_token.is_some()
    }
}
