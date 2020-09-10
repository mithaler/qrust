use bitvec::prelude::*;
use encoding::all::{ISO_8859_1, UTF_8};
use encoding::{EncoderTrap, Encoding};
use phf::phf_map;
use std::cmp::min;

fn div_rem(a: usize, b: usize) -> (usize, usize) {
    (a / b, a % b)
}

#[derive(PartialEq, Debug)]
pub enum QREncoding {
    Numeric,
    Alphanumeric,
    Byte,
    // Kanji not supported yet!
}

pub type QREncodedData = BitVec<Lsb0, u8>;
type QREncodingMap = phf::Map<char, u16>;

static NUMERIC_CHARS: QREncodingMap = phf_map! {
    '0' => 0,
    '1' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
};

static ALPHANUMERIC_CHARS: QREncodingMap = phf_map! {
    '0' => 0,
    '1' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'A' => 10,
    'B' => 11,
    'C' => 12,
    'D' => 13,
    'E' => 14,
    'F' => 15,
    'G' => 16,
    'H' => 17,
    'I' => 18,
    'J' => 19,
    'K' => 20,
    'L' => 21,
    'M' => 22,
    'N' => 23,
    'O' => 24,
    'P' => 25,
    'Q' => 26,
    'R' => 27,
    'S' => 28,
    'T' => 29,
    'U' => 30,
    'V' => 31,
    'W' => 32,
    'X' => 33,
    'Y' => 34,
    'Z' => 35,
    ' ' => 36,
    '$' => 37,
    '%' => 38,
    '*' => 39,
    '+' => 40,
    '-' => 41,
    '.' => 42,
    '/' => 43,
    ':' => 44,
};

use QREncoding::*;

fn slice_to_bitvec(data: &[u8]) -> QREncodedData {
    BitVec::<Lsb0, u8>::from_bitslice(BitSlice::from_slice(data).unwrap())
}

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
        println!("number {}", three_digits);

        let bitcount = if !rest.is_empty() || three_digits.len() == 3 {
            10
        } else if three_digits.len() == 2 {
            7
        } else {
            4
        };

        let mut value = three_digits.parse::<u16>().unwrap() << (16 - bitcount);
        println!("binary {:0>16b}", value);
        for _ in 0..bitcount {
            out.push(0b1000_0000_0000_0000 & value > 0);
            value <<= 1;
        }
        cur = rest;
    }
    out
}

fn encode_alphanumeric(data: &str) -> QREncodedData {
    let mut cur = data;
    let (pairs, remainder) = div_rem(data.len(), 2);
    let mut out = BitVec::with_capacity(pairs * 11 + if remainder > 0 { 6 } else { 0 });

    while !cur.is_empty() {
        let (two_letters, rest) = cur.split_at(min(2, cur.len()));
        let mut chars = two_letters.chars();
        println!("pair {}", two_letters);

        let (mut value, bitcount) = if two_letters.len() == 2 {
            (
                ALPHANUMERIC_CHARS.get(&chars.next().unwrap()).unwrap() * 45
                    + ALPHANUMERIC_CHARS.get(&chars.next().unwrap()).unwrap(),
                11,
            )
        } else {
            (
                ALPHANUMERIC_CHARS
                    .get(&chars.next().unwrap())
                    .unwrap()
                    .to_owned(),
                6,
            )
        };

        println!("binary {:0>16b}", value);
        value <<= 16 - bitcount;
        for _ in 0..bitcount {
            out.push(0b1000_0000_0000_0000 & value > 0);
            value <<= 1;
        }
        cur = rest;
    }
    out
}

fn encode_bytes(data: &str) -> QREncodedData {
    let bytes = ISO_8859_1
        .encode(data, EncoderTrap::Strict)
        .or_else(|_| UTF_8.encode(data, EncoderTrap::Replace))
        .unwrap();
    slice_to_bitvec(bytes.as_slice())
}

impl QREncoding {
    fn allows_char(&self, character: &char) -> bool {
        match self {
            Numeric => NUMERIC_CHARS.contains_key(character),
            Alphanumeric => ALPHANUMERIC_CHARS.contains_key(character),
            Byte => true,
        }
    }

    fn encode(&self, data: &str) -> QREncodedData {
        match self {
            Numeric => encode_numeric(data),
            Alphanumeric => encode_alphanumeric(data),
            Byte => encode_bytes(data),
        }
    }

    fn mode(&self) -> QREncodedData {
        match self {
            Numeric => bitvec![Lsb0, u8; 0, 0, 0, 1],
            Alphanumeric => bitvec![Lsb0, u8; 0, 0, 1, 0],
            Byte => bitvec![Lsb0, u8; 0, 1, 0, 0],
        }
    }
}

pub fn choose_encoding(data: &str) -> QREncoding {
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
        Byte
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_encoding() {
        assert_eq!(choose_encoding("0051023159"), Numeric);
        assert_eq!(choose_encoding("0051023159 ASDABGVASXD$-"), Alphanumeric);
        assert_eq!(choose_encoding("00510231 59asfasdASDASFGAQS"), Byte);
        assert_eq!(choose_encoding("I am the Code"), Byte);
        assert_eq!(choose_encoding("Привет, мир!"), Byte);
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
                slice_to_bitvec(
                    vec![
                        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64,
                        0x21
                    ]
                    .as_slice()
                )
            )
        }

        #[test]
        fn test_encode_byte_utf8() {
            let data = "Привет, мир!";
            let encoding = choose_encoding(&data);
            assert_eq!(
                encoding.encode(&data),
                slice_to_bitvec(
                    vec![
                        208, 159, 209, 128, 208, 184, 208, 178, 208, 181, 209, 130, 44, 32, 208,
                        188, 208, 184, 209, 128, 33
                    ]
                    .as_slice()
                )
            );
        }
    }
}
