use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};

use crate::SolanaPayError;

/// The scheme of a Solana Pay URL
pub const SOLANA_SCHEME: &str = "solana:";

/// Structure of a Solana Pay URL.
/// **Credit: ** [Solana Pay Docs](https://docs.solanapay.com/spec)
///
/// solana:<recipient>
///     ?amount=<amount>
///     &spl-token=<spl-token>
///     &reference=<reference>
///     &label=<label>
///     &message=<message>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SolanaPayUrl {
    /// A single recipient field is required as the pathname.
    /// The value must be the base58-encoded public key of a native SOL account.
    /// Associated token accounts must not be used.
    /// Instead, to request an SPL Token transfer, the spl-token field must be used to specify an SPL Token mint,
    /// from which the associated token address of the recipient must be derived.
    pub recipient: String,
    /// A single amount field is allowed as an optional query parameter.
    /// The value must be a non-negative integer or decimal number of "user" units.
    /// For SOL, that's SOL and not lamports. For tokens, use uiAmountString and not amount.
    /// 0 is a valid value. If the value is a decimal number less than 1,
    /// it must have a leading `0`` before the `.`. Scientific notation is prohibited.
    /// If a value is not provided, the wallet must prompt the user for the amount.
    /// If the number of decimal places exceed what's supported for SOL (9) or the SPL Token (mint specific),
    /// the wallet must reject the URL as malformed.
    pub amount: Option<String>,
    /// A single spl-token field is allowed as an optional query parameter.
    /// The value must be the base58-encoded public key of an SPL Token mint account.
    /// If the field is provided, the Associated Token Account convention must be used,
    /// and the wallet must include a `TokenProgram.Transfer` or `TokenProgram.TransferChecked` instruction
    /// as the last instruction of the transaction.
    /// If the field is not provided, the URL describes a native SOL transfer,
    /// and the wallet must include a SystemProgram.Transfer instruction as the last instruction of the transaction instead.
    /// The wallet must derive the ATA address from the recipient and spl-token fields.
    /// Transfers to auxiliary token accounts are not supported.
    pub spl_token: Option<String>,
    /// Multiple reference fields are allowed as optional query parameters.
    /// The values must be base58-encoded 32 byte arrays.
    /// These may or may not be public keys, on or off the curve, and may or may not correspond with accounts on Solana.
    /// If the values are provided, the wallet must include them in the order provided as read-only,
    /// non-signer keys to the `SystemProgram.Transfer` or `TokenProgram.Transfer/TokenProgram.TransferChecked`
    /// instruction in the payment transaction. The values may or may not be unique to the payment request,
    /// and may or may not correspond to an account on Solana. Because Solana validators index transactions
    /// by these account keys, reference values can be used as client IDs
    /// (IDs usable before knowing the eventual payment transaction).
    /// The `getSignaturesForAddress` RPC method can be used locate transactions this way.
    pub references: Vec<String>,
    /// A single label field is allowed as an optional query parameter.
    /// The value must be a URL-encoded UTF-8 string that describes the source of the transfer request.
    /// For example, this might be the name of a brand, store, application, or person making the request.
    /// The wallet should URL-decode the value and display the decoded value to the user.
    pub label: Option<String>,
    /// A single message field is allowed as an optional query parameter.
    /// The value must be a URL-encoded UTF-8 string that describes the nature of the transfer request.
    /// For example, this might be the name of an item being purchased, an order ID, or a thank you note.
    /// The wallet should URL-decode the value and display the decoded value to the user.
    pub message: Option<String>,
    /// A single memo field is allowed as an optional query parameter.
    /// The value must be a URL-encoded UTF-8 string that must be included in an SPL Memo instruction in the payment transaction.
    /// The wallet must URL-decode the value and should display the decoded value to the user.
    /// The memo will be recorded by validators and should not include private or sensitive information.
    /// If the field is provided, the wallet must include a MemoProgram instruction as the second to last
    /// instruction of the transaction, immediately before the SOL or SPL Token transfer instruction,
    /// to avoid ambiguity with other instructions in the transaction.
    pub spl_memo: Option<String>,
}

