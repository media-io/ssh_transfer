use base64::DecodeError;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
  Base64DecodingError(base64::DecodeError),
  IoError(std::io::Error),
  RegexError(regex::Error),
  SshError(ssh2::Error),

  AuthenticationError(String),
  KnownHostCheckError(String),
  KnownHostParsingError(String),
  SftpError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Error::Base64DecodingError(error) => write!(f, "Base64 error: {}", error),
      Error::IoError(error) => write!(f, "IO error: {}", error),
      Error::RegexError(error) => write!(f, "Regex error: {}", error),
      Error::SshError(error) => write!(f, "SSH error: {}", error),
      Error::AuthenticationError(error) => write!(f, "Authentication failure: {}", error),
      Error::KnownHostCheckError(error) => write!(f, "Known host check failure: {}", error),
      Error::KnownHostParsingError(error) => write!(f, "Known host parsing error: {}", error),
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

impl From<base64::DecodeError> for Error {
  fn from(error: DecodeError) -> Self {
    Error::Base64DecodingError(error)
  }
}

impl From<regex::Error> for Error {
  fn from(error: regex::Error) -> Self {
    Error::RegexError(error)
  }
}

impl Into<std::io::Error> for Error {
  fn into(self) -> std::io::Error {
    use std::io::ErrorKind;
    match self {
      Error::Base64DecodingError(error) => {
        std::io::Error::new(ErrorKind::Other, format!("Base64: {}", error))
      }
      Error::IoError(error) => error,
      Error::RegexError(error) => {
        std::io::Error::new(ErrorKind::Other, format!("Regex: {}", error))
      }
      Error::SshError(error) => std::io::Error::new(ErrorKind::Other, format!("SSH: {}", error)),
      Error::AuthenticationError(error) => {
        std::io::Error::new(ErrorKind::Other, format!("Auth: {}", error))
      }
      Error::KnownHostCheckError(error) | Error::KnownHostParsingError(error) => {
        std::io::Error::new(ErrorKind::Other, format!("Host: {}", error))
      }
      Error::SftpError(error) => std::io::Error::new(ErrorKind::Other, format!("SFTP: {}", error)),
    }
  }
}
