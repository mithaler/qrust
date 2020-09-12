use std::borrow::Cow;

use crate::qr::encode::QRBitstreamEncoder;
use crate::qr::error_correction::ErrorCorrectionLevel;
use crate::qr::Error;

#[derive(Debug)]
pub struct VersionEclData {
    data_codewords: usize,
    ec_codewords_per_block: u16,
    group1_blocks: u16,
    group1_codewords: u16,
    group2_blocks: Option<u16>,
    group2_codewords: Option<u16>,
}

/// A QR code version. All caps are codeword counts.
#[derive(Debug)]
pub struct Version {
    pub num: u8,
    l_data: VersionEclData,
    m_data: VersionEclData,
    q_data: VersionEclData,
    h_data: VersionEclData,
}

/// A QR code version, numbered 1 to 40.
impl Version {
    /// Looks up a version by its number.
    pub fn by_num(num: usize) -> &'static Version {
        VERSIONS[num - 1]
    }

    pub fn values_at_ecl(&self, ecl: &ErrorCorrectionLevel) -> &VersionEclData {
        match ecl {
            ErrorCorrectionLevel::Low => &self.l_data,
            ErrorCorrectionLevel::Medium => &self.m_data,
            ErrorCorrectionLevel::Quartile => &self.q_data,
            ErrorCorrectionLevel::High => &self.h_data,
        }
    }

    pub fn codeword_count(&self, ecl: &ErrorCorrectionLevel) -> usize {
        self.values_at_ecl(ecl).data_codewords
    }

    /// Returns the number of modules on a single side of the finished QR code.
    pub fn modules_per_side(&self) -> u32 {
        (4 * (self.num as u32 - 1)) + 21
    }
}

