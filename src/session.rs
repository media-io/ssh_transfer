use ssh2::Session;
use std::net::TcpStream;

use crate::authentication::SshAuthenticationType;
use crate::config::SshConfiguration;
use crate::known_hosts::SshKnownHosts;
use crate::sftp::{SftpReader, SftpWriter};

pub struct SshSession {
  pub config: SshConfiguration,
  pub session: Session,
}

impl SshSession {
  pub fn new(config: &SshConfiguration) -> Result<Self, String> {
    let session = Self::open_session(config)?;
    Ok(SshSession {
      config: config.clone(),
      session,
    })
  }

  fn open_session(config: &SshConfiguration) -> Result<Session, String> {
    let tcp_stream =
      TcpStream::connect((config.hostname.as_str(), config.port)).map_err(|e| e.to_string())?;

    let mut session = Session::new().map_err(|e| e.to_string())?;
    session.set_timeout(config.timeout);
    session.set_compress(config.compress);

    session.set_tcp_stream(tcp_stream);
    session.handshake().map_err(|e| e.to_string())?;

    Ok(session)
  }

  pub fn connect(&self) -> Result<(), String> {
    let mut known_hosts = SshKnownHosts::new(&self.session)?;
    known_hosts.check_remote(&self.session, &self.config.hostname, self.config.port)?;
    self.authenticate()
  }

  fn authenticate(&self) -> Result<(), String> {
    match &self.config.authentication {
      SshAuthenticationType::Anonymous => {
        unimplemented!()
      }
      SshAuthenticationType::Password(login) => login.authenticate(&self.session),
      SshAuthenticationType::KeyFile(_key_file_path) => {
        unimplemented!()
      }
    }
  }

  pub fn read_over_sftp(&self, path: &str) -> Result<SftpReader, String> {
    SftpReader::new(&self.session, path)
  }

  pub fn write_over_sftp(&self, path: &str) -> Result<SftpWriter, String> {
    SftpWriter::new(&self.session, path)
  }
}
