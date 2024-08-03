use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};

use crate::SolanaPayError;

pub const SOLANA_SCHEME: &str = "solana:";

/// Structure of a Solana Pay URL
///
/// solana:<recipient>
///     ?amount=<amount>
///     &spl-token=<spl-token>
///     &reference=<reference>
///     &label=<label>
///     &message=<message>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SolanaPayUrl {
    pub recipient: String,
    pub amount: Option<String>,
    pub spl_token: Option<String>,
    pub references: Vec<String>,
    pub label: Option<String>,
    pub message: Option<String>,
    pub spl_memo: Option<String>,
}

impl SolanaPayUrl {
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn add_recipient(mut self, recipient: &str) -> Self {
        self.recipient = recipient.into();

        self
    }

    pub fn add_amount(mut self, amount: &str) -> Self {
        self.amount.replace(amount.into());

        self
    }

    pub fn add_spl_token(mut self, spl_token: &str) -> Self {
        self.spl_token.replace(spl_token.into());

        self
    }

    pub fn add_reference(mut self, reference: &str) -> Self {
        self.references.push(reference.into());

        self.references.dedup();

        self
    }

    pub fn add_reference_multiple(mut self, references: &[&str]) -> Self {
        references.iter().for_each(|reference| {
            self.references.push(reference.to_string());
        });

        self.references.dedup();

        self
    }

    pub fn add_label(mut self, label: &str) -> Self {
        self.label.replace(label.into());

        self
    }

    pub fn add_message(mut self, message: &str) -> Self {
        self.message.replace(message.into());

        self
    }

    pub fn add_spl_memo(mut self, spl_memo: &str) -> Self {
        self.spl_memo.replace(spl_memo.into());

        self
    }

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
pub enum QueryParam {
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
