use crate::error::{Error, Result};
use regex::Regex;
use ssh2::KnownHostKeyFormat;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
pub struct KnownHost {
  pub hostname: String,
  pub key_format: KnownHostKeyFormat,
  pub fingerprint: Vec<u8>,
  pub comment: Option<String>,
}

impl TryFrom<&str> for KnownHost {
  type Error = crate::error::Error;
  fn try_from(value: &str) -> Result<Self> {
    let re = Regex::new(r"(?P<host>\S*) (?P<type>\S*) (?P<hash>\S*) ?(?P<comment>.*)?")?;
    let captures = re
      .captures(value)
      .ok_or_else(|| Error::KnownHostParsingError("invalid known host format.".to_string()))?;

    let hostname = captures
      .name("host")
      .ok_or_else(|| Error::KnownHostParsingError("cannot get hostname.".to_string()))?
      .as_str()
      .to_string();
    let key_format = captures
      .name("type")
      .ok_or_else(|| Error::KnownHostParsingError("cannot get key format.".to_string()))?
      .as_str();
    let fingerprint = captures
      .name("hash")
      .ok_or_else(|| Error::KnownHostParsingError("cannot get fingerprint.".to_string()))?
      .as_str();
    let fingerprint = base64::decode(fingerprint)?;
    let comment = captures
      .name("comment")
      .map(|c| c.as_str().to_string())
      .filter(|c| !c.is_empty());

    let key_format = match key_format {
      "rsa1" => KnownHostKeyFormat::Rsa1,
      "ssh-rsa" => KnownHostKeyFormat::SshRsa,
      "ssh-dss" => KnownHostKeyFormat::SshDss,
      "ecdsa-sha2-nistp256" => KnownHostKeyFormat::Ecdsa256,
      "ecdsa-sha2-nistp384" => KnownHostKeyFormat::Ecdsa384,
      "ecdsa-sha2-nistp521" => KnownHostKeyFormat::Ecdsa521,
      "ssh-ed25519" => KnownHostKeyFormat::Ed255219,
      _ => KnownHostKeyFormat::Unknown,
    };

    Ok(KnownHost {
      hostname,
      key_format,
      fingerprint,
      comment,
    })
  }
}

#[test]
pub fn test_known_host() {
  let known_host = KnownHost::try_from("localhost ssh-ed25519 SGVsbG8gV29ybGQh").unwrap();
  assert_eq!("localhost", &known_host.hostname);
  assert_eq!(
    format!("{:?}", KnownHostKeyFormat::Ed255219),
    format!("{:?}", known_host.key_format)
  );
  assert_eq!("Hello World!".as_bytes().to_vec(), known_host.fingerprint);
  assert_eq!(None, known_host.comment);
}
