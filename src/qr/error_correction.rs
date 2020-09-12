use std::str::FromStr;

#[derive(Debug)]
pub enum ErrorCorrectionLevel {
    Low,
    Medium,
    Quartile,
    High,
}

impl FromStr for ErrorCorrectionLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(ErrorCorrectionLevel::Low),
            "medium" => Ok(ErrorCorrectionLevel::Medium),
            "quartile" => Ok(ErrorCorrectionLevel::Quartile),
            "high" => Ok(ErrorCorrectionLevel::High),
            _ => Err(format!(
                "Unknown error correction level {} (options are low, medium, quartile, high)",
                s
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(test, derive(serde::Deserialize))]
struct Term {
    coefficient: u8,
    exponent: u16,
}

type _Polynomial = Vec<Term>;

fn _from_codewords(codewords: Vec<u8>) -> _Polynomial {
    let mut polynomial = Vec::with_capacity(codewords.len());
    for (i, codeword) in codewords.iter().enumerate() {
        polynomial.push(Term {
            coefficient: codeword.to_owned(),
            exponent: (codewords.len() - i - 1) as u16,
        });
    }
    polynomial
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qr::encode::QRBitstreamEncoder;
    use crate::qr::version::Version;
    use crate::read_fixture;

    #[test]
    fn test_from_codewords() {
        let codewords = QRBitstreamEncoder::new("HELLO WORLD")
            .codewords(Version::by_num(5), &ErrorCorrectionLevel::Quartile)
            .unwrap();
        let polynomial = _from_codewords(codewords);

        let fixture: _Polynomial =
            read_fixture("fixtures/error_correction/test_from_codewords.yml");

        assert_eq!(polynomial, fixture);
    }
}
