use crate::{SolanaPayError, SolanaPayResult};

/// The a number comprising of an integral and fractional part
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Number {
    /// The integral part of a number with a fraction
    pub integral: u64,
    /// The fractional part of a number with a fraction
    pub fractional: Option<u64>,
    /// Number of fractional values
    pub fractional_count: usize,
}

impl Number {
    /// Parse a number that may contain a fractional part
    pub fn parse(str_number: &str) -> SolanaPayResult<Self> {
        if !str_number.contains('.') {
            return Ok(Self {
                integral: str_number
                    .parse::<u64>()
                    .map_err(|_| SolanaPayError::InvalidNumber)?,
                ..Default::default()
            });
        }

        let (str_integral, str_fractional) = {
            let mut iter_str_number = str_number.split('.');
            if iter_str_number.clone().nth(2).is_some() {
                return Err(SolanaPayError::InvalidNumber);
            }

            let str_integral = iter_str_number
                .next()
                .ok_or(SolanaPayError::InvalidNumber)?;

            let str_fractional = iter_str_number
                .next()
                .ok_or(SolanaPayError::InvalidNumber)?;

            (str_integral, str_fractional)
        };

        let integral = str_integral
            .parse::<u64>()
            .map_err(|_| SolanaPayError::InvalidNumber)?;

        let fractional = str_fractional
            .parse::<u64>()
            .map_err(|_| SolanaPayError::InvalidNumber)?;

        let fractional = if fractional == 0 {
            None
        } else {
            Some(fractional)
        };

        let fractional_count = Self::fractional_count(fractional);

        Ok(Self {
            integral,
            fractional,
            fractional_count,
        })
    }

    pub fn fractional_count(fractional_value: Option<u64>) -> usize {
        let mut count = 0usize;

        if let Some(value_exists) = fractional_value {
            let mut value = value_exists;

            while value != 0 {
                count += 1;
                value /= 10;
            }
        }

        count
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
                fractional: None,
                fractional_count: 0
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
                fractional: Some(1),
                fractional_count: 1
            }
        );
    }

    #[test]
    fn empty_str() {
        let foo = "";

        let parsed = Number::parse(foo);

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidNumber));
    }

    #[test]
    fn integral_decimal_point() {
        let foo = "1.";

        let parsed = Number::parse(foo);

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidNumber));
    }

    #[test]
    fn fractional_decimal_point() {
        let foo = ".1";

        let parsed = Number::parse(foo);

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidNumber));
    }

    #[test]
    fn decimal_point() {
        let foo = ".";

        let parsed = Number::parse(foo);

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidNumber));
    }

    #[test]
    fn three_decimal_points() {
        let foo = "1.1.";

        let parsed = Number::parse(foo);

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidNumber));
    }

    #[test]
    fn three_decimal_points_and_number() {
        let foo = "1.1.1";

        let parsed = Number::parse(foo);

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidNumber));
    }

    #[test]
    fn count_is_valid() {
        let fractional = Some(1u64);
        let count = Number::fractional_count(fractional);

        assert_eq!(count, 1);

        let fractional = Some(146_785u64);
        let count = Number::fractional_count(fractional);

        assert_eq!(count, 6);

        let fractional = Some(146_780u64);
        let count = Number::fractional_count(fractional);

        assert_eq!(count, 6);
    }
}
