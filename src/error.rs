use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
  IoError(std::io::Error),
  SshError(ssh2::Error),

  AuthenticationError(String),
  KnownHostCheckError(String),
  SftpError(String),
}

impl std::error::Error for Error {}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Error::IoError(error) => write!(f, "IO error: {}", error),
      Error::SshError(error) => write!(f, "SSH error: {}", error),
      Error::AuthenticationError(error) => write!(f, "Authentication failure: {}", error),
      Error::KnownHostCheckError(error) => write!(f, "Known host check failure: {}", error),
      Error::SftpError(error) => write!(f, "SFTP error: {}", error),
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Error::IoError(error)
  }
}

impl From<ssh2::Error> for Error {
  fn from(error: ssh2::Error) -> Self {
    Error::SshError(error)
  }
}

impl Into<std::io::Error> for Error {
  fn into(self) -> std::io::Error {
    use std::io::ErrorKind;
    match self {
      Error::IoError(error) => error,
      Error::SshError(error) => std::io::Error::new(ErrorKind::Other, format!("SSH: {}", error)),
      Error::AuthenticationError(error) => {
        std::io::Error::new(ErrorKind::Other, format!("Auth: {}", error))
      }
      Error::KnownHostCheckError(error) => {
        std::io::Error::new(ErrorKind::Other, format!("Host: {}", error))
      }
      Error::SftpError(error) => std::io::Error::new(ErrorKind::Other, format!("SFTP: {}", error)),
    }
  }
}
