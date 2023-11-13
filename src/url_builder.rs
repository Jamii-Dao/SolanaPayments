use crate::{PayError, PayResult, PayUtils};
use arrayvec::ArrayVec;
use core::str;

/// A 32 byte array
pub type Byte32Array = [u8; 32];

/// A Url Builder for Solana pay with checks for public key fields,
/// The number of references must be added or an error will occur showing that the
/// number of references in invalid.
#[derive(Clone)]
pub struct SolanaPay<'p, const N: usize> {
    recipient: [u8; 32],
    amount: u64,
    spl_token: &'p str,
    reference: ArrayVec<Byte32Array, N>,
    label: &'p str,
    message: &'p str,
    memo: &'p str,
}

impl<'p, const N: usize> SolanaPay<'p, N> {
    /// Initialize the structure using the Recipient Ed25519 Public Key 32 bytes
    /// without caring whether the bytes lie on the curve
    pub fn new(recipient: [u8; 32]) -> Self {
        Self {
            recipient,
            amount: u64::default(),
            spl_token: "",
            reference: ArrayVec::<Byte32Array, N>::new(),
            label: "",
            message: "",
            memo: "",
        }
    }

    /// Initialize the structure using the Recipient Base58 encoded Ed25519 Public Key
    /// without caring whether the Public Key lies on the Edwards Twisted curve
    pub fn new_from_base64(recipient_base58_address: &str) -> PayResult<Self> {
        let mut recipient = [0u8; 32];
        bs58::decode(recipient_base58_address).onto(&mut recipient)?;

        Ok(Self {
            recipient,
            amount: u64::default(),
            spl_token: "",
            reference: ArrayVec::<Byte32Array, N>::new(),
            label: "",
            message: "",
            memo: "",
        })
    }

    /// Initialize the structure using the Recipient Ed25519 Public Key 32 bytes
    /// ensuring that the Public Key lies on the Edwards Twisted curve
    pub fn new_with_curve(recipient: [u8; 32]) -> PayResult<Self> {
        if PayUtils::on_edwards_curve(&recipient)? {
            Ok(Self {
                recipient,
                amount: u64::default(),
                spl_token: "",
                reference: ArrayVec::<Byte32Array, N>::new(),
                label: "",
                message: "",
                memo: "",
            })
        } else {
            Err(PayError::Ed25519KeyMustLieOnCurve)
        }
    }

    /// Initialize the structure using the Recipient Base58 encoded Ed25519 Public Key
    /// ensuring that the Public Key lies on the Edwards Twisted curve
    pub fn new_from_base64_with_curve(recipient_base58_address: &str) -> PayResult<Self> {
        let mut recipient = [0u8; 32];
        bs58::decode(recipient_base58_address).onto(&mut recipient)?;

        if PayUtils::on_edwards_curve(&recipient)? {
            Ok(Self {
                recipient,
                amount: u64::default(),
                spl_token: "",
                reference: ArrayVec::<Byte32Array, N>::new(),
                label: "",
                message: "",
                memo: "",
            })
        } else {
            Err(PayError::Ed25519KeyMustLieOnCurve)
        }
    }

    /// Initialize the structure using the Recipient Ed25519 Public Key 32 bytes
    /// ensuring that the Public Key does not lie on the Edwards Twisted curve
    pub fn new_without_curve(recipient: [u8; 32]) -> PayResult<Self> {
        if !PayUtils::on_edwards_curve(&recipient)? {
            Ok(Self {
                recipient,
                amount: u64::default(),
                spl_token: "",
                reference: ArrayVec::<Byte32Array, N>::new(),
                label: "",
                message: "",
                memo: "",
            })
        } else {
            Err(PayError::Ed25519KeyMustNotLieOnCurve)
        }
    }

    /// Initialize the structure using the Recipient Ed25519 Public Key
    /// ensuring that the Public Key does not lie on the Edwards Twisted curve
    pub fn new_from_base64_without_curve(recipient_base58_address: &str) -> PayResult<Self> {
        let mut recipient = [0u8; 32];
        bs58::decode(recipient_base58_address).onto(&mut recipient)?;

        if !PayUtils::on_edwards_curve(&recipient)? {
            Ok(Self {
                recipient,
                amount: u64::default(),
                spl_token: "",
                reference: ArrayVec::<Byte32Array, N>::new(),
                label: "",
                message: "",
                memo: "",
            })
        } else {
            Err(PayError::Ed25519KeyMustNotLieOnCurve)
        }
    }
}
