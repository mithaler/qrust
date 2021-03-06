use std::cmp::min;

use bitvec::prelude::*;
use encoding::all::{ISO_8859_1, UTF_8};
use encoding::{EncoderTrap, Encoding};

use QREncoding::*;

use crate::qr::error_correction::ErrorCorrectionLevel;
use crate::qr::version::Version;
use crate::qr::{bytes_to_bitvec, insert_into_data, Error, QREncodedData};

fn div_rem(a: usize, b: usize) -> (usize, usize) {
    (a / b, a % b)
}

#[derive(PartialEq, Debug)]
pub enum QREncoding {
    Numeric,
    Alphanumeric,
    Bytes,
    Kanji, // TODO
}

fn alphanumeric_char_value(character: &char) -> Option<u16> {
    match character {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'A' => Some(10),
        'B' => Some(11),
        'C' => Some(12),
        'D' => Some(13),
        'E' => Some(14),
        'F' => Some(15),
        'G' => Some(16),
        'H' => Some(17),
        'I' => Some(18),
        'J' => Some(19),
        'K' => Some(20),
        'L' => Some(21),
        'M' => Some(22),
        'N' => Some(23),
        'O' => Some(24),
        'P' => Some(25),
        'Q' => Some(26),
        'R' => Some(27),
        'S' => Some(28),
        'T' => Some(29),
        'U' => Some(30),
        'V' => Some(31),
        'W' => Some(32),
        'X' => Some(33),
        'Y' => Some(34),
        'Z' => Some(35),
        ' ' => Some(36),
        '$' => Some(37),
        '%' => Some(38),
        '*' => Some(39),
        '+' => Some(40),
        '-' => Some(41),
        '.' => Some(42),
        '/' => Some(43),
        ':' => Some(44),
        _ => None,
    }
}

/// Performs encoding in Numeric mode, as described in section 8.4.2 of the spec.
fn encode_numeric(data: &str) -> QREncodedData {
    let mut cur = data;
    let (tria, remainder) = div_rem(data.len(), 2);
    let mut out = BitVec::with_capacity(
        tria * 10
            + if remainder == 2 {
                7
            } else if remainder == 1 {
                4
            } else {
                0
            },
    );
    while !cur.is_empty() {
        let (three_digits, rest) = cur.split_at(min(3, cur.len()));

        let bitcount = if !rest.is_empty() || three_digits.len() == 3 {
            10
        } else if three_digits.len() == 2 {
            7
        } else {
            4
        };

        let value = three_digits.parse::<u16>().unwrap() << (16 - bitcount);
        insert_into_data(&mut out, value, bitcount);
        cur = rest;
    }
    out
}

/// Performs encoding in Alphanumeric mode, as described in section 8.4.3 of the spec.
fn encode_alphanumeric(data: &str) -> QREncodedData {
    let mut cur = data;
    let (pairs, remainder) = div_rem(data.len(), 2);
    let mut out = BitVec::with_capacity(pairs * 11 + if remainder > 0 { 6 } else { 0 });

    while !cur.is_empty() {
        let (two_letters, rest) = cur.split_at(min(2, cur.len()));
        let mut chars = two_letters.chars();

        let (mut value, bitcount) = if two_letters.len() == 2 {
            (
                alphanumeric_char_value(&chars.next().unwrap()).unwrap() * 45
                    + alphanumeric_char_value(&chars.next().unwrap()).unwrap(),
                11,
            )
        } else {
            (alphanumeric_char_value(&chars.next().unwrap()).unwrap(), 6)
        };

        value <<= 16 - bitcount;
        insert_into_data(&mut out, value, bitcount);
        cur = rest;
    }
    out
}

/// Performs encoding in Bytes mode, as described in section 8.4.4 of the spec.
fn encode_bytes(data: &str) -> QREncodedData {
    let bytes = ISO_8859_1
        .encode(data, EncoderTrap::Strict)
        .or_else(|_| UTF_8.encode(data, EncoderTrap::Replace))
        .unwrap();
    bytes_to_bitvec(bytes)
}

impl QREncoding {
    fn allows_char(&self, character: &char) -> bool {
        match self {
            Numeric => character.is_digit(10),
            Alphanumeric => alphanumeric_char_value(&character).is_some(),
            Bytes => true,
            _ => unimplemented!(),
        }
    }

