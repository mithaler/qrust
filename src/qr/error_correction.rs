use std::str::FromStr;

#[derive(Debug)]
pub enum ErrorCorrectionLevel {
    Low,
    Medium,
    Quartile,
    High,
}

impl FromStr for ErrorCorrectionLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(ErrorCorrectionLevel::Low),
            "medium" => Ok(ErrorCorrectionLevel::Medium),
            "quartile" => Ok(ErrorCorrectionLevel::Quartile),
            "high" => Ok(ErrorCorrectionLevel::High),
            _ => Err(format!(
                "Unknown error correction level {} (options are low, medium, quartile, high)",
                s
            )),
        }
    }
}
