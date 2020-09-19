use crate::qr::encode::QRBitstreamEncoder;
use crate::qr::error_correction::ErrorCorrectionLevel;
use crate::qr::version::choose_version;
use crate::qr::Error;

pub mod qr;

pub fn create_qr_code(data: &str, ecl: ErrorCorrectionLevel) -> Result<(), Error> {
    let bitstream = QRBitstreamEncoder::new(data);
    let version = choose_version(&bitstream, ecl)?;
    println!(
        "encoding: {:#?}, codeword count: {}, version: {}",
        bitstream.encoding,
        bitstream.codeword_count_before_padding(version.num),
        version.num
    );
    Ok(())
}

#[cfg(test)]
pub(self) fn _read_fixture<T: serde::de::DeserializeOwned>(path: &str) -> T {
    let mut fixture_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixture_file.push(path);
    serde_yaml::from_reader(std::fs::File::open(fixture_file).unwrap()).unwrap()
}
