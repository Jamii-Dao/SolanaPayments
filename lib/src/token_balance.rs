use crate::{Number, PublicKey};

/// Information on the token account balance
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct TokenBalance {
    /// Index of the account in which the token balance is provided for
    pub account_index: u8,
    /// Pubkey of the token's mint
    pub mint: PublicKey,
    /// Pubkey of token balance's owner
    pub owner: Option<PublicKey>,
    ///  Pubkey of the Token program that owns the account
    pub program_id: Option<PublicKey>,
    pub ui_token_amount: UiTokenAmount,
}

/// Token amount accounting for decimals
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct UiTokenAmount {
    /// Raw amount of tokens as a string, ignoring decimals.
    pub amount: u64,
    /// Number of decimals configured for token's mint.
    pub decimals: u8,
    /// Token amount as a string, accounting for decimals.
    pub ui_amount_string: Option<Number>,
}
