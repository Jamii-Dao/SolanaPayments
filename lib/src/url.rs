use crate::{Number, SolanaPayError, SolanaPayResult};

pub struct SolanaPayUrl<'pay> {
    recipient: &'pay str,
    amount: Number,
    spl_token: &'pay str,
    reference: &'pay str,
    label: &'pay str,
    message: &'pay str,
    memo: &'pay str,
}

impl<'pay> SolanaPayUrl<'pay> {
    pub fn recipient(&self) -> &str {
        &self.recipient
    }

    pub fn amount(&self) -> &Number {
        &self.amount
    }

    pub fn spl_token(&self) -> &str {
        &self.spl_token
    }

    pub fn reference(&self) -> &str {
        &self.reference
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn memo(&self) -> &str {
        &self.memo
    }
}
