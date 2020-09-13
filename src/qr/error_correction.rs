use std::borrow::Cow;
use std::str::FromStr;

use bitvec::prelude::*;

use crate::qr::encode::QREncodedData;
use crate::qr::version::VersionEclData;
use crate::qr::Error;

#[derive(Debug)]
pub enum ErrorCorrectionLevel {
    Low,
    Medium,
    Quartile,
    High,
}

impl FromStr for ErrorCorrectionLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(ErrorCorrectionLevel::Low),
            "medium" => Ok(ErrorCorrectionLevel::Medium),
            "quartile" => Ok(ErrorCorrectionLevel::Quartile),
            "high" => Ok(ErrorCorrectionLevel::High),
            _ => Err(Cow::from(format!(
                "Unknown error correction level {} (options are low, medium, quartile, high)",
                s
            ))),
        }
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(test, derive(serde::Deserialize))]
struct PolynomialTerm {
    coefficient: u8,
    exponent: u16,
}

type _Polynomial = Vec<PolynomialTerm>;

fn generator_polynomial(count: usize) -> _Polynomial {
    Vec::with_capacity(count)
}

fn codewords_to_polynomial(codewords: &[u8]) -> _Polynomial {
    let mut polynomial = Vec::with_capacity(codewords.len());
    for (i, codeword) in codewords.iter().enumerate() {
        polynomial.push(PolynomialTerm {
            coefficient: codeword.to_owned(),
            exponent: (codewords.len() - i - 1) as u16,
        });
    }
    polynomial
}

fn compute_ec_codewords(block: &[u8], count: usize) -> Vec<u8> {
    let _generator = generator_polynomial(count);
    let _codewords = codewords_to_polynomial(block);
    Vec::with_capacity(count)
}

type Block = Vec<u8>;
type Group = Vec<Block>;

#[derive(Debug, PartialEq)]
struct GroupedCodewords {
    version_data: &'static VersionEclData,
    group1_data: Group,
    group2_data: Option<Group>,
    group1_ec: Group,
    group2_ec: Option<Group>,
}

impl GroupedCodewords {
    fn block_group(mut codewords: &[u8], block_count: u8, codewords_per_block: u8) -> Group {
        let mut block = Group::with_capacity(block_count.into());
        for _ in 0..block_count {
            let (next, rest) = codewords.split_at(codewords_per_block.into());
            codewords = rest;
            block.push(next.to_vec());
        }
        block
    }

    fn compute_ec_for_group(data: &[Vec<u8>], ec_codeword_count: usize) -> Group {
        let mut out = Group::with_capacity(data.len());
        for block in data {
            out.push(compute_ec_codewords(block, ec_codeword_count));
        }
        out
    }

    fn new(codewords: Vec<u8>, version_data: &'static VersionEclData) -> GroupedCodewords {
        let group1 = &version_data.group1;
        let group2 = version_data.group2.as_ref();

        let group1_data = Self::block_group(&codewords, group1.blocks, group1.codewords);
        let group2_data = group2.map(|grp| {
            Self::block_group(
                &codewords[group1.blocks as usize * group1.codewords as usize..],
                grp.blocks,
                grp.codewords,
            )
        });
        let group1_ec =
            Self::compute_ec_for_group(&group1_data, version_data.ec_codewords_per_block.into());
        let group2_ec = group2_data.as_ref().map(|data| {
            Self::compute_ec_for_group(data, version_data.ec_codewords_per_block.into())
        });

        GroupedCodewords {
            version_data,
            group1_data,
            group2_data,
            group1_ec,
            group2_ec,
        }
    }

    fn bitstream(&self) -> QREncodedData {
        // TODO interleave codewords between groups, convert to bitstream
        bitvec![Lsb0, u8; 0]
    }
}

pub fn bitstream_with_ec(
    data_codewords: Vec<u8>,
    ecl_data: &'static VersionEclData,
) -> QREncodedData {
    GroupedCodewords::new(data_codewords, ecl_data).bitstream()
}

#[cfg(test)]
mod tests {
    use crate::qr::encode::QRBitstreamEncoder;
    use crate::qr::version::Version;
    use crate::read_fixture;

    use super::*;

    #[test]
    fn test_from_codewords() {
        let codewords = QRBitstreamEncoder::new("HELLO WORLD")
            .codewords(Version::by_num(5), &ErrorCorrectionLevel::Quartile)
            .unwrap();
        let polynomial = codewords_to_polynomial(codewords.as_slice());

        let fixture: _Polynomial =
            read_fixture("fixtures/error_correction/test_from_codewords.yml");

        assert_eq!(polynomial, fixture);
    }

    mod block_grouping {
        use super::*;

        #[test]
        fn test_no_block2() {
            let ver = Version::by_num(1);
            let ecl = &ErrorCorrectionLevel::Quartile;
            let ecl_data = ver.values_at_ecl(ecl);
            let codewords = QRBitstreamEncoder::new("HELLO WORLD")
                .codewords(ver, ecl)
                .unwrap();

            assert_eq!(
                GroupedCodewords::new(codewords, ecl_data),
                GroupedCodewords {
                    version_data: ecl_data,
                    group1_data: vec![vec![
                        0x20, 0x5B, 0x0B, 0x78, 0xD1, 0x72, 0xDC, 0x4D, 0x43, 0x40, 0xEC, 0x11,
                        0xEC
                    ]],
                    group2_data: None,
                    group1_ec: vec![vec![]],
                    group2_ec: None
                }
            )
        }

        #[test]
        fn test_with_block2() {
            let ver = Version::by_num(5);
            let ecl = &ErrorCorrectionLevel::Quartile;
            let ecl_data = ver.values_at_ecl(ecl);
            let codewords =
                QRBitstreamEncoder::new("Hello, world! I am a weirdly complicated QR code!")
                    .codewords(ver, ecl)
                    .unwrap();

            assert_eq!(
                GroupedCodewords::new(codewords, ecl_data),
                GroupedCodewords {
                    version_data: ecl_data,
                    group1_data: vec![
                        vec![
                            0x43, 0x14, 0x86, 0x56, 0xC6, 0xC6, 0xF2, 0xC2, 0x07, 0x76, 0xF7, 0x26,
                            0xC6, 0x42, 0x12
                        ],
                        vec![
                            0x04, 0x92, 0x06, 0x16, 0xD2, 0x06, 0x12, 0x07, 0x76, 0x56, 0x97, 0x26,
                            0x46, 0xC7, 0x92
                        ]
                    ],
                    group2_data: Some(vec![
                        vec![
                            0x06, 0x36, 0xF6, 0xD7, 0x06, 0xC6, 0x96, 0x36, 0x17, 0x46, 0x56, 0x42,
                            0x05, 0x15, 0x22, 0x06
                        ],
                        vec![
                            0x36, 0xF6, 0x46, 0x52, 0x10, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC,
                            0x11, 0xEC, 0x11, 0xEC
                        ]
                    ]),
                    group1_ec: vec![vec![], vec![]],
                    group2_ec: Some(vec![vec![], vec![]])
                }
            )
        }
    }
}