impl SolanaPayUrl {
    /// Instantiate a new url
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse a Solana Pay URL
    pub fn parse(mut self, solana_pay_url: &str) -> Self {
        let decoded = percent_decode_str(solana_pay_url)
            .decode_utf8()
            .unwrap()
            .to_string();

        if !decoded.starts_with(SOLANA_SCHEME) {
            panic!("InvalidSolanaPayScheme");
        }

        let decoded = decoded.split(SOLANA_SCHEME).collect::<Vec<&str>>()[1];

        let first_split = if decoded.contains('?') {
            decoded.split('?').collect::<Vec<&str>>()
        } else {
            decoded.split('&').collect::<Vec<&str>>()
        };

        if let Some(recipient) = first_split.first() {
            self.recipient = recipient.to_string();
        } else {
            panic!("SolanaPayUrlPartsEmpty");
        };

        if first_split.len() > 2 {
            panic!("TooManySolanaPayUrlParts");
        }

        let mut queries = Vec::<&str>::new();
        if let Some(options) = first_split.get(1) {
            options.split("&").for_each(|value| queries.push(value))
        }

        for query in queries {
            let split_query = query.split('=').collect::<Vec<&str>>();
            if split_query.len() != 2 {
                panic!("InvalidQuery");
            }

            let query_param: QueryParam = split_query[0].try_into().unwrap();

            match query_param {
                QueryParam::Amount => Self::parse_query(
                    &mut self.amount,
                    split_query[1],
                    SolanaPayError::AmountAlreadyExists,
                )
                .unwrap(),

                QueryParam::SplToken => Self::parse_query(
                    &mut self.spl_token,
                    split_query[1],
                    SolanaPayError::SplTokenAlreadyExists,
                )
                .unwrap(),

                QueryParam::Reference => self.references.push(split_query[1].into()),

                QueryParam::Label => Self::parse_query(
                    &mut self.label,
                    split_query[1],
                    SolanaPayError::LabelAlreadyExists,
                )
                .unwrap(),

                QueryParam::Message => Self::parse_query(
                    &mut self.message,
                    split_query[1],
                    SolanaPayError::MessageAlreadyExists,
                )
                .unwrap(),

                QueryParam::SplMemo => Self::parse_query(
                    &mut self.spl_memo,
                    split_query[1],
                    SolanaPayError::MemoAlreadyExists,
                )
                .unwrap(),
                QueryParam::Unsupported => panic!("UnsupportedQueryParam"),
            }
        }

        self
    }

    fn parse_query(
        existing_value: &mut Option<String>,
        value: &str,
        error: SolanaPayError,
    ) -> Result<(), SolanaPayError> {
        if existing_value.is_some() {
            Err(error)
        } else {
            existing_value.replace(value.to_string());

            Ok(())
        }
    }

    /// Add a Base58 encoded Ed25519 public key for the recipient
    pub fn add_recipient(mut self, recipient: &str) -> Self {
        self.recipient = recipient.into();

        self
    }

    /// A single amount field is allowed as an optional query parameter.
    /// The value must be a non-negative integer or decimal number of "user" units. For SOL, that's SOL and not lamports.
    pub fn add_amount(mut self, amount: &str) -> Self {
        self.amount.replace(amount.into());

        self
    }

    /// Add a Base58 encoded public key for the mint account
    pub fn add_spl_token(mut self, spl_token: &str) -> Self {
        self.spl_token.replace(spl_token.into());

        self
    }

    /// Multiple reference fields are allowed as optional query parameters. The values must be base58-encoded 32 byte arrays.
    /// These may or may not be public keys, on or off the curve, and may or may not correspond with accounts on Solana.
    /// Because Solana validators index transactions by these account keys,
    /// reference values can be used as client IDs (IDs usable before knowing the eventual payment transaction).
    /// The getSignaturesForAddress RPC method can be used locate transactions this way.
    pub fn add_reference(mut self, reference: &str) -> Self {
        self.references.push(reference.into());

        self.references.dedup();

        self
    }

    /// Same as [SolanaPayUrl::add_reference] above but allows adding multiple references at once
    pub fn add_reference_multiple(mut self, references: &[&str]) -> Self {
        references.iter().for_each(|reference| {
            self.references.push(reference.to_string());
        });

        self.references.dedup();

        self
    }

    /// Add a UTF-8 URL label
    pub fn add_label(mut self, label: &str) -> Self {
        self.label.replace(label.into());

        self
    }

    /// Add a UTF-8 URL message
    pub fn add_message(mut self, message: &str) -> Self {
        self.message.replace(message.into());

        self
    }

    /// Add a UTF-8 URL memo to be included in the SPL memo part of a transaction
    pub fn add_spl_memo(mut self, spl_memo: &str) -> Self {
        self.spl_memo.replace(spl_memo.into());

        self
    }

    /// Convert [Self] to a Solana Pay  URL
    pub fn to_url(&self) -> String {
        String::from(SOLANA_SCHEME)
            + &self.recipient
            + &self.prepare_amount()
            + &self.prepare_spl_token()
            + &self.prepare_references()
            + &self.prepare_label()
            + &self.prepare_message()
            + &self.prepare_spl_memo()
    }

    fn prepare_optional_value_with_encoding(
        &self,
        name: &str,
        optional_value: Option<&String>,
    ) -> String {
        if let Some(value) = optional_value.as_ref() {
            let encoded = utf8_percent_encode(value, NON_ALPHANUMERIC).to_string();

            String::new() + "&" + name + "=" + &encoded
        } else {
            String::default()
        }
    }

