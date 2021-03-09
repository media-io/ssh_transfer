use ssh2::Session;
use std::net::TcpStream;

use crate::authentication::AuthenticationType;
use crate::configuration::Configuration;
use crate::known_hosts::KnownHosts;
use crate::sftp::{SftpReader, SftpWriter};

pub struct Connection {
  pub config: Configuration,
  pub session: Session,
}

impl Connection {
  pub fn new(config: &Configuration) -> Result<Self, String> {
    let session = Self::open_session(config)?;
    Ok(Connection {
      config: config.clone(),
      session,
    })
  }

  fn open_session(config: &Configuration) -> Result<Session, String> {
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
    let mut hosts = KnownHosts::new(&self.session)?;
    hosts.check_remote(&self.session, &self.config.hostname, self.config.port)?;
    self.authenticate()
  }

  fn authenticate(&self) -> Result<(), String> {
    match &self.config.authentication {
      AuthenticationType::Anonymous => {
        unimplemented!()
      }
      AuthenticationType::Password(login) => login.authenticate(&self.session),
      AuthenticationType::KeyFile(_key_file_path) => {
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
