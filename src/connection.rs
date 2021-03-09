use crate::configuration::Configuration;
use crate::error::Error;
use crate::known_hosts::KnownHosts;
use crate::sftp::{SftpReader, SftpWriter};
use ssh2::Session;
use std::net::TcpStream;

pub struct Connection {
  pub config: Configuration,
  pub session: Session,
}

impl Connection {
  pub fn new(config: &Configuration) -> Result<Self, Error> {
    let session = Self::open_session(config)?;
    Ok(Connection {
      config: config.clone(),
      session,
    })
  }

  fn open_session(config: &Configuration) -> Result<Session, Error> {
    let tcp_stream = TcpStream::connect((config.hostname.as_str(), config.port))?;

    let mut session = Session::new()?;
    session.set_timeout(config.timeout);
    session.set_compress(config.compress);

    session.set_tcp_stream(tcp_stream);
    session.handshake()?;

    Ok(session)
  }

  pub fn start(&self) -> Result<(), Error> {
    let mut hosts = KnownHosts::new(&self.session)?;
    hosts.check_remote(&self.session, &self.config.hostname, self.config.port)?;
    self.authenticate()
  }

  fn authenticate(&self) -> Result<(), Error> {
    self
      .config
      .authentication
      .authenticate(&self.session, &self.config.username)
  }

  pub fn read_over_sftp(&self, path: &str) -> Result<SftpReader, Error> {
    SftpReader::new(&self.session, path)
  }

  pub fn write_over_sftp(&self, path: &str) -> Result<SftpWriter, Error> {
    SftpWriter::new(&self.session, path)
  }
}
