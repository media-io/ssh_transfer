use crate::error::{Error, Error::SftpError};
use ssh2::{File, Session};
use std::{io::Read, path::Path};

pub struct SftpReader {
  path: String,
  file: File,
}

impl SftpReader {
  pub fn new(session: &Session, path: &str) -> Result<Self, Error> {
    let file = session.sftp()?.open(Path::new(path))?;
    Ok(SftpReader {
      path: path.to_string(),
      file,
    })
  }

  pub fn get_size(&mut self) -> Result<u64, Error> {
    self
      .file
      .stat()?
      .size
      .ok_or_else(|| SftpError(format!("Cannot retrieve size for path: {}", self.path)))
  }
}

impl Read for SftpReader {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
    self.file.read(buf)
  }
}
