use std::borrow::Cow;

use crate::qr::encode::QRBitstreamEncoder;
use crate::qr::error_correction::ErrorCorrectionLevel;
use crate::qr::Error;

/// A QR code version. All caps are codeword counts.
#[derive(Debug)]
pub struct Version {
    pub num: u8,
    l_cap: usize,
    m_cap: usize,
    q_cap: usize,
    h_cap: usize,
}

/// A QR code version, numbered 1 to 40.
impl Version {
    /// Looks up a version by its number.
    pub fn by_num(num: usize) -> &'static Version {
        VERSIONS[num - 1]
    }

    pub fn codeword_count(&self, ecl: &ErrorCorrectionLevel) -> usize {
        match ecl {
            ErrorCorrectionLevel::Low => self.l_cap,
            ErrorCorrectionLevel::Medium => self.m_cap,
            ErrorCorrectionLevel::Quartile => self.q_cap,
            ErrorCorrectionLevel::High => self.h_cap,
        }
    }

    /// Returns the number of modules on a single side of the finished QR code.
    pub fn modules_per_side(&self) -> u32 {
        (4 * (self.num as u32 - 1)) + 21
    }
}

const VERSIONS: [&Version; 40] = [
    &Version {
        num: 1,
        l_cap: 19,
        m_cap: 16,
        q_cap: 13,
        h_cap: 9,
    },
    &Version {
        num: 2,
        l_cap: 34,
        m_cap: 28,
        q_cap: 22,
        h_cap: 16,
    },
    &Version {
        num: 3,
        l_cap: 55,
        m_cap: 44,
        q_cap: 34,
        h_cap: 26,
    },
    &Version {
        num: 4,
        l_cap: 80,
        m_cap: 64,
        q_cap: 48,
        h_cap: 36,
    },
    &Version {
        num: 5,
        l_cap: 108,
        m_cap: 86,
        q_cap: 62,
        h_cap: 46,
    },
    &Version {
        num: 6,
        l_cap: 136,
        m_cap: 108,
        q_cap: 76,
        h_cap: 60,
    },
    &Version {
        num: 7,
        l_cap: 156,
        m_cap: 124,
        q_cap: 88,
        h_cap: 66,
    },
    &Version {
        num: 8,
        l_cap: 194,
        m_cap: 154,
        q_cap: 110,
        h_cap: 86,
    },
    &Version {
        num: 9,
        l_cap: 232,
        m_cap: 182,
        q_cap: 132,
        h_cap: 100,
    },
    &Version {
        num: 10,
        l_cap: 274,
        m_cap: 216,
        q_cap: 154,
        h_cap: 122,
    },
    &Version {
        num: 11,
        l_cap: 324,
        m_cap: 254,
        q_cap: 180,
        h_cap: 140,
    },
    &Version {
        num: 12,
        l_cap: 370,
        m_cap: 290,
        q_cap: 206,
        h_cap: 158,
    },
    &Version {
        num: 13,
        l_cap: 428,
        m_cap: 334,
        q_cap: 244,
        h_cap: 180,
    },
    &Version {
        num: 14,
        l_cap: 461,
        m_cap: 365,
        q_cap: 261,
        h_cap: 197,
    },
    &Version {
        num: 15,
        l_cap: 523,
        m_cap: 415,
        q_cap: 295,
        h_cap: 223,
    },
    &Version {
        num: 16,
        l_cap: 589,
        m_cap: 453,
        q_cap: 325,
        h_cap: 253,
    },
    &Version {
        num: 17,
        l_cap: 647,
        m_cap: 507,
        q_cap: 367,
        h_cap: 283,
    },
    &Version {
        num: 18,
        l_cap: 721,
        m_cap: 563,
        q_cap: 397,
        h_cap: 313,
    },
    &Version {
        num: 19,
        l_cap: 795,
        m_cap: 627,
        q_cap: 445,
        h_cap: 341,
    },
    &Version {
        num: 20,
        l_cap: 861,
        m_cap: 669,
        q_cap: 485,
        h_cap: 385,
    },
    &Version {
        num: 21,
        l_cap: 932,
        m_cap: 714,
        q_cap: 512,
        h_cap: 406,
    },
    &Version {
        num: 22,
        l_cap: 1006,
        m_cap: 782,
        q_cap: 568,
        h_cap: 442,
    },
    &Version {
        num: 23,
        l_cap: 1094,
        m_cap: 860,
        q_cap: 614,
        h_cap: 464,
    },
    &Version {
        num: 24,
        l_cap: 1174,
        m_cap: 914,
        q_cap: 664,
        h_cap: 514,
    },
    &Version {
        num: 25,
        l_cap: 1276,
        m_cap: 1000,
        q_cap: 718,
        h_cap: 538,
    },
    &Version {
        num: 26,
        l_cap: 1370,
        m_cap: 1062,
        q_cap: 754,
        h_cap: 596,
    },
    &Version {
        num: 27,
        l_cap: 1468,
        m_cap: 1128,
        q_cap: 808,
        h_cap: 628,
    },
    &Version {
        num: 28,
        l_cap: 1531,
        m_cap: 1193,
        q_cap: 871,
        h_cap: 661,
    },
    &Version {
        num: 29,
        l_cap: 1631,
        m_cap: 1267,
        q_cap: 911,
        h_cap: 701,
    },
    &Version {
        num: 30,
        l_cap: 1735,
        m_cap: 1373,
        q_cap: 985,
        h_cap: 745,
    },
    &Version {
        num: 31,
        l_cap: 1843,
        m_cap: 1455,
        q_cap: 1033,
        h_cap: 793,
    },
    &Version {
        num: 32,
        l_cap: 1955,
        m_cap: 1541,
        q_cap: 1115,
        h_cap: 845,
    },
    &Version {
        num: 33,
        l_cap: 2071,
        m_cap: 1631,
        q_cap: 1171,
        h_cap: 901,
    },
    &Version {
        num: 34,
        l_cap: 2191,
        m_cap: 1725,
        q_cap: 1231,
        h_cap: 961,
    },
    &Version {
        num: 35,
        l_cap: 2306,
        m_cap: 1812,
        q_cap: 1286,
        h_cap: 986,
    },
    &Version {
        num: 36,
        l_cap: 2434,
        m_cap: 1914,
        q_cap: 1354,
        h_cap: 1054,
    },
    &Version {
        num: 37,
        l_cap: 2566,
        m_cap: 1992,
        q_cap: 1426,
        h_cap: 1096,
    },
    &Version {
        num: 38,
        l_cap: 2702,
        m_cap: 2102,
        q_cap: 1502,
        h_cap: 1142,
    },
    &Version {
        num: 39,
        l_cap: 2812,
        m_cap: 2216,
        q_cap: 1582,
        h_cap: 1222,
    },
    &Version {
        num: 40,
        l_cap: 2956,
        m_cap: 2334,
        q_cap: 1666,
        h_cap: 1276,
    },
];

pub fn choose_version(
    encoder: &QRBitstreamEncoder,
    ecl: ErrorCorrectionLevel,
) -> Result<&'static Version, Error> {
    for version in VERSIONS.iter() {
        let codewords = encoder.codeword_count_before_padding(version.num);
        let cap = match ecl {
            ErrorCorrectionLevel::Low => version.l_cap,
            ErrorCorrectionLevel::Medium => version.m_cap,
            ErrorCorrectionLevel::Quartile => version.q_cap,
            ErrorCorrectionLevel::High => version.h_cap,
        };
        if codewords < cap {
            return Ok(version);
        }
    }
    Err(Cow::Borrowed(
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