    fn encode(&self, data: &str) -> QREncodedData {
        match self {
            Numeric => encode_numeric(data),
            Alphanumeric => encode_alphanumeric(data),
            Bytes => encode_bytes(data),
            _ => unimplemented!(),
        }
    }

    fn mode(&self) -> QREncodedData {
        // Spec: 8.4, Table 2
        match self {
            Numeric => bitvec![Lsb0, u8; 0, 0, 0, 1],
            Alphanumeric => bitvec![Lsb0, u8; 0, 0, 1, 0],
            Bytes => bitvec![Lsb0, u8; 0, 1, 0, 0],
            _ => unimplemented!(),
        }
    }

    fn character_count_bits(&self, version_num: u8) -> usize {
        // Spec: 8.4, Table 3
        let (tier_1, tier_2, tier_3) = match self {
            Numeric => (10, 12, 14),
            Alphanumeric => (9, 11, 13),
            Bytes => (8, 16, 16),
            Kanji => (8, 10, 12),
        };
        match version_num {
            1..=9 => tier_1,
            10..=26 => tier_2,
            27..=40 => tier_3,
            _ => unreachable!("Version numbers don't go above 40, silly!"),
        }
    }
}

/// Selects the encoding based on the input data. Currently Kanji mode is unsupported.
/// ECI mode support is possible in the future, I suppose, but unlikely.
fn choose_encoding(data: &str) -> QREncoding {
    let mut can_be_numeric = true;
    let mut can_be_alphanumeric = true;
    for char in data.chars() {
        if can_be_numeric && !Numeric.allows_char(&char) {
            can_be_numeric = false;
        }
        if can_be_alphanumeric && !Alphanumeric.allows_char(&char) {
            can_be_alphanumeric = false;
        }
    }

    if can_be_numeric {
        Numeric
    } else if can_be_alphanumeric {
        Alphanumeric
    } else {
        Bytes
    }
}

#[derive(Debug)]
pub struct QRBitstreamEncoder {
    pub data: QREncodedData,
    pub encoding: QREncoding,
    pub character_count: u16,
}

impl QRBitstreamEncoder {
    pub fn new(data: &str) -> QRBitstreamEncoder {
        let encoding = choose_encoding(&data);
        let encoded_data = encoding.encode(&data);
        QRBitstreamEncoder {
            data: encoded_data,
            encoding,
            character_count: data.len() as u16,
        }
    }

    fn bitstream_length_before_terminator(&self, version_num: u8) -> usize {
        // mode + character count indicator + data
        4 + self.encoding.character_count_bits(version_num) + self.data.len()
    }

    pub fn codeword_count_before_padding(&self, version_num: u8) -> usize {
        let character_count_bits = self.bitstream_length_before_terminator(version_num);
        ((character_count_bits + (8 - 1)) / 8) as usize // divide rounding up
    }

    pub fn bitstream(
        &mut self,
        version: &Version,
        ecl: &ErrorCorrectionLevel,
    ) -> Result<QREncodedData, Error> {
        let codeword_count = version.codeword_count(ecl);
        let mut bitstream = BitVec::with_capacity(codeword_count * 8);
        let mut mode = self.encoding.mode();

        let char_count_value = self.character_count;
        let char_count_size = self.encoding.character_count_bits(version.num);
        let mut char_count_indicator = BitVec::with_capacity(char_count_size);
        insert_into_data(
            &mut char_count_indicator,
            char_count_value << (16 - char_count_size),
            char_count_size,
        );

        bitstream.append(&mut mode);
        bitstream.append(&mut char_count_indicator);
        bitstream.append(&mut self.data);

        // Add the terminator of up to 4 zeroes
        let remaining_size = codeword_count * 8 - bitstream.len();
        for _ in 0..(min(4, remaining_size)) {
            bitstream.push(false);
        }

        // Finish out the codeword with zeroes
        let codeword_remainder = bitstream.len() % 8;
        if codeword_remainder > 0 {
            for _ in 0..(8 - codeword_remainder) {
                bitstream.push(false);
            }
        }

        // Make sure we haven't somehow gone over (if that happened, there's a bug somewhere!)
        if bitstream.len() / 8 > codeword_count {
            return Err(format!(
                "The data length of {} doesn't fit into the chosen version of {}!",
                bitstream.len(),
                version.num
            )
            .into());
        }

        // Pad remaining codewords with a cycle of 0xEC and 0x11
        let mut padding_cycle = [0xEC00u16, 0x1100u16].iter().cycle();
        while bitstream.len() / 8 != codeword_count {
            insert_into_data(&mut bitstream, padding_cycle.next().unwrap().to_owned(), 8);
        }

        Ok(bitstream)
    }

