use dirs::home_dir;
use log::{debug, info, warn};
use ssh2::{CheckResult, HashType, KnownHostFileKind, KnownHosts, Session};
use std::path::PathBuf;

pub struct SshKnownHosts {
  known_hosts: KnownHosts,
  known_hosts_file_path: PathBuf,
}

impl SshKnownHosts {
  pub fn new(session: &Session) -> Result<Self, String> {
    let mut known_hosts = session.known_hosts().map_err(|e| e.to_string())?;

    let known_hosts_file_path = home_dir()
      .ok_or_else(|| "Unable to find home directory".to_string())?
      .join(".ssh/known_hosts");

    known_hosts
      .read_file(&known_hosts_file_path, KnownHostFileKind::OpenSSH)
      .map_err(|e| e.to_string())?;

    Ok(SshKnownHosts {
      known_hosts,
      known_hosts_file_path,
    })
  }

  pub fn check_remote(
    &mut self,
    session: &Session,
    hostname: &str,
    port: u16,
  ) -> Result<(), String> {
    let (host_key, host_key_type) = session
      .host_key()
      .ok_or_else(|| "Host key not found".to_string())?;

    match self.known_hosts.check_port(hostname, port, host_key) {
      CheckResult::Match => {
        debug!(
          "Host key for {}:{} matches entry in {:?}.",
          hostname, port, self.known_hosts_file_path
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
          .ok_or_else(|| "Host hash not found".to_string())?;

        info!(
          "No matching host key for {}:{} was not found in {:?}.",
          hostname, port, self.known_hosts_file_path
        );

        // TODO Ask before adding fingerprint to known hosts?
        warn!("Add fingerprint to known hosts: {}", host_fingerprint);
        self
          .known_hosts
          .add(
            hostname,
            host_key,
            "Added by ssh_transfer",
            host_key_type.into(),
          )
          .map_err(|e| e.to_string())?;

        self
          .known_hosts
          .write_file(&self.known_hosts_file_path, KnownHostFileKind::OpenSSH)
          .map_err(|e| e.to_string())?;
        Ok(())
      }
      CheckResult::Mismatch => {
        warn!("####################################################");
        warn!("# WARNING: REMOTE HOST IDENTIFICATION HAS CHANGED! #");
        warn!("####################################################");
        Err(format!("Fingerprint for '{}' host mismatched!", hostname))
      }
      CheckResult::Failure => Err(format!("Host file check failed for '{}'!", hostname)),
    }
  }
}
