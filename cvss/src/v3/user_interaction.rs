use crate::error::{CVSSError, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
// UI
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserInteractionType {
  // UI:R
  Required,
  // UI:N
  None,
}
impl FromStr for UserInteractionType {
  type Err = CVSSError;

  fn from_str(s: &str) -> Result<Self> {
    let mut s = s.to_uppercase();
    if s.starts_with("UI:") {
      s = s.strip_prefix("UI:").unwrap_or_default().to_string();
    }
    let c = {
      let c = s.to_uppercase().chars().next();
      c.ok_or(CVSSError::InvalidCVSS {
        value: s,
        scope: "UserInteractionType from_str".to_string(),
      })?
    };
    match c {
      'N' => Ok(Self::None),
      'R' => Ok(Self::Required),
      _ => Err(CVSSError::InvalidCVSS {
        value: c.to_string(),
        scope: "UserInteractionType".to_string(),
      }),
    }
  }
}
