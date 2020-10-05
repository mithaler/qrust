use crate::qr::encode::QRBitstreamEncoder;
use crate::qr::error_correction::ErrorCorrectionLevel;
use crate::qr::pattern::QRCode;
use crate::qr::version::choose_version;
use crate::qr::Error;

pub mod qr;

pub fn create_qr_code(data: &str, ecl: ErrorCorrectionLevel) -> Result<QRCode, Error> {
    let mut bitstream = QRBitstreamEncoder::new(data);
    let version = choose_version(&bitstream, &ecl)?;
    let data = bitstream.bitstream(version, &ecl)?;
    Ok(QRCode::new(version, data))
}

#[cfg(test)]
pub(self) fn _read_fixture<T: serde::de::DeserializeOwned>(path: &str) -> T {
    let mut fixture_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixture_file.push(path);
    serde_yaml::from_reader(std::fs::File::open(fixture_file).unwrap()).unwrap()
}
