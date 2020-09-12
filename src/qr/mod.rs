use std::borrow::Cow;

pub mod encode;
pub mod error_correction;
pub mod version;

pub type Error = Cow<'static, str>;
