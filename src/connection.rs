use crate::configuration::Configuration;
use crate::error::Result;
use crate::known_host::KnownHost;
use crate::known_hosts::KnownHosts;
use crate::sftp::{list, SftpEntry, SftpReader, SftpWriter};
use ssh2::{HostKeyType, Session};
use std::net::TcpStream;

pub struct Connection {
  pub config: Configuration,
  pub session: Session,
  pub known_hosts: KnownHosts,
}

unsafe impl Send for Connection {}

impl Connection {
  pub fn new(config: &Configuration) -> Result<Self> {
    let session = Self::open_session(config)?;
    let known_hosts = KnownHosts::new(&session)?;
    Ok(Connection {
      config: config.clone(),
      session,
      known_hosts,
    })
  }

  fn open_session(config: &Configuration) -> Result<Session> {
    let tcp_stream = TcpStream::connect((config.hostname.as_str(), config.port))?;

    let mut session = Session::new()?;
    session.set_timeout(config.timeout);
    session.set_compress(config.compress);

    session.set_tcp_stream(tcp_stream);
    session.handshake()?;

    Ok(session)
  }

  pub fn add_known_host(&mut self, known_host: &KnownHost) -> Result<()> {
    self.known_hosts.add_known_host(known_host)
  }

  pub fn get_remote_host_key(&self) -> Option<(String, HostKeyType)> {
    self
      .session
      .host_key()
      .map(|(key, key_type)| (base64::encode(key), key_type))
  }

  pub fn start(&mut self) -> Result<()> {
    self.known_hosts.check_remote(
      &self.session,
      &self.config.hostname,
      self.config.port,
      self.config.trust_host,
    )?;
    self.authenticate()
  }

  fn authenticate(&self) -> Result<()> {
    self
      .config
      .authentication
      .authenticate(&self.session, &self.config.username)
  }

  pub fn read_over_sftp(&self, path: &str) -> Result<SftpReader> {
    SftpReader::new(&self.session, path)
  }

  pub fn write_over_sftp(&self, path: &str) -> Result<SftpWriter> {
    SftpWriter::new(&self.session, path)
  }

  pub fn list_over_sftp(&self, path: &str) -> Result<Vec<SftpEntry>> {
    list(&self.session, path)
  }
}
