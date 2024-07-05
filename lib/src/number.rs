use crate::{SolanaPayError, SolanaPayResult};

/// The a number comprising of an integral and fractional part
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Number {
    /// The integral part of a number with a fraction
    pub integral: u64,
    /// The fractional part of a number with a fraction
    pub fractional: Option<u64>,
}

impl Number {
    /// Parse a number that may contain a fractional part
    pub fn parse(str_number: &str) -> SolanaPayResult<Self> {
        if !str_number.contains(".") {
            return Ok(Self {
                integral: str_number
                    .parse::<u64>()
                    .map_err(|_| SolanaPayError::InvalidRecipientAmount)?,
                fractional: None,
            });
        }

        let (str_integral, str_fractional) = {
            let mut iter_str_number = str_number.split(".");
            if iter_str_number.clone().skip(2).next().is_some() {
                return Err(SolanaPayError::InvalidRecipientAmount);
            }

            let str_integral = iter_str_number
                .next()
                .ok_or(SolanaPayError::InvalidRecipientAmount)?;

            let str_fractional = iter_str_number
                .next()
                .ok_or(SolanaPayError::InvalidRecipientAmount)?;

            (str_integral, str_fractional)
        };

        let integral = str_integral
            .parse::<u64>()
            .map_err(|_| SolanaPayError::InvalidRecipientAmount)?;

        let fractional = str_fractional
            .parse::<u64>()
            .map_err(|_| SolanaPayError::InvalidRecipientAmount)?;

        let fractional = if fractional == 0 {
            None
        } else {
            Some(fractional)
        };

        Ok(Self {
            integral,
            fractional,
        })
    }
}

#[cfg(test)]
mod test_number_sanity {
    use crate::Number;

    #[test]
    fn integral_only() {
        let foo = "1";

        let parsed = Number::parse(foo).unwrap();

        assert_eq!(
            parsed,
            Number {
                integral: 1,
                fractional: None
            }
        );
    }

    #[test]
    fn fractional_only() {
        let foo = "0.1";

        let parsed = Number::parse(foo).unwrap();

        assert_eq!(
            parsed,
            Number {
                integral: 0,
                fractional: Some(1)
            }
        );
    }

    #[test]
    fn empty_str() {
        let foo = "";

        let parsed = Number::parse(foo);

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidRecipientAmount));
    }

    #[test]
    fn integral_decimal_point() {
        let foo = "1.";

        let parsed = Number::parse(foo);

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidRecipientAmount));
    }

    #[test]
    fn fractional_decimal_point() {
        let foo = ".1";

        let parsed = Number::parse(foo);

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidRecipientAmount));
    }

    #[test]
    fn decimal_point() {
        let foo = ".";

        let parsed = Number::parse(foo);

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidRecipientAmount));
    }

    #[test]
    fn three_decimal_points() {
        let foo = "1.1.";

        let parsed = Number::parse(foo);

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidRecipientAmount));
    }

    #[test]
    fn three_decimal_points_and_number() {
        let foo = "1.1.1";

        let parsed = Number::parse(foo);

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidRecipientAmount));
    }
}
