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

fn gf256_multiply(x: u8, y: u8) -> u8 {
    let mut z = 0u16;
    for i in (0..8).rev() {
        z = (z << 1) ^ ((z >> 7) * 0b100011101);
        z ^= ((y as u16 >> i) & 1) * x as u16;
    }
    z as u8
}

fn generator_polynomial(count: usize) -> Vec<u8> {
    let mut generator = vec![0; (count - 1) as usize];
    generator.push(1);
    let mut multiplicand = 1;
    for _ in 0..count {
        for j in 0..count {
            generator[j] = gf256_multiply(generator[j], multiplicand);
            if j + 1 < count {
                generator[j] ^= generator[j + 1];
            }
        }
        multiplicand = gf256_multiply(multiplicand, 2);
    }
    generator
}

fn compute_ec_codewords(block: &[u8], generator: &[u8]) -> Vec<u8> {
    let mut ec_codewords = vec![0; generator.len()];
    for codeword in block {
        let curr = codeword ^ ec_codewords.remove(0);
        ec_codewords.push(0);
        for (x, &y) in ec_codewords.iter_mut().zip(generator.iter()) {
            *x ^= gf256_multiply(y, curr);
        }
    }
    ec_codewords
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

    fn compute_ec_for_group(data: &[Vec<u8>], generator_polynomial: &[u8]) -> Group {
        let mut out = Group::with_capacity(data.len());
        for block in data {
            out.push(compute_ec_codewords(block, generator_polynomial));
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

        let generator_polynomial = generator_polynomial(version_data.ec_codewords_per_block);
        let group1_ec = Self::compute_ec_for_group(&group1_data, generator_polynomial.as_slice());
        let group2_ec = group2_data
            .as_ref()
            .map(|data| Self::compute_ec_for_group(data, generator_polynomial.as_slice()));

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

    use super::*;

    mod generator_polynomial {
        use crate::qr::error_correction::generator_polynomial;

        #[test]
        fn test_generator_polynomial_3() {
            let three = generator_polynomial(3);
            assert_eq!(three, vec![7, 14, 8]);
        }

        #[test]
        fn test_generator_polynomial_12() {
            let twelve = generator_polynomial(12);
            assert_eq!(
                twelve,
                vec![68, 119, 67, 118, 220, 31, 7, 84, 92, 127, 213, 97]
            );
        }
    }

    #[test]
    fn test_compute_ec_codewords() {
        let mut encoder = QRBitstreamEncoder::new("HELLO WORLD");
        let codewords = encoder
            .codewords(Version::by_num(1), &ErrorCorrectionLevel::Medium)
            .unwrap();
        assert_eq!(
            compute_ec_codewords(codewords.as_slice(), generator_polynomial(10).as_slice()),
            vec![196, 35, 39, 119, 235, 215, 231, 226, 93, 23]
        )
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
                        32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236
                    ]],
                    group2_data: None,
                    group1_ec: vec![vec![168, 72, 22, 82, 217, 54, 156, 0, 46, 15, 180, 122, 16]],
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
                        vec![67, 20, 134, 86, 198, 198, 242, 194, 7, 118, 247, 38, 198, 66, 18],
                        vec![4, 146, 6, 22, 210, 6, 18, 7, 118, 86, 151, 38, 70, 199, 146]
                    ],
                    group2_data: Some(vec![
                        vec![6, 54, 246, 215, 6, 198, 150, 54, 23, 70, 86, 66, 5, 21, 34, 6],
                        vec![54, 246, 70, 82, 16, 236, 17, 236, 17, 236, 17, 236, 17, 236, 17, 236]
                    ]),
                    group1_ec: vec![
                        vec![
                            102, 248, 250, 159, 123, 170, 252, 51, 18, 31, 51, 24, 104, 188, 208,
                            136, 8, 198
                        ],
                        vec![
                            123, 74, 15, 136, 193, 56, 23, 192, 192, 81, 252, 199, 239, 165, 100,
                            14, 60, 235
                        ]
                    ],
                    group2_ec: Some(vec![
                        vec![
                            172, 221, 66, 59, 100, 224, 86, 252, 31, 180, 136, 45, 29, 191, 139,
                            107, 185, 211
                        ],
                        vec![
                            133, 244, 41, 66, 140, 102, 194, 255, 224, 0, 1, 47, 66, 239, 248, 244,
                            134, 241
                        ]
                    ])
                }
            )
        }
    }
}