const VERSIONS: [&Version; 40] = [
    &Version {
        num: 1,
        l_data: VersionEclData {
            data_codewords: 19,
            ec_codewords_per_block: 7,
            group1_blocks: 1,
            group1_codewords: 19,
            group2_blocks: None,
            group2_codewords: None,
        },
        m_data: VersionEclData {
            data_codewords: 16,
            ec_codewords_per_block: 10,
            group1_blocks: 1,
            group1_codewords: 16,
            group2_blocks: None,
            group2_codewords: None,
        },
        q_data: VersionEclData {
            data_codewords: 13,
            ec_codewords_per_block: 13,
            group1_blocks: 1,
            group1_codewords: 13,
            group2_blocks: None,
            group2_codewords: None,
        },
        h_data: VersionEclData {
            data_codewords: 9,
            ec_codewords_per_block: 17,
            group1_blocks: 1,
            group1_codewords: 9,
            group2_blocks: None,
            group2_codewords: None,
        },
    },
    &Version {
        num: 2,
        l_data: VersionEclData {
            data_codewords: 34,
            ec_codewords_per_block: 10,
            group1_blocks: 1,
            group1_codewords: 34,
            group2_blocks: None,
            group2_codewords: None,
        },
        m_data: VersionEclData {
            data_codewords: 28,
            ec_codewords_per_block: 16,
            group1_blocks: 1,
            group1_codewords: 28,
            group2_blocks: None,
            group2_codewords: None,
        },
        q_data: VersionEclData {
            data_codewords: 22,
            ec_codewords_per_block: 22,
            group1_blocks: 1,
            group1_codewords: 22,
            group2_blocks: None,
            group2_codewords: None,
        },
        h_data: VersionEclData {
            data_codewords: 16,
            ec_codewords_per_block: 28,
            group1_blocks: 1,
            group1_codewords: 16,
            group2_blocks: None,
            group2_codewords: None,
        },
    },
    &Version {
        num: 3,
        l_data: VersionEclData {
            data_codewords: 55,
            ec_codewords_per_block: 15,
            group1_blocks: 1,
            group1_codewords: 55,
            group2_blocks: None,
            group2_codewords: None,
        },
        m_data: VersionEclData {
            data_codewords: 44,
            ec_codewords_per_block: 26,
            group1_blocks: 1,
            group1_codewords: 44,
            group2_blocks: None,
            group2_codewords: None,
        },
        q_data: VersionEclData {
            data_codewords: 34,
            ec_codewords_per_block: 18,
            group1_blocks: 2,
            group1_codewords: 17,
            group2_blocks: None,
            group2_codewords: None,
        },
        h_data: VersionEclData {
            data_codewords: 26,
            ec_codewords_per_block: 22,
            group1_blocks: 2,
            group1_codewords: 13,
            group2_blocks: None,
            group2_codewords: None,
        },
    },
    &Version {
        num: 4,
        l_data: VersionEclData {
            data_codewords: 80,
            ec_codewords_per_block: 20,
            group1_blocks: 1,
            group1_codewords: 80,
            group2_blocks: None,
            group2_codewords: None,
        },
        m_data: VersionEclData {
            data_codewords: 64,
            ec_codewords_per_block: 18,
            group1_blocks: 2,
            group1_codewords: 32,
            group2_blocks: None,
            group2_codewords: None,
        },
        q_data: VersionEclData {
            data_codewords: 48,
            ec_codewords_per_block: 26,
            group1_blocks: 2,
            group1_codewords: 24,
            group2_blocks: None,
            group2_codewords: None,
        },
        h_data: VersionEclData {
            data_codewords: 36,
            ec_codewords_per_block: 16,
            group1_blocks: 4,
            group1_codewords: 9,
            group2_blocks: None,
            group2_codewords: None,
        },
    },
    &Version {
        num: 5,
        l_data: VersionEclData {
            data_codewords: 108,
            ec_codewords_per_block: 26,
            group1_blocks: 1,
            group1_codewords: 108,
            group2_blocks: None,
            group2_codewords: None,
        },
        m_data: VersionEclData {
            data_codewords: 86,
            ec_codewords_per_block: 24,
            group1_blocks: 2,
            group1_codewords: 43,
            group2_blocks: None,
            group2_codewords: None,
        },
        q_data: VersionEclData {
            data_codewords: 62,
            ec_codewords_per_block: 18,
            group1_blocks: 2,
            group1_codewords: 15,
            group2_blocks: Some(2),
            group2_codewords: Some(16),
        },
        h_data: VersionEclData {
            data_codewords: 46,
            ec_codewords_per_block: 22,
            group1_blocks: 2,
            group1_codewords: 11,
            group2_blocks: Some(2),
            group2_codewords: Some(12),
        },
    },
    &Version {
        num: 6,
        l_data: VersionEclData {
            data_codewords: 136,
            ec_codewords_per_block: 18,
            group1_blocks: 2,
            group1_codewords: 68,
            group2_blocks: None,
            group2_codewords: None,
        },
        m_data: VersionEclData {
            data_codewords: 108,
            ec_codewords_per_block: 16,
            group1_blocks: 4,
            group1_codewords: 27,
            group2_blocks: None,
            group2_codewords: None,
        },
        q_data: VersionEclData {
            data_codewords: 76,
            ec_codewords_per_block: 24,
            group1_blocks: 4,
            group1_codewords: 19,
            group2_blocks: None,
            group2_codewords: None,
        },
        h_data: VersionEclData {
            data_codewords: 60,
            ec_codewords_per_block: 28,
            group1_blocks: 4,
            group1_codewords: 15,
            group2_blocks: None,
            group2_codewords: None,
        },
    },
    &Version {
        num: 7,
        l_data: VersionEclData {
            data_codewords: 156,
            ec_codewords_per_block: 20,
            group1_blocks: 2,
            group1_codewords: 78,
            group2_blocks: None,
            group2_codewords: None,
        },
        m_data: VersionEclData {
            data_codewords: 124,
            ec_codewords_per_block: 18,
            group1_blocks: 4,
            group1_codewords: 31,
            group2_blocks: None,
            group2_codewords: None,
        },
        q_data: VersionEclData {
            data_codewords: 88,
            ec_codewords_per_block: 18,
            group1_blocks: 2,
            group1_codewords: 14,
            group2_blocks: Some(4),
            group2_codewords: Some(15),
        },
        h_data: VersionEclData {
            data_codewords: 66,
            ec_codewords_per_block: 26,
            group1_blocks: 4,
            group1_codewords: 13,
            group2_blocks: Some(1),
            group2_codewords: Some(14),
        },
    },
    &Version {
        num: 8,
        l_data: VersionEclData {
            data_codewords: 194,
            ec_codewords_per_block: 24,
            group1_blocks: 2,
            group1_codewords: 97,
            group2_blocks: None,
            group2_codewords: None,
        },
        m_data: VersionEclData {
            data_codewords: 154,
            ec_codewords_per_block: 22,
            group1_blocks: 2,
            group1_codewords: 38,
            group2_blocks: Some(2),
            group2_codewords: Some(39),
        },
        q_data: VersionEclData {
            data_codewords: 110,
            ec_codewords_per_block: 22,
            group1_blocks: 4,
            group1_codewords: 18,
            group2_blocks: Some(2),
            group2_codewords: Some(19),
        },
        h_data: VersionEclData {
            data_codewords: 86,
            ec_codewords_per_block: 26,
            group1_blocks: 4,
            group1_codewords: 14,
            group2_blocks: Some(2),
            group2_codewords: Some(15),
        },
    },
    &Version {
        num: 9,
        l_data: VersionEclData {
            data_codewords: 232,
            ec_codewords_per_block: 30,
            group1_blocks: 2,
            group1_codewords: 116,
            group2_blocks: None,
            group2_codewords: None,
        },
        m_data: VersionEclData {
            data_codewords: 182,
            ec_codewords_per_block: 22,
            group1_blocks: 3,
            group1_codewords: 36,
            group2_blocks: Some(2),
            group2_codewords: Some(37),
        },
        q_data: VersionEclData {
            data_codewords: 132,
            ec_codewords_per_block: 20,
            group1_blocks: 4,
            group1_codewords: 16,
            group2_blocks: Some(4),
            group2_codewords: Some(17),
        },
        h_data: VersionEclData {
            data_codewords: 100,
            ec_codewords_per_block: 24,
            group1_blocks: 4,
            group1_codewords: 12,
            group2_blocks: Some(4),
            group2_codewords: Some(13),
        },
    },
    &Version {
        num: 10,
        l_data: VersionEclData {
            data_codewords: 274,
            ec_codewords_per_block: 18,
            group1_blocks: 2,
            group1_codewords: 68,
            group2_blocks: Some(2),
            group2_codewords: Some(69),
        },
        m_data: VersionEclData {
            data_codewords: 216,
            ec_codewords_per_block: 26,
            group1_blocks: 4,
            group1_codewords: 43,
            group2_blocks: Some(1),
            group2_codewords: Some(44),
        },
        q_data: VersionEclData {
            data_codewords: 154,
            ec_codewords_per_block: 24,
            group1_blocks: 6,
            group1_codewords: 19,
            group2_blocks: Some(2),
            group2_codewords: Some(20),
        },
        h_data: VersionEclData {
            data_codewords: 122,
            ec_codewords_per_block: 28,
            group1_blocks: 6,
            group1_codewords: 15,
            group2_blocks: Some(2),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 11,
        l_data: VersionEclData {
            data_codewords: 324,
            ec_codewords_per_block: 20,
            group1_blocks: 4,
            group1_codewords: 81,
            group2_blocks: None,
            group2_codewords: None,
        },
        m_data: VersionEclData {
            data_codewords: 254,
            ec_codewords_per_block: 30,
            group1_blocks: 1,
            group1_codewords: 50,
            group2_blocks: Some(4),
            group2_codewords: Some(51),
        },
        q_data: VersionEclData {
            data_codewords: 180,
            ec_codewords_per_block: 28,
            group1_blocks: 4,
            group1_codewords: 22,
            group2_blocks: Some(4),
            group2_codewords: Some(23),
        },
        h_data: VersionEclData {
            data_codewords: 140,
            ec_codewords_per_block: 24,
            group1_blocks: 3,
            group1_codewords: 12,
            group2_blocks: Some(8),
            group2_codewords: Some(13),
        },
    },
    &Version {
        num: 12,
        l_data: VersionEclData {
            data_codewords: 370,
            ec_codewords_per_block: 24,
            group1_blocks: 2,
            group1_codewords: 92,
            group2_blocks: Some(2),
            group2_codewords: Some(93),
        },
        m_data: VersionEclData {
            data_codewords: 290,
            ec_codewords_per_block: 22,
            group1_blocks: 6,
            group1_codewords: 36,
            group2_blocks: Some(2),
            group2_codewords: Some(37),
        },
        q_data: VersionEclData {
            data_codewords: 206,
            ec_codewords_per_block: 26,
            group1_blocks: 4,
            group1_codewords: 20,
            group2_blocks: Some(6),
            group2_codewords: Some(21),
        },
        h_data: VersionEclData {
            data_codewords: 158,
            ec_codewords_per_block: 28,
            group1_blocks: 7,
            group1_codewords: 14,
            group2_blocks: Some(4),
            group2_codewords: Some(15),
        },
    },
    &Version {
        num: 13,
        l_data: VersionEclData {
            data_codewords: 428,
            ec_codewords_per_block: 26,
            group1_blocks: 4,
            group1_codewords: 107,
            group2_blocks: None,
            group2_codewords: None,
        },
        m_data: VersionEclData {
            data_codewords: 334,
            ec_codewords_per_block: 22,
            group1_blocks: 8,
            group1_codewords: 37,
            group2_blocks: Some(1),
            group2_codewords: Some(38),
        },
        q_data: VersionEclData {
            data_codewords: 244,
            ec_codewords_per_block: 24,
            group1_blocks: 8,
            group1_codewords: 20,
            group2_blocks: Some(4),
            group2_codewords: Some(21),
        },
        h_data: VersionEclData {
            data_codewords: 180,
            ec_codewords_per_block: 22,
            group1_blocks: 12,
            group1_codewords: 11,
            group2_blocks: Some(4),
            group2_codewords: Some(12),
        },
    },
    &Version {
        num: 14,
        l_data: VersionEclData {
            data_codewords: 461,
            ec_codewords_per_block: 30,
            group1_blocks: 3,
            group1_codewords: 115,
            group2_blocks: Some(1),
            group2_codewords: Some(116),
        },
        m_data: VersionEclData {
            data_codewords: 365,
            ec_codewords_per_block: 24,
            group1_blocks: 4,
            group1_codewords: 40,
            group2_blocks: Some(5),
            group2_codewords: Some(41),
        },
        q_data: VersionEclData {
            data_codewords: 261,
            ec_codewords_per_block: 20,
            group1_blocks: 11,
            group1_codewords: 16,
            group2_blocks: Some(5),
            group2_codewords: Some(17),
        },
        h_data: VersionEclData {
            data_codewords: 197,
            ec_codewords_per_block: 24,
            group1_blocks: 11,
            group1_codewords: 12,
            group2_blocks: Some(5),
            group2_codewords: Some(13),
        },
    },
    &Version {
        num: 15,
        l_data: VersionEclData {
            data_codewords: 523,
            ec_codewords_per_block: 22,
            group1_blocks: 5,
            group1_codewords: 87,
            group2_blocks: Some(1),
            group2_codewords: Some(88),
        },
        m_data: VersionEclData {
            data_codewords: 415,
            ec_codewords_per_block: 24,
            group1_blocks: 5,
            group1_codewords: 41,
            group2_blocks: Some(5),
            group2_codewords: Some(42),
        },
        q_data: VersionEclData {
            data_codewords: 295,
            ec_codewords_per_block: 30,
            group1_blocks: 5,
            group1_codewords: 24,
            group2_blocks: Some(7),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 223,
            ec_codewords_per_block: 24,
            group1_blocks: 11,
            group1_codewords: 12,
            group2_blocks: Some(7),
            group2_codewords: Some(13),
        },
    },
    &Version {
        num: 16,
        l_data: VersionEclData {
            data_codewords: 589,
            ec_codewords_per_block: 24,
            group1_blocks: 5,
            group1_codewords: 98,
            group2_blocks: Some(1),
            group2_codewords: Some(99),
        },
        m_data: VersionEclData {
            data_codewords: 453,
            ec_codewords_per_block: 28,
            group1_blocks: 7,
            group1_codewords: 45,
            group2_blocks: Some(3),
            group2_codewords: Some(46),
        },
        q_data: VersionEclData {
            data_codewords: 325,
            ec_codewords_per_block: 24,
            group1_blocks: 15,
            group1_codewords: 19,
            group2_blocks: Some(2),
            group2_codewords: Some(20),
        },
        h_data: VersionEclData {
            data_codewords: 253,
            ec_codewords_per_block: 30,
            group1_blocks: 3,
            group1_codewords: 15,
            group2_blocks: Some(13),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 17,
        l_data: VersionEclData {
            data_codewords: 647,
            ec_codewords_per_block: 28,
            group1_blocks: 1,
            group1_codewords: 107,
            group2_blocks: Some(5),
            group2_codewords: Some(108),
        },
        m_data: VersionEclData {
            data_codewords: 507,
            ec_codewords_per_block: 28,
            group1_blocks: 10,
            group1_codewords: 46,
            group2_blocks: Some(1),
            group2_codewords: Some(47),
        },
        q_data: VersionEclData {
            data_codewords: 367,
            ec_codewords_per_block: 28,
            group1_blocks: 1,
            group1_codewords: 22,
            group2_blocks: Some(15),
            group2_codewords: Some(23),
        },
        h_data: VersionEclData {
            data_codewords: 283,
            ec_codewords_per_block: 28,
            group1_blocks: 2,
            group1_codewords: 14,
            group2_blocks: Some(17),
            group2_codewords: Some(15),
        },
    },
    &Version {
        num: 18,
        l_data: VersionEclData {
            data_codewords: 721,
            ec_codewords_per_block: 30,
            group1_blocks: 5,
            group1_codewords: 120,
            group2_blocks: Some(1),
            group2_codewords: Some(121),
        },
        m_data: VersionEclData {
            data_codewords: 563,
            ec_codewords_per_block: 26,
            group1_blocks: 9,
            group1_codewords: 43,
            group2_blocks: Some(4),
            group2_codewords: Some(44),
        },
        q_data: VersionEclData {
            data_codewords: 397,
            ec_codewords_per_block: 28,
            group1_blocks: 17,
            group1_codewords: 22,
            group2_blocks: Some(1),
            group2_codewords: Some(23),
        },
        h_data: VersionEclData {
            data_codewords: 313,
            ec_codewords_per_block: 28,
            group1_blocks: 2,
            group1_codewords: 14,
            group2_blocks: Some(19),
            group2_codewords: Some(15),
        },
    },
    &Version {
        num: 19,
        l_data: VersionEclData {
            data_codewords: 795,
            ec_codewords_per_block: 28,
            group1_blocks: 3,
            group1_codewords: 113,
            group2_blocks: Some(4),
            group2_codewords: Some(114),
        },
        m_data: VersionEclData {
            data_codewords: 627,
            ec_codewords_per_block: 26,
            group1_blocks: 3,
            group1_codewords: 44,
            group2_blocks: Some(11),
            group2_codewords: Some(45),
        },
        q_data: VersionEclData {
            data_codewords: 445,
            ec_codewords_per_block: 26,
            group1_blocks: 17,
            group1_codewords: 21,
            group2_blocks: Some(4),
            group2_codewords: Some(22),
        },
        h_data: VersionEclData {
            data_codewords: 341,
            ec_codewords_per_block: 26,
            group1_blocks: 9,
            group1_codewords: 13,
            group2_blocks: Some(16),
            group2_codewords: Some(14),
        },
    },
    &Version {
        num: 20,
        l_data: VersionEclData {
            data_codewords: 861,
            ec_codewords_per_block: 28,
            group1_blocks: 3,
            group1_codewords: 107,
            group2_blocks: Some(5),
            group2_codewords: Some(108),
        },
        m_data: VersionEclData {
            data_codewords: 669,
            ec_codewords_per_block: 26,
            group1_blocks: 3,
            group1_codewords: 41,
            group2_blocks: Some(13),
            group2_codewords: Some(42),
        },
        q_data: VersionEclData {
            data_codewords: 485,
            ec_codewords_per_block: 30,
            group1_blocks: 15,
            group1_codewords: 24,
            group2_blocks: Some(5),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 385,
            ec_codewords_per_block: 28,
            group1_blocks: 15,
            group1_codewords: 15,
            group2_blocks: Some(10),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 21,
        l_data: VersionEclData {
            data_codewords: 932,
            ec_codewords_per_block: 28,
            group1_blocks: 4,
            group1_codewords: 116,
            group2_blocks: Some(4),
            group2_codewords: Some(117),
        },
        m_data: VersionEclData {
            data_codewords: 714,
            ec_codewords_per_block: 26,
            group1_blocks: 17,
            group1_codewords: 42,
            group2_blocks: None,
            group2_codewords: None,
        },
        q_data: VersionEclData {
            data_codewords: 512,
            ec_codewords_per_block: 28,
            group1_blocks: 17,
            group1_codewords: 22,
            group2_blocks: Some(6),
            group2_codewords: Some(23),
        },
        h_data: VersionEclData {
            data_codewords: 406,
            ec_codewords_per_block: 30,
            group1_blocks: 19,
            group1_codewords: 16,
            group2_blocks: Some(6),
            group2_codewords: Some(17),
        },
    },
    &Version {
        num: 22,
        l_data: VersionEclData {
            data_codewords: 1006,
            ec_codewords_per_block: 28,
            group1_blocks: 2,
            group1_codewords: 111,
            group2_blocks: Some(7),
            group2_codewords: Some(112),
        },
        m_data: VersionEclData {
            data_codewords: 782,
            ec_codewords_per_block: 28,
            group1_blocks: 17,
            group1_codewords: 46,
            group2_blocks: None,
            group2_codewords: None,
        },
        q_data: VersionEclData {
            data_codewords: 568,
            ec_codewords_per_block: 30,
            group1_blocks: 7,
            group1_codewords: 24,
            group2_blocks: Some(16),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 442,
            ec_codewords_per_block: 24,
            group1_blocks: 34,
            group1_codewords: 13,
            group2_blocks: None,
            group2_codewords: None,
        },
    },
    &Version {
        num: 23,
        l_data: VersionEclData {
            data_codewords: 1094,
            ec_codewords_per_block: 30,
            group1_blocks: 4,
            group1_codewords: 121,
            group2_blocks: Some(5),
            group2_codewords: Some(122),
        },
        m_data: VersionEclData {
            data_codewords: 860,
            ec_codewords_per_block: 28,
            group1_blocks: 4,
            group1_codewords: 47,
            group2_blocks: Some(14),
            group2_codewords: Some(48),
        },
        q_data: VersionEclData {
            data_codewords: 614,
            ec_codewords_per_block: 30,
            group1_blocks: 11,
            group1_codewords: 24,
            group2_blocks: Some(14),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 464,
            ec_codewords_per_block: 30,
            group1_blocks: 16,
            group1_codewords: 15,
            group2_blocks: Some(14),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 24,
        l_data: VersionEclData {
            data_codewords: 1174,
            ec_codewords_per_block: 30,
            group1_blocks: 6,
            group1_codewords: 117,
            group2_blocks: Some(4),
            group2_codewords: Some(118),
        },
        m_data: VersionEclData {
            data_codewords: 914,
            ec_codewords_per_block: 28,
            group1_blocks: 6,
            group1_codewords: 45,
            group2_blocks: Some(14),
            group2_codewords: Some(46),
        },
        q_data: VersionEclData {
            data_codewords: 664,
            ec_codewords_per_block: 30,
            group1_blocks: 11,
            group1_codewords: 24,
            group2_blocks: Some(16),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 514,
            ec_codewords_per_block: 30,
            group1_blocks: 30,
            group1_codewords: 16,
            group2_blocks: Some(2),
            group2_codewords: Some(17),
        },
    },
    &Version {
        num: 25,
        l_data: VersionEclData {
            data_codewords: 1276,
            ec_codewords_per_block: 26,
            group1_blocks: 8,
            group1_codewords: 106,
            group2_blocks: Some(4),
            group2_codewords: Some(107),
        },
        m_data: VersionEclData {
            data_codewords: 1000,
            ec_codewords_per_block: 28,
            group1_blocks: 8,
            group1_codewords: 47,
            group2_blocks: Some(13),
            group2_codewords: Some(48),
        },
        q_data: VersionEclData {
            data_codewords: 718,
            ec_codewords_per_block: 30,
            group1_blocks: 7,
            group1_codewords: 24,
            group2_blocks: Some(22),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 538,
            ec_codewords_per_block: 30,
            group1_blocks: 22,
            group1_codewords: 15,
            group2_blocks: Some(13),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 26,
        l_data: VersionEclData {
            data_codewords: 1370,
            ec_codewords_per_block: 28,
            group1_blocks: 10,
            group1_codewords: 114,
            group2_blocks: Some(2),
            group2_codewords: Some(115),
        },
        m_data: VersionEclData {
            data_codewords: 1062,
            ec_codewords_per_block: 28,
            group1_blocks: 19,
            group1_codewords: 46,
            group2_blocks: Some(4),
            group2_codewords: Some(47),
        },
        q_data: VersionEclData {
            data_codewords: 754,
            ec_codewords_per_block: 28,
            group1_blocks: 28,
            group1_codewords: 22,
            group2_blocks: Some(6),
            group2_codewords: Some(23),
        },
        h_data: VersionEclData {
            data_codewords: 596,
            ec_codewords_per_block: 30,
            group1_blocks: 33,
            group1_codewords: 16,
            group2_blocks: Some(4),
            group2_codewords: Some(17),
        },
    },
    &Version {
        num: 27,
        l_data: VersionEclData {
            data_codewords: 1468,
            ec_codewords_per_block: 30,
            group1_blocks: 8,
            group1_codewords: 122,
            group2_blocks: Some(4),
            group2_codewords: Some(123),
        },
        m_data: VersionEclData {
            data_codewords: 1128,
            ec_codewords_per_block: 28,
            group1_blocks: 22,
            group1_codewords: 45,
            group2_blocks: Some(3),
            group2_codewords: Some(46),
        },
        q_data: VersionEclData {
            data_codewords: 808,
            ec_codewords_per_block: 30,
            group1_blocks: 8,
            group1_codewords: 23,
            group2_blocks: Some(26),
            group2_codewords: Some(24),
        },
        h_data: VersionEclData {
            data_codewords: 628,
            ec_codewords_per_block: 30,
            group1_blocks: 12,
            group1_codewords: 15,
            group2_blocks: Some(28),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 28,
        l_data: VersionEclData {
            data_codewords: 1531,
            ec_codewords_per_block: 30,
            group1_blocks: 3,
            group1_codewords: 117,
            group2_blocks: Some(10),
            group2_codewords: Some(118),
        },
        m_data: VersionEclData {
            data_codewords: 1193,
            ec_codewords_per_block: 28,
            group1_blocks: 3,
            group1_codewords: 45,
            group2_blocks: Some(23),
            group2_codewords: Some(46),
        },
        q_data: VersionEclData {
            data_codewords: 871,
            ec_codewords_per_block: 30,
            group1_blocks: 4,
            group1_codewords: 24,
            group2_blocks: Some(31),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 661,
            ec_codewords_per_block: 30,
            group1_blocks: 11,
            group1_codewords: 15,
            group2_blocks: Some(31),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 29,
        l_data: VersionEclData {
            data_codewords: 1631,
            ec_codewords_per_block: 30,
            group1_blocks: 7,
            group1_codewords: 116,
            group2_blocks: Some(7),
            group2_codewords: Some(117),
        },
        m_data: VersionEclData {
            data_codewords: 1267,
            ec_codewords_per_block: 28,
            group1_blocks: 21,
            group1_codewords: 45,
            group2_blocks: Some(7),
            group2_codewords: Some(46),
        },
        q_data: VersionEclData {
            data_codewords: 911,
            ec_codewords_per_block: 30,
            group1_blocks: 1,
            group1_codewords: 23,
            group2_blocks: Some(37),
            group2_codewords: Some(24),
        },
        h_data: VersionEclData {
            data_codewords: 701,
            ec_codewords_per_block: 30,
            group1_blocks: 19,
            group1_codewords: 15,
            group2_blocks: Some(26),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 30,
        l_data: VersionEclData {
            data_codewords: 1735,
            ec_codewords_per_block: 30,
            group1_blocks: 5,
            group1_codewords: 115,
            group2_blocks: Some(10),
            group2_codewords: Some(116),
        },
        m_data: VersionEclData {
            data_codewords: 1373,
            ec_codewords_per_block: 28,
            group1_blocks: 19,
            group1_codewords: 47,
            group2_blocks: Some(10),
            group2_codewords: Some(48),
        },
        q_data: VersionEclData {
            data_codewords: 985,
            ec_codewords_per_block: 30,
            group1_blocks: 15,
            group1_codewords: 24,
            group2_blocks: Some(25),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 745,
            ec_codewords_per_block: 30,
            group1_blocks: 23,
            group1_codewords: 15,
            group2_blocks: Some(25),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 31,
        l_data: VersionEclData {
            data_codewords: 1843,
            ec_codewords_per_block: 30,
            group1_blocks: 13,
            group1_codewords: 115,
            group2_blocks: Some(3),
            group2_codewords: Some(116),
        },
        m_data: VersionEclData {
            data_codewords: 1455,
            ec_codewords_per_block: 28,
            group1_blocks: 2,
            group1_codewords: 46,
            group2_blocks: Some(29),
            group2_codewords: Some(47),
        },
        q_data: VersionEclData {
            data_codewords: 1033,
            ec_codewords_per_block: 30,
            group1_blocks: 42,
            group1_codewords: 24,
            group2_blocks: Some(1),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 793,
            ec_codewords_per_block: 30,
            group1_blocks: 23,
            group1_codewords: 15,
            group2_blocks: Some(28),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 32,
        l_data: VersionEclData {
            data_codewords: 1955,
            ec_codewords_per_block: 30,
            group1_blocks: 17,
            group1_codewords: 115,
            group2_blocks: None,
            group2_codewords: None,
        },
        m_data: VersionEclData {
            data_codewords: 1541,
            ec_codewords_per_block: 28,
            group1_blocks: 10,
            group1_codewords: 46,
            group2_blocks: Some(23),
            group2_codewords: Some(47),
        },
        q_data: VersionEclData {
            data_codewords: 1115,
            ec_codewords_per_block: 30,
            group1_blocks: 10,
            group1_codewords: 24,
            group2_blocks: Some(35),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 845,
            ec_codewords_per_block: 30,
            group1_blocks: 19,
            group1_codewords: 15,
            group2_blocks: Some(35),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 33,
        l_data: VersionEclData {
            data_codewords: 2071,
            ec_codewords_per_block: 30,
            group1_blocks: 17,
            group1_codewords: 115,
            group2_blocks: Some(1),
            group2_codewords: Some(116),
        },
        m_data: VersionEclData {
            data_codewords: 1631,
            ec_codewords_per_block: 28,
            group1_blocks: 14,
            group1_codewords: 46,
            group2_blocks: Some(21),
            group2_codewords: Some(47),
        },
        q_data: VersionEclData {
            data_codewords: 1171,
            ec_codewords_per_block: 30,
            group1_blocks: 29,
            group1_codewords: 24,
            group2_blocks: Some(19),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 901,
            ec_codewords_per_block: 30,
            group1_blocks: 11,
            group1_codewords: 15,
            group2_blocks: Some(46),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 34,
        l_data: VersionEclData {
            data_codewords: 2191,
            ec_codewords_per_block: 30,
            group1_blocks: 13,
            group1_codewords: 115,
            group2_blocks: Some(6),
            group2_codewords: Some(116),
        },
        m_data: VersionEclData {
            data_codewords: 1725,
            ec_codewords_per_block: 28,
            group1_blocks: 14,
            group1_codewords: 46,
            group2_blocks: Some(23),
            group2_codewords: Some(47),
        },
        q_data: VersionEclData {
            data_codewords: 1231,
            ec_codewords_per_block: 30,
            group1_blocks: 44,
            group1_codewords: 24,
            group2_blocks: Some(7),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 961,
            ec_codewords_per_block: 30,
            group1_blocks: 59,
            group1_codewords: 16,
            group2_blocks: Some(1),
            group2_codewords: Some(17),
        },
    },
    &Version {
        num: 35,
        l_data: VersionEclData {
            data_codewords: 2306,
            ec_codewords_per_block: 30,
            group1_blocks: 12,
            group1_codewords: 121,
            group2_blocks: Some(7),
            group2_codewords: Some(122),
        },
        m_data: VersionEclData {
            data_codewords: 1812,
            ec_codewords_per_block: 28,
            group1_blocks: 12,
            group1_codewords: 47,
            group2_blocks: Some(26),
            group2_codewords: Some(48),
        },
        q_data: VersionEclData {
            data_codewords: 1286,
            ec_codewords_per_block: 30,
            group1_blocks: 39,
            group1_codewords: 24,
            group2_blocks: Some(14),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 986,
            ec_codewords_per_block: 30,
            group1_blocks: 22,
            group1_codewords: 15,
            group2_blocks: Some(41),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 36,
        l_data: VersionEclData {
            data_codewords: 2434,
            ec_codewords_per_block: 30,
            group1_blocks: 6,
            group1_codewords: 121,
            group2_blocks: Some(14),
            group2_codewords: Some(122),
        },
        m_data: VersionEclData {
            data_codewords: 1914,
            ec_codewords_per_block: 28,
            group1_blocks: 6,
            group1_codewords: 47,
            group2_blocks: Some(34),
            group2_codewords: Some(48),
        },
        q_data: VersionEclData {
            data_codewords: 1354,
            ec_codewords_per_block: 30,
            group1_blocks: 46,
            group1_codewords: 24,
            group2_blocks: Some(10),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 1054,
            ec_codewords_per_block: 30,
            group1_blocks: 2,
            group1_codewords: 15,
            group2_blocks: Some(64),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 37,
        l_data: VersionEclData {
            data_codewords: 2566,
            ec_codewords_per_block: 30,
            group1_blocks: 17,
            group1_codewords: 122,
            group2_blocks: Some(4),
            group2_codewords: Some(123),
        },
        m_data: VersionEclData {
            data_codewords: 1992,
            ec_codewords_per_block: 28,
            group1_blocks: 29,
            group1_codewords: 46,
            group2_blocks: Some(14),
            group2_codewords: Some(47),
        },
        q_data: VersionEclData {
            data_codewords: 1426,
            ec_codewords_per_block: 30,
            group1_blocks: 49,
            group1_codewords: 24,
            group2_blocks: Some(10),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 1096,
            ec_codewords_per_block: 30,
            group1_blocks: 24,
            group1_codewords: 15,
            group2_blocks: Some(46),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 38,
        l_data: VersionEclData {
            data_codewords: 2702,
            ec_codewords_per_block: 30,
            group1_blocks: 4,
            group1_codewords: 122,
            group2_blocks: Some(18),
            group2_codewords: Some(123),
        },
        m_data: VersionEclData {
            data_codewords: 2102,
            ec_codewords_per_block: 28,
            group1_blocks: 13,
            group1_codewords: 46,
            group2_blocks: Some(32),
            group2_codewords: Some(47),
        },
        q_data: VersionEclData {
            data_codewords: 1502,
            ec_codewords_per_block: 30,
            group1_blocks: 48,
            group1_codewords: 24,
            group2_blocks: Some(14),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 1142,
            ec_codewords_per_block: 30,
            group1_blocks: 42,
            group1_codewords: 15,
            group2_blocks: Some(32),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 39,
        l_data: VersionEclData {
            data_codewords: 2812,
            ec_codewords_per_block: 30,
            group1_blocks: 20,
            group1_codewords: 117,
            group2_blocks: Some(4),
            group2_codewords: Some(118),
        },
        m_data: VersionEclData {
            data_codewords: 2216,
            ec_codewords_per_block: 28,
            group1_blocks: 40,
            group1_codewords: 47,
            group2_blocks: Some(7),
            group2_codewords: Some(48),
        },
        q_data: VersionEclData {
            data_codewords: 1582,
            ec_codewords_per_block: 30,
            group1_blocks: 43,
            group1_codewords: 24,
            group2_blocks: Some(22),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 1222,
            ec_codewords_per_block: 30,
            group1_blocks: 10,
            group1_codewords: 15,
            group2_blocks: Some(67),
            group2_codewords: Some(16),
        },
    },
    &Version {
        num: 40,
        l_data: VersionEclData {
            data_codewords: 2956,
            ec_codewords_per_block: 30,
            group1_blocks: 19,
            group1_codewords: 118,
            group2_blocks: Some(6),
            group2_codewords: Some(119),
        },
        m_data: VersionEclData {
            data_codewords: 2334,
            ec_codewords_per_block: 28,
            group1_blocks: 18,
            group1_codewords: 47,
            group2_blocks: Some(31),
            group2_codewords: Some(48),
        },
        q_data: VersionEclData {
            data_codewords: 1666,
            ec_codewords_per_block: 30,
            group1_blocks: 34,
            group1_codewords: 24,
            group2_blocks: Some(34),
            group2_codewords: Some(25),
        },
        h_data: VersionEclData {
            data_codewords: 1276,
            ec_codewords_per_block: 30,
            group1_blocks: 20,
            group1_codewords: 15,
            group2_blocks: Some(61),
            group2_codewords: Some(16),
        },
    },
];

pub fn choose_version(
    encoder: &QRBitstreamEncoder,
    ecl: ErrorCorrectionLevel,
) -> Result<&'static Version, Error> {
    for version in VERSIONS.iter() {
        let codewords = encoder.codeword_count_before_padding(version.num);
        let cap = version.codeword_count(&ecl);
        if codewords < cap {
            return Ok(version);
        }
    }
    Err(Cow::from(
        "The data is too long for a QR code at that error correction level!",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modules_per_side() {
        assert_eq!(Version::by_num(1).modules_per_side(), 21);
        assert_eq!(Version::by_num(6).modules_per_side(), 41);
        assert_eq!(Version::by_num(40).modules_per_side(), 177);
    }

    #[test]
    fn test_choose_version_low() {
        let encoder = QRBitstreamEncoder::new("12300001010");
        assert_eq!(
            choose_version(&encoder, ErrorCorrectionLevel::Low)
                .unwrap()
                .num,
            1
        );
    }

    #[test]
    fn test_choose_version_medium() {
        let encoder = QRBitstreamEncoder::new("12300001010ASKOIDGOAS");
        assert_eq!(
            choose_version(&encoder, ErrorCorrectionLevel::Medium)
                .unwrap()
                .num,
            2
        );
    }

    #[test]
    fn test_choose_version_quartile() {
        let encoder = QRBitstreamEncoder::new("12300001010asdfgbasdfsadfASAEDFGSDGSDG");
        assert_eq!(
            choose_version(&encoder, ErrorCorrectionLevel::Quartile)
                .unwrap()
                .num,
            4
        );
    }

    #[test]
    fn test_choose_version_high() {
        let encoder = QRBitstreamEncoder::new(
            "Pi π = 3.1415926535897932384626433832795028841971693993751058209749445923078164\
            0628620899862803482534211706798214808651328230664709384460955058223172535940812848111\
            7450284102701938521105559644622948954930381964428810975665933446128475648233786783165\
            2712019091456485669234603486104543266482133936072602491412737245870066063155881748815\
            2092096282925409171536436789259036001133053054882046652138414695194151160943305727036\
            5759591953092186117381932611793105118548074462379962749567351885752724891227938183011\
            9491298336733624406566430860213949463952247371907021798609437027705392171762931767523\
            8467481846766940513200056812714526356082778577134275778960917363717872146844090122495\
            3430146549585371050792279689258923542019956112129021960864034418159813629774771309960\
            5187072113499999983729780499510597317328160963185950244594553469083026425223082533446\
            8503526193118817101000313783875288658753320838142061717766914730359825349042875546873\
            1159562863882353787593751957781857780532171226806613001927876611195909216420198938095\
            2572010654858632788659361533818279682303019520353018529689957736225994138912497217752\
            8347913151557485724245...",
        );
        assert_eq!(
            choose_version(&encoder, ErrorCorrectionLevel::High)
                .unwrap()
                .num,
            38
        );
    }
}
