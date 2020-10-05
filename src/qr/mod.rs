use std::borrow::Cow;

use bitvec::prelude::*;

pub mod encode;
pub mod error_correction;
pub mod image;
pub mod pattern;
pub mod version;

pub type Error = Cow<'static, str>;

pub(crate) type QREncodedData = BitVec<Lsb0, u8>;

pub(crate) fn insert_into_data(data: &mut QREncodedData, mut value: u16, count_bits: usize) {
    for _ in 0..count_bits {
        data.push(value & 0b1000_0000_0000_0000 > 0);
        value <<= 1;
    }
}

pub(crate) fn bytes_to_bitvec(data: Vec<u8>) -> QREncodedData {
    let mut out = BitVec::with_capacity(data.len() * 8);
    for value in data {
        insert_into_data(&mut out, (value as u16) << 8, 8);
    }
    out
}
