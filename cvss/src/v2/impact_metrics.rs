//! CIA Impact Metric
//!
//! The three impact metrics measure how a vulnerability, if exploited, will directly affect an IT asset, where the impacts are independently defined as the degree of loss of confidentiality, integrity, and availability. For example, a vulnerability could cause a partial loss of integrity and availability, but no loss of confidentiality.
//!
use crate::error::{CVSSError, Result};
use crate::metric::{Metric, MetricType, MetricTypeV2};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// ### 2.1.4. Confidentiality Impact (C)
///
/// This metric measures the impact on confidentiality of a successfully exploited vulnerability. Confidentiality refers to limiting information access and disclosure to only authorized users, as well as preventing access by, or disclosure to, unauthorized ones. The possible values for this metric are listed in Table 4. Increased confidentiality impact increases the vulnerability score.
///
/// | Metric Value | Description |
/// | --- | --- |
/// | None (N) | There is no impact to the confidentiality of the system. |
/// | Partial (P) | There is considerable informational disclosure. Access to some system files is possible, but the attacker does not have control over what is obtained, or the scope of the loss is constrained. An example is a vulnerability that divulges only certain tables in a database. |
/// | Complete (C) | There is total information disclosure, resulting in all system files being revealed. The attacker is able to read all of the system's data (memory, files, etc.) |
///
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ConfidentialityImpactType {
  /// none: 0.0
  None,
  /// partial: 0.275
  Partial,
  /// complete: 0.660
  Complete,
}

/// ### 2.1.5. Integrity Impact (I)
///
/// This metric measures the impact to integrity of a successfully exploited vulnerability. Integrity refers to the trustworthiness and guaranteed veracity of information. The possible values for this metric are listed in Table 5. Increased integrity impact increases the vulnerability score.
///
/// | Metric Value | Description |
/// | --- | --- |
/// | None (N) | There is no impact to the integrity of the system. |
/// | Partial (P) | Modification of some system files or information is possible, but the attacker does not have control over what can be modified, or the scope of what the attacker can affect is limited. For example, system or application files may be overwritten or modified, but either the attacker has no control over which files are affected or the attacker can modify files within only a limited context or scope. |
/// | Complete (C) | There is a total compromise of system integrity. There is a complete loss of system protection, resulting in the entire system being compromised. The attacker is able to modify any files on the target system. |
///
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum IntegrityImpactType {
  /// none: 0.0
  None,
  /// partial: 0.275
  Partial,
  /// complete: 0.660
  Complete,
}

/// ### 2.1.6 Availability Impact (A)
///
/// This metric measures the impact to availability of a successfully exploited vulnerability. Availability refers to the accessibility of information resources. Attacks that consume network bandwidth, processor cycles, or disk space all impact the availability of a system. The possible values for this metric are listed in Table 6. Increased availability impact increases the vulnerability score.
///
/// | Metric Value | Description |
/// | --- | --- |
/// | None (N) | There is no impact to the availability of the system. |
/// | Partial (P) | There is reduced performance or interruptions in resource availability. An example is a network-based flood attack that permits a limited number of successful connections to an Internet service. |
/// | Complete (C) | There is a total shutdown of the affected resource. The attacker can render the resource completely unavailable. |
///
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AvailabilityImpactType {
  /// none: 0.0
  None,
  /// partial: 0.275
  Partial,
  /// complete: 0.660
  Complete,
}
impl FromStr for ConfidentialityImpactType {
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
      let c = s.to_uppercase().chars().next();
      c.ok_or(CVSSError::InvalidCVSS {
        value: s,
        scope: Self::description(),
      })?
    };
    match c {
      'N' => Ok(Self::None),
      'P' => Ok(Self::Partial),
      'C' => Ok(Self::Complete),
      _ => Err(CVSSError::InvalidCVSS {
        value: c.to_string(),
        scope: Self::description(),
      }),
    }
  }
}
impl FromStr for IntegrityImpactType {
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
      let c = s.to_uppercase().chars().next();
      c.ok_or(CVSSError::InvalidCVSS {
        value: s,
        scope: Self::description(),
      })?
    };
    match c {
      'N' => Ok(Self::None),
      'P' => Ok(Self::Partial),
      'C' => Ok(Self::Complete),
      _ => Err(CVSSError::InvalidCVSS {
        value: c.to_string(),
        scope: Self::description(),
      }),
    }
  }
}
impl FromStr for AvailabilityImpactType {
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
      let c = s.to_uppercase().chars().next();
      c.ok_or(CVSSError::InvalidCVSS {
        value: s,
        scope: Self::description(),
      })?
    };
    match c {
      'N' => Ok(Self::None),
      'P' => Ok(Self::Partial),
      'C' => Ok(Self::Complete),
      _ => Err(CVSSError::InvalidCVSS {
        value: c.to_string(),
        scope: Self::description(),
      }),
    }
  }
}

impl Display for ConfidentialityImpactType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", Self::name(), self.as_str())
  }
}

impl Metric for ConfidentialityImpactType {
  const TYPE: MetricType = MetricType::V2(MetricTypeV2::C);

  fn score(&self) -> f32 {
    match self {
      ConfidentialityImpactType::None => 0.0,
      ConfidentialityImpactType::Partial => 0.275,
      ConfidentialityImpactType::Complete => 0.660,
    }
  }

  fn as_str(&self) -> &'static str {
    match self {
      ConfidentialityImpactType::None => "N",
      ConfidentialityImpactType::Partial => "P",
      ConfidentialityImpactType::Complete => "C",
    }
  }
}

impl Display for IntegrityImpactType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", Self::name(), self.as_str())
  }
}

impl Metric for IntegrityImpactType {
  const TYPE: MetricType = MetricType::V2(MetricTypeV2::I);

  fn score(&self) -> f32 {
    match self {
      IntegrityImpactType::None => 0.0,
      IntegrityImpactType::Partial => 0.275,
      IntegrityImpactType::Complete => 0.660,
    }
  }

  fn as_str(&self) -> &'static str {
    match self {
      IntegrityImpactType::None => "N",
      IntegrityImpactType::Partial => "P",
      IntegrityImpactType::Complete => "C",
    }
  }
}

impl Display for AvailabilityImpactType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", Self::name(), self.as_str())
  }
}

impl Metric for AvailabilityImpactType {
  const TYPE: MetricType = MetricType::V2(MetricTypeV2::A);

  fn score(&self) -> f32 {
    match self {
      AvailabilityImpactType::None => 0.0,
      AvailabilityImpactType::Partial => 0.275,
      AvailabilityImpactType::Complete => 0.660,
    }
  }

  fn as_str(&self) -> &'static str {
    match self {
      AvailabilityImpactType::None => "N",
      AvailabilityImpactType::Partial => "P",
      AvailabilityImpactType::Complete => "C",
    }
  }
}
