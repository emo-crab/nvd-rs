//! ### 2.1.2. Access Complexity (AC)
//!
//! This metric measures the complexity of the attack required to exploit the vulnerability once an attacker has gained access to the target system. For example, consider a buffer overflow in an Internet service: once the target system is located, the attacker can launch an exploit at will.
//!
//! Other vulnerabilities, however, may require additional steps in order to be exploited. For example, a vulnerability in an email client is only exploited after the user downloads and opens a tainted attachment. The possible values for this metric are listed in Table 2. The lower the required complexity, the higher the vulnerability score.
//!
//! | Metric Value | Description |
//! | --- | --- |
//! | High (H) | Specialized access conditions exist. For example:
//! ||\- In most configurations, the attacking party must already have elevated privileges or spoof additional systems in addition to the attacking system (e.g., DNS hijacking).
//! ||\- The attack depends on social engineering methods that would be easily detected by knowledgeable people. For example, the victim must perform several suspicious or atypical actions.
//! ||\- The vulnerable configuration is seen very rarely in practice.
//! ||\- If a race condition exists, the window is very narrow. |
//! | Medium (M) | The access conditions are somewhat specialized; the following are examples:
//! ||\- The attacking party is limited to a group of systems or users at some level of authorization, possibly untrusted.
//! ||\- Some information must be gathered before a successful attack can be launched.
//! ||\- The affected configuration is non-default, and is not commonly configured (e.g., a vulnerability present when a server performs user account authentication via a specific scheme, but not present for another authentication scheme).
//! ||\- The attack requires a small amount of social engineering that might occasionally fool cautious users (e.g., phishing attacks that modify a web browsers status bar to show a false link, having to be on someones buddy list before sending an IM exploit). |
//! | Low (L) | Specialized access conditions or extenuating circumstances do not exist. The following are examples:
//! ||\- The affected product typically requires access to a wide range of systems and users, possibly anonymous and untrusted (e.g., Internet-facing web or mail server).
//! ||\- The affected configuration is default or ubiquitous.
//! ||\- The attack can be performed manually and requires little skill or additional information gathering.
//! ||\- The race condition is a lazy one (i.e., it is technically a race but easily winnable). |
//!

use crate::error::{CVSSError, Result};
use crate::metric::{Metric, MetricType, MetricTypeV3};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// AccessComplexity
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccessComplexityType {
  /// high: 0.35
  High,
  /// medium: 0.61
  Medium,
  /// low: 0.71
  Low,
}
impl Display for AccessComplexityType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", Self::name(), self.as_str())
  }
}

impl Metric for AccessComplexityType {
  const TYPE: MetricType = MetricType::V3(MetricTypeV3::AC);

  fn score(&self) -> f32 {
    match self {
      AccessComplexityType::High => 0.35,
      AccessComplexityType::Medium => 0.61,
      AccessComplexityType::Low => 0.71,
    }
  }

  fn as_str(&self) -> &'static str {
    match self {
      AccessComplexityType::High => "H",
      AccessComplexityType::Medium => "M",
      AccessComplexityType::Low => "L",
    }
  }
}
impl FromStr for AccessComplexityType {
  type Err = CVSSError;

  fn from_str(s: &str) -> Result<Self> {
    let mut s = s.to_uppercase();
    if s.starts_with(Self::name()) {
      s = s
        .strip_prefix(&format!("{}:", Self::name()))
        .unwrap_or_default()
        .to_string();
    }
    let c = {
      let c = s.chars().next();
      c.ok_or(CVSSError::InvalidCVSS {
        value: s,
        scope: Self::description(),
      })?
    };
    match c {
      'H' => Ok(Self::High),
      'M' => Ok(Self::Medium),
      'L' => Ok(Self::Low),
      _ => Err(CVSSError::InvalidCVSS {
        value: c.to_string(),
        scope: "AccessComplexityType".to_string(),
      }),
    }
  }
}
