use crate::error::{Error::KnownHostCheckError, Result};
use crate::known_host::KnownHost;
use dirs::home_dir;
use log::{debug, warn};
use ssh2::{CheckResult, HashType, KnownHostFileKind, Session};

use std::path::PathBuf;

pub struct KnownHosts {
  known_hosts: ssh2::KnownHosts,
  known_hosts_file_path: Option<PathBuf>,
}

impl KnownHosts {
  pub fn new(session: &Session) -> Result<Self> {
    let mut known_hosts = session.known_hosts()?;

    let known_hosts_file_path = std::env::var("SSH_KNOWN_HOSTS_PATH")
      .map(PathBuf::from)
      .unwrap_or(
        home_dir()
          .ok_or_else(|| KnownHostCheckError("Unable to find home directory".to_string()))?
          .join(".ssh/known_hosts"),
      );

    let known_hosts_file_path = if known_hosts_file_path.as_path().exists() {
      debug!("Read known hosts from {:?}", known_hosts_file_path);
      known_hosts.read_file(&known_hosts_file_path, KnownHostFileKind::OpenSSH)?;
      Some(known_hosts_file_path)
    } else {
      None
    };

    Ok(KnownHosts {
      known_hosts,
      known_hosts_file_path,
    })
  }

  pub fn add_known_host(&mut self, known_host: &KnownHost) -> Result<()> {
    Ok(
      self.known_hosts.add(
        &known_host.hostname,
        &known_host.fingerprint,
        &known_host
          .comment
          .clone()
          .unwrap_or_else(|| "Added by ssh-transfer".to_string()),
        known_host.key_format,
      )?,
    )
  }

  pub fn check_remote(
    &mut self,
    session: &Session,
    hostname: &str,
    port: u16,
    trust_host: bool,
  ) -> Result<()> {
    let (host_key, host_key_type) = session
      .host_key()
      .ok_or_else(|| KnownHostCheckError("Host key not found.".to_string()))?;

    match self.known_hosts.check_port(hostname, port, host_key) {
      CheckResult::Match => {
        debug!(
          "Host key for {}:{} matches entry in known hosts.",
          hostname, port
        );
        Ok(())
      }
      CheckResult::NotFound => {
        let host_fingerprint = session
          .host_key_hash(HashType::Sha256)
          .map(|hash| ("SHA256", hash))
          .or_else(|| {
            session
              .host_key_hash(HashType::Sha1)
              .map(|hash| ("SHA128", hash))
          })
          .map(|(hash_type, fingerprint)| format!("{}:{}", hash_type, base64::encode(fingerprint)))
          .ok_or_else(|| KnownHostCheckError("Host hash not found.".to_string()))?;

        if !trust_host {
          return Err(KnownHostCheckError(format!(
            "No matching host key found for {}:{} in known hosts.",
            hostname, port
          )));
        }

        warn!("No matching host key found for {}:{} in known hosts, but trust host anyway. Fingerprint: {}", hostname, port, host_fingerprint);
        self.known_hosts.add(
          hostname,
          host_key,
          "Added by ssh-transfer",
          host_key_type.into(),
        )?;

        Ok(())
      }
      CheckResult::Mismatch => Err(KnownHostCheckError(format!(
        "Remote host fingerprint for {}:{} host has changed.",
        hostname, port,
      ))),
      CheckResult::Failure => Err(KnownHostCheckError(format!(
        "Host file check failed for '{}'.",
        hostname
      ))),
    }
  }

  pub fn write_to_file(&self) -> Result<()> {
    if let Some(known_hosts_file_path) = &self.known_hosts_file_path {
      self
        .known_hosts
        .write_file(&known_hosts_file_path, KnownHostFileKind::OpenSSH)?;
    }
    Ok(())
  }
}