    fn prepare_amount(&self) -> String {
        if let Some(amount) = self.amount.as_ref() {
            String::new() + "?" + "amount=" + amount
        } else {
            String::default()
        }
    }

    fn prepare_spl_token(&self) -> String {
        if let Some(spl_token) = self.spl_token.as_ref() {
            String::new() + "&" + "spl-token=" + spl_token
        } else {
            String::default()
        }
    }

    fn prepare_references(&self) -> String {
        let mut outcome = String::default();

        self.references.iter().for_each(|reference| {
            outcome.push('&');
            outcome.push_str("reference=");
            outcome.push_str(reference);
        });

        outcome
    }

    fn prepare_label(&self) -> String {
        self.prepare_optional_value_with_encoding("label", self.label.as_ref())
    }

    fn prepare_message(&self) -> String {
        self.prepare_optional_value_with_encoding("message", self.message.as_ref())
    }

    fn prepare_spl_memo(&self) -> String {
        self.prepare_optional_value_with_encoding("memo", self.spl_memo.as_ref())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum QueryParam {
    Amount,
    SplToken,
    Reference,
    Label,
    Message,
    SplMemo,
    Unsupported,
}

impl TryFrom<&str> for QueryParam {
    type Error = SolanaPayError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let outcome = match value {
            "amount" => Self::Amount,
            "spl-token" => Self::SplToken,
            "reference" => Self::Reference,
            "label" => Self::Label,
            "message" => Self::Message,
            "memo" => Self::SplMemo,
            _ => Self::Unsupported,
        };

        Ok(outcome)
    }
}

#[cfg(test)]
mod url_parsing_checks {
    use crate::*;
    #[test]
    fn parse_1_sol() {
        let transfer_1_sol = "solana:mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN?amount=1&label=Michael&message=Thanks%20for%20all%20the%20fish&memo=OrderId12345";
        let transfer_1_sol_decoded = SolanaPayUrl::new().parse(transfer_1_sol);

        let transfer_1_sol_other = SolanaPayUrl::default()
            .add_recipient("mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN")
            .add_amount("1")
            .add_label("Michael")
            .add_message("Thanks for all the fish")
            .add_spl_memo("OrderId12345");

        assert_eq!(transfer_1_sol_decoded, transfer_1_sol_other);

        let encode_again = transfer_1_sol_decoded.to_url();
        assert_eq!(&encode_again, transfer_1_sol);
    }

    #[test]
    fn parse_spl_token() {
        let zero_zero_one_usdc = "solana:mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN?amount=0.01&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
        let decoded_zero_zero_one_usdc = SolanaPayUrl::new().parse(&zero_zero_one_usdc);

        let zero_zero_one_usdc_other = SolanaPayUrl::default()
            .add_recipient("mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN")
            .add_amount("0.01")
            .add_spl_token("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

        assert_eq!(decoded_zero_zero_one_usdc, zero_zero_one_usdc_other);

        let encode_again = decoded_zero_zero_one_usdc.to_url();
        assert_eq!(&encode_again, zero_zero_one_usdc);
    }

    #[test]
    fn prompt_amount() {
        let prompt_amount = "solana:mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN&label=Michael";
        let decoded_prompt_amount = SolanaPayUrl::new().parse(&prompt_amount);

        let prompt_amount_other = SolanaPayUrl::default()
            .add_recipient("mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN")
            .add_label("Michael");

        assert_eq!(decoded_prompt_amount, prompt_amount_other);

        let encode_again = decoded_prompt_amount.to_url();
        assert_eq!(&encode_again, prompt_amount);
    }

    #[test]
    fn all_fields_encode_decode() {
        let all_fields = "solana:mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN?amount=0.01&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
        let decoded_all_fields = SolanaPayUrl::new()
            .parse(&all_fields)
            .add_reference("7owWEdgJRWpKsiDFNU4qT2kgMe2kitPXem5Yy8VdNatx")
            .add_reference_multiple(&[
                "7owWEdgJRWpKsiDFNU4qT2kgMe2kitPXem5Yy8VdNatx",
                "7owWEdgJRWpKsiDFNU4qT2kgMe2kitPXem5Yy8VdNatx",
                "7owWEdgJRWpKsiDFNU4qT2kgMe2kitPXem5Yy8VdNatx",
            ]);
        assert!(decoded_all_fields.references.len() == 1);

        let decoded_all_fields = decoded_all_fields.add_reference_multiple(&[
            "7owWEdgJRWpKsiDFNU4qT2kgMe2kitPXem5Yy8VdNatx",
            "7owWEdgJRWpKsiDFNU4qT2kgMe2kitPXem5Yy8VdNaty",
            "7owWEdgJRWpKsiDFNU4qT2kgMe2kitPXem5Yy8VdNatz",
        ]);
        assert!(decoded_all_fields.references.len() == 3);
    }
}