    pub fn codewords(
        &mut self,
        version: &Version,
        ecl: &ErrorCorrectionLevel,
    ) -> Result<Vec<u8>, Error> {
        let bitstream = self.bitstream(&version, &ecl)?;
        if bitstream.len() % 8 != 0 {
            Err("The bitstream didn't come out in even bytes!".into())
        } else if bitstream.len() / 8 != version.codeword_count(&ecl) {
            Err("The bitstream has the wrong number of codewords for the version!".into())
        } else {
            // We have to reverse each individual byte to get them to come out right
            Ok(bitstream.domain().map(|byte| byte.reverse_bits()).collect())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_encoding() {
        assert_eq!(choose_encoding("0051023159"), Numeric);
        assert_eq!(choose_encoding("0051023159 ASDABGVASXD$-"), Alphanumeric);
        assert_eq!(choose_encoding("00510231 59asfasdASDASFGAQS"), Bytes);
        assert_eq!(choose_encoding("I am the Code"), Bytes);
        assert_eq!(choose_encoding("Привет, мир!"), Bytes);
    }

    mod numeric {
        use super::*;

        #[test]
        fn test_encode_numeric() {
            let data = "12300001010";
            let encoding = choose_encoding(&data);
            let encoded = encoding.encode(&data);
            assert_eq!(encoded.len(), 37);
            assert_eq!(
                encoded,
                bitvec![
                    0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0
                ]
            );
        }
    }

    mod alphanumeric {
        use super::*;

        #[test]
        fn test_encode_hello_world() {
            let data = "HELLO WORLD";
            let encoding = choose_encoding(&data);
            let encoded = encoding.encode(&data);
            assert_eq!(encoded.len(), 61);
            assert_eq!(
                encoded,
                bitvec![
                    0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0,
                    1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0,
                    1, 0, 0, 0, 0, 1, 1, 0, 1
                ]
            )
        }
    }

    mod bytes {
        use super::*;

        #[test]
        fn test_encode_byte_iso8859() {
            let data = "Hello, world!";
            let encoding = choose_encoding(&data);
            assert_eq!(
                encoding.encode(&data),
                bytes_to_bitvec(vec![
                    0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21
                ])
            )
        }

        #[test]
        fn test_encode_byte_utf8() {
            let data = "Привет, мир!";
            let encoding = choose_encoding(&data);
            assert_eq!(
                encoding.encode(&data),
                bytes_to_bitvec(vec![
                    208, 159, 209, 128, 208, 184, 208, 178, 208, 181, 209, 130, 44, 32, 208, 188,
                    208, 184, 209, 128, 33
                ])
            );
        }
    }

    mod encoder {
        use crate::qr::version::Version;

        use super::*;

        #[test]
        fn test_numeric() {
            let mut encoder = QRBitstreamEncoder::new("12300001010");
            assert_eq!(encoder.bitstream_length_before_terminator(1), 51);
            assert_eq!(encoder.bitstream_length_before_terminator(9), 51);
            assert_eq!(encoder.bitstream_length_before_terminator(10), 53);
            assert_eq!(encoder.bitstream_length_before_terminator(25), 53);
            assert_eq!(encoder.bitstream_length_before_terminator(39), 55);
            assert_eq!(encoder.bitstream_length_before_terminator(40), 55);

            assert_eq!(encoder.codeword_count_before_padding(1), 7);
            assert_eq!(encoder.codeword_count_before_padding(9), 7);
            assert_eq!(encoder.codeword_count_before_padding(10), 7);
            assert_eq!(encoder.codeword_count_before_padding(25), 7);
            assert_eq!(encoder.codeword_count_before_padding(39), 7);
            assert_eq!(encoder.codeword_count_before_padding(40), 7);

            assert_eq!(
                encoder
                    .bitstream(Version::by_num(1), &ErrorCorrectionLevel::Medium)
                    .unwrap(),
                bitvec![Lsb0, u8;
                    0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0,
                    0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1,
                    0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1,
                    1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0
                ]
            )
        }

        #[test]
        fn test_alphanumeric() {
            let mut encoder = QRBitstreamEncoder::new(
                "12300001010\
                AGASSLKDJOAKSJDGPIOIASDFGKJAHSSDGFKJHSDGLKJSHDLJKFHSDFJ  \
                SDKLJFHSLKDJFHSLKDJHFLSDJKHF",
            );

            assert_eq!(encoder.bitstream_length_before_terminator(1), 541);
            assert_eq!(encoder.bitstream_length_before_terminator(9), 541);
            assert_eq!(encoder.bitstream_length_before_terminator(10), 543);
            assert_eq!(encoder.bitstream_length_before_terminator(25), 543);
            assert_eq!(encoder.bitstream_length_before_terminator(39), 545);
            assert_eq!(encoder.bitstream_length_before_terminator(40), 545);

            assert_eq!(encoder.codeword_count_before_padding(1), 68);
            assert_eq!(encoder.codeword_count_before_padding(9), 68);
            assert_eq!(encoder.codeword_count_before_padding(10), 68);
            assert_eq!(encoder.codeword_count_before_padding(25), 68);
            assert_eq!(encoder.codeword_count_before_padding(39), 69);
            assert_eq!(encoder.codeword_count_before_padding(40), 69);

            assert_eq!(
                encoder
                    .bitstream(Version::by_num(5), &ErrorCorrectionLevel::Medium)
                    .unwrap(),
                bitvec![Lsb0, u8;
                    0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0,
                    0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
                    0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1,
                    1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0,
                    0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0,
                    0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0,
                    0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0,
                    0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1,
                    0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0,
                    1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0,
                    0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0,
                    0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1,
                    0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0,
                    1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0,
                    1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0,
                    0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1,
                    0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0,
                    0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1,
                    0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0,
                    1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0,
                    1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0,
                    0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0,
                    0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1,
                    1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1,
                    0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0
                ]
            )
        }

        #[test]
        fn test_bytes() {
            let encoder = QRBitstreamEncoder::new(
                "Golden ratio φ = 1.6180339887498948482045868343656381177203091798057628621354486227052604628189024497072072041893911374......"
            );
            assert_eq!(encoder.bitstream_length_before_terminator(1), 1020);
            assert_eq!(encoder.bitstream_length_before_terminator(9), 1020);
            assert_eq!(encoder.bitstream_length_before_terminator(10), 1028);
            assert_eq!(encoder.bitstream_length_before_terminator(25), 1028);
            assert_eq!(encoder.bitstream_length_before_terminator(39), 1028);
            assert_eq!(encoder.bitstream_length_before_terminator(40), 1028);

            assert_eq!(encoder.codeword_count_before_padding(1), 128);
            assert_eq!(encoder.codeword_count_before_padding(9), 128);
            assert_eq!(encoder.codeword_count_before_padding(10), 129);
            assert_eq!(encoder.codeword_count_before_padding(25), 129);
            assert_eq!(encoder.codeword_count_before_padding(39), 129);
            assert_eq!(encoder.codeword_count_before_padding(40), 129);
        }

        #[test]
        fn test_bytes_bitstream() {
            let mut encoder = QRBitstreamEncoder::new("aЉ윇😱");
            assert_eq!(
                encoder
                    .bitstream(Version::by_num(2), &ErrorCorrectionLevel::High)
                    .unwrap(),
                bitvec![Lsb0, u8;
                    0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0,
                    0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0,
                    1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0,
                    0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0,
                    0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1
                ]
            )
        }

        #[test]
        fn test_bytes_into_codewords() {
            let mut encoder =
                QRBitstreamEncoder::new("Hello, world! I am a weirdly complicated QR code!");
            assert_eq!(
                encoder
                    .codewords(Version::by_num(5), &ErrorCorrectionLevel::Quartile)
                    .unwrap(),
                vec![
                    0x43, 0x14, 0x86, 0x56, 0xC6, 0xC6, 0xF2, 0xC2, 0x07, 0x76, 0xF7, 0x26, 0xC6,
                    0x42, 0x12, 0x04, 0x92, 0x06, 0x16, 0xD2, 0x06, 0x12, 0x07, 0x76, 0x56, 0x97,
                    0x26, 0x46, 0xC7, 0x92, 0x06, 0x36, 0xF6, 0xD7, 0x06, 0xC6, 0x96, 0x36, 0x17,
                    0x46, 0x56, 0x42, 0x05, 0x15, 0x22, 0x06, 0x36, 0xF6, 0x46, 0x52, 0x10, 0xEC,
                    0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC, 0x11, 0xEC
                ],
            )
        }
    }
}
