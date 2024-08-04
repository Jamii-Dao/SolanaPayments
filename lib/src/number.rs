use crate::{SolanaPayError, SolanaPayResult};

/// Parse a number that can a fractional part.
#[derive(Debug, PartialEq, Default, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Number<'a> {
    /// The integral part of the number
    pub integral: usize,
    /// The fractional part of the number
    pub fractional: usize,
    /// The number of leading zeroes in the number
    pub leading_zeroes: usize,
    /// Number of significant digits  
    pub significant_digits_count: usize,
    /// The string representation of the number
    pub as_string: &'a str,
    /// The total count of the significant fractional part and leading zeroes
    pub total_fractional_count: usize,
}

impl<'a> Number<'a> {
    /// instantiate the struct with the [str] representation of the number
    pub fn new(str_number: &'a str) -> Self {
        Self {
            as_string: str_number,
            ..Default::default()
        }
    }

    /// Parse a number that may contain a fractional part
    pub fn parse(mut self) -> SolanaPayResult<Self> {
        let convert_integral = |integral: &str| {
            integral
                .parse::<usize>()
                .map_err(|_| SolanaPayError::InvalidNumber)
        };
        if !self.as_string.contains('.') {
            self.integral = convert_integral(self.as_string)?;

            return Ok(self);
        }

        let (str_integral, str_fractional) = {
            let mut iter_str_number = self.as_string.split('.');
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

        self.integral = convert_integral(str_integral)?;

        self.fractional_ops(str_fractional)?;

        self.total_fractional_count = self.leading_zeroes + self.significant_digits_count;

        Ok(self)
    }

    fn fractional_ops(&mut self, fractional_str: &str) -> SolanaPayResult<&mut Self> {
        let leading_zeroes_count = fractional_str
            .chars()
            .take_while(|char| char == &'0')
            .count();

        self.leading_zeroes = leading_zeroes_count;

        let non_zero_count = fractional_str.chars().skip(leading_zeroes_count).count();
        self.significant_digits_count = non_zero_count;

        self.fractional = fractional_str
            .parse::<usize>()
            .map_err(|_| SolanaPayError::InvalidNumber)?;

        Ok(self)
    }
}

#[cfg(test)]
mod test_number_sanity {
    use crate::Number;

    #[test]
    fn integral_only() {
        let foo = "1";

        let parsed = Number::new(foo).parse().unwrap();

        assert_eq!(
            parsed,
            Number {
                as_string: foo,
                integral: 1,
                ..Default::default()
            }
        );
    }

    #[test]
    fn fractional_only() {
        let foo = "0.1";

        let parsed = Number::new(foo).parse().unwrap();

        assert_eq!(
            parsed,
            Number {
                integral: 0,
                fractional: 1,
                significant_digits_count: 1,
                as_string: foo,
                total_fractional_count: 1,
                ..Default::default()
            }
        );
    }

    #[test]
    fn empty_str() {
        let foo = "";

        let parsed = Number::new(foo).parse();

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidNumber));
    }

    #[test]
    fn integral_decimal_point() {
        let foo = "1.";

        let parsed = Number::new(foo).parse();

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidNumber));
    }

    #[test]
    fn fractional_decimal_point() {
        let foo = ".1";

        let parsed = Number::new(foo).parse();

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidNumber));
    }

    #[test]
    fn decimal_point() {
        let foo = ".";

        let parsed = Number::new(foo).parse();

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidNumber));
    }

    #[test]
    fn three_decimal_points() {
        let foo = "1.1.";

        let parsed = Number::new(foo).parse();

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidNumber));
    }

    #[test]
    fn three_decimal_points_and_number() {
        let foo = "1.1.1";

        let parsed = Number::new(foo).parse();

        assert_eq!(parsed, Err(crate::SolanaPayError::InvalidNumber));
    }

    #[test]
    fn count_is_valid() {
        let count = Number::new(146_785u64.to_string().as_str())
            .parse()
            .unwrap()
            .significant_digits_count;
        assert_eq!(count, 0);

        let count = Number::new("0.146785")
            .parse()
            .unwrap()
            .significant_digits_count;
        assert_eq!(count, 6);
    }

    #[test]
    fn with_leading_zeroes() {
        let outcome = Number::new("0.001").parse().unwrap();

        assert_eq!(outcome.integral, 0);
        assert_eq!(outcome.fractional, 1);
        assert_eq!(outcome.leading_zeroes, 2);
        assert_eq!(outcome.significant_digits_count, 1);
        assert_eq!(outcome.as_string, "0.001");
    }
}
