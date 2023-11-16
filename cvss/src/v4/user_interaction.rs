//! ### User Interaction (UI)
//!
//! This metric captures the requirement for a human user, other than the attacker, to participate in the successful compromise of the vulnerable system. This metric determines whether the vulnerability can be exploited solely at the will of the attacker, or whether a separate user (or user-initiated process) must participate in some manner. The resulting score is greatest when no user interaction is required. The list of possible values is presented in Table 5.
//!
//! **Table 5: User Interaction**
//!
//! | **Metric Value** | **Description** |
//! | --- | --- |
//! | None (N) | The vulnerable system can be exploited without interaction from any human user, other than the attacker. Examples include: a remote attacker is able to send packets to a target system a locally authenticated attacker executes code to elevate privileges |
//! | Passive (P) | Successful exploitation of this vulnerability requires limited interaction by the targeted user with the vulnerable system and the attacker’s payload. These interactions would be considered involuntary and do not require that the user actively subvert protections built into the vulnerable system. Examples include: utilizing a website that has been modified to display malicious content when the page is rendered (most stored XSS or CSRF) running an application that calls a malicious binary that has been planted on the system using an application which generates traffic over an untrusted or compromised network (vulnerabilities requiring an on-path attacker) |
//! | Active (A) | Successful exploitation of this vulnerability requires a targeted user to perform specific, conscious interactions with the vulnerable system and the attacker’s payload, or the user’s interactions would actively subvert protection mechanisms which would lead to exploitation of the vulnerability. Examples include: importing a file into a vulnerable system in a specific manner placing files into a specific directory prior to executing code submitting a specific string into a web application (e.g. reflected or self XSS) dismiss or accept prompts or security warnings prior to taking an action (e.g. opening/editing a file, connecting a device). |
//!

use crate::error::{CVSSError, Result};
use crate::metric::{Help, Metric, MetricType, MetricTypeV3, Worth};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// User Interaction (UI) 用户交互
///
/// 此指标描述攻击脆弱组件对除攻击者之外的用户参与的需求，即确定脆弱组件是仅攻击者本身就可以随意利用，还是需要用户（或用户进程）以某种方式参与。
///
/// > This metric captures the requirement for a user, other than the attacker,
/// > to participate in the successful compromise of the vulnerable component.
/// > This metric determines whether the vulnerability can be exploited solely at the will of the
/// > attacker, or whether a separate user (or user-initiated process) must participate in some manner.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserInteractionType {
  Active,
  /// Require(R) 有需求
  ///
  /// 需要用户采取一些措施才能成功攻击此脆弱组件，例如说服用户单击电子邮件中的链接。
  Passive,
  /// None(N) 无需求
  ///
  /// 不需要任何用户的交互就可以成功攻击此脆弱组件。
  None,
}

impl Display for UserInteractionType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", Self::name(), self.as_str())
  }
}

impl UserInteractionType {
  #[allow(dead_code)]
  pub fn metric_help(&self) -> Help {
    self.help()
  }
}
impl Metric for UserInteractionType {
  const TYPE: MetricType = MetricType::V3(MetricTypeV3::UI);

  fn help(&self) -> Help {
    match self {
      UserInteractionType::Passive => {Help{ worth: Worth::Bad, des: "Successful exploitation of this vulnerability requires a user to take some action before the vulnerability can be exploited. For example, a successful exploit may only be possible during the installation of an application by a system administrator.".to_string() }}
      UserInteractionType::None => {Help{ worth: Worth::Worst, des: "The vulnerable system can be exploited without interaction from any human user, other than the attacker.".to_string() }}
      UserInteractionType::Active=>{Help{ worth: Worth::Worst, des: "Successful exploitation of this vulnerability requires a targeted user to perform specific, conscious interactions with the vulnerable system and the attacker’s payload, or the user’s interactions would actively subvert protection mechanisms which would lead to exploitation of the vulnerability.".to_string() }}
    }
  }

  fn score(&self) -> f32 {
    match self {
      UserInteractionType::Passive => 0.62,
      UserInteractionType::None => 0.85,
      UserInteractionType::Active => 0.0,
    }
  }

  fn as_str(&self) -> &'static str {
    match self {
      UserInteractionType::Passive => "P",
      UserInteractionType::None => "N",
      UserInteractionType::Active => "A",
    }
  }
}

impl FromStr for UserInteractionType {
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
      c.ok_or(CVSSError::InvalidCVSS { value: s })?
    };
    match c {
      'N' => Ok(Self::None),
      'P' => Ok(Self::Passive),
      'A' => Ok(Self::Active),
      _ => Err(CVSSError::InvalidCVSS {
        value: c.to_string(),
      }),
    }
  }
}
