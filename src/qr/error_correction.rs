use std::borrow::Cow;
use std::str::FromStr;

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

type Block = Vec<Vec<u8>>;

#[derive(Debug, PartialEq)]
struct GroupedCodewords {
    block1: Block,
    block2: Option<Block>,
}

fn _group_into_block(mut codewords: &[u8], block_count: u8, codewords_per_block: u8) -> Block {
    let mut block = Block::with_capacity(block_count.into());
    for _ in 0..block_count {
        let (next, rest) = codewords.split_at(codewords_per_block.into());
        codewords = rest;
        block.push(next.to_vec());
    }
    block
}

fn _group_into_blocks(codewords: Vec<u8>, ecl_data: &VersionEclData) -> GroupedCodewords {
    let block1_grp = &ecl_data.group1;
    let block2_grp = ecl_data.group2.as_ref();
    GroupedCodewords {
        block1: _group_into_block(&codewords, block1_grp.blocks, block1_grp.codewords),
        block2: block2_grp.map(|grp| {
            _group_into_block(
                &codewords[block1_grp.blocks as usize * block1_grp.codewords as usize..],
                grp.blocks,
                grp.codewords,
            )
        }),
    }
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
        let polynomial = _from_codewords(codewords);

        let fixture: _Polynomial =
            read_fixture("fixtures/error_correction/test_from_codewords.yml");

        assert_eq!(polynomial, fixture);
    }

    mod group_into_blocks {
        use super::*;

        #[test]
        fn test_no_block2() {
            let ver = Version::by_num(1);
            let ecl = &ErrorCorrectionLevel::Quartile;
            let codewords = QRBitstreamEncoder::new("HELLO WORLD")
                .codewords(ver, ecl)
                .unwrap();

            assert_eq!(
                _group_into_blocks(codewords, ver.values_at_ecl(ecl)),
                GroupedCodewords {
                    block1: vec![vec![
                        0x20, 0x5B, 0x0B, 0x78, 0xD1, 0x72, 0xDC, 0x4D, 0x43, 0x40, 0xEC, 0x11,
                        0xEC
                    ]],
                    block2: None
                }
            )
        }

        #[test]
        fn test_with_block2() {
            let ver = Version::by_num(5);
            let ecl = &ErrorCorrectionLevel::Quartile;
            let codewords =
                QRBitstreamEncoder::new("Hello, world! I am a weirdly complicated QR code!")
                    .codewords(ver, ecl)
                    .unwrap();

            assert_eq!(
                _group_into_blocks(codewords, ver.values_at_ecl(ecl)),
                GroupedCodewords {
                    block1: vec![
                        vec![
                            0x43, 0x14, 0x86, 0x56, 0xC6, 0xC6, 0xF2, 0xC2, 0x07, 0x76, 0xF7, 0x26,
                            0xC6, 0x42, 0x12
                        ],
                        vec![
                            0x04, 0x92, 0x06, 0x16, 0xD2, 0x06, 0x12, 0x07, 0x76, 0x56, 0x97, 0x26,
                            0x46, 0xC7, 0x92
                        ]
                    ],
                    block2: Some(vec![
                        vec![
                            0x06, 0x36, 0xF6, 0xD7, 0x06, 0xC6, 0x96, 0x36, 0x17, 0x46, 0x56, 0x42,
                            0x05, 0x15, 0x22, 0x06
                        ],
                        vec![
                            0x36, 0xF6, 0x46, 0x52, 0x10, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC,
                            0x11, 0xEC, 0x11, 0xEC
                        ]
                    ])
                }
            )
        }
    }
}
