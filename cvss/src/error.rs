//! cvss error
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CVSSError>;

/// CVSSError
#[derive(Error, Debug, Clone)]
pub enum CVSSError {
  #[error("error decoding value `{value}`, not well formed UTF-8")]
  Utf8Error {
    #[source]
    source: std::str::Utf8Error,
    value: String,
  },

  #[error("invalid prefix for `{value}`")]
  InvalidPrefix { value: String },
  #[error("Invalid CVSS `{value}` at {scope}")]
  InvalidCVSS { value: String, scope: String },
  #[error("invalid cvss version `{value}` ({expected})")]
  InvalidCVSSVersion { value: String, expected: String },
  #[error("unknown CVSS metric name: `{name}`")]
  UnknownMetric { name: String },
}

impl From<&CVSSError> for CVSSError {
  fn from(value: &CVSSError) -> Self {
    value.clone()
  }
}
