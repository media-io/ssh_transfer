use crate::error::{Error::SftpError, Result};
use ssh2::{File, Session};
use std::{io::Read, path::Path};

pub struct SftpReader {
  path: String,
  file: File,
}

impl SftpReader {
  pub fn new(session: &Session, path: &str) -> Result<Self> {
    let file = session.sftp()?.open(Path::new(path))?;
    Ok(SftpReader {
      path: path.to_string(),
      file,
    })
  }

  pub fn get_size(&mut self) -> Result<u64> {
    self
      .file
      .stat()?
      .size
      .ok_or_else(|| SftpError(format!("Cannot retrieve size for path: {}", self.path)))
  }
}

impl Read for SftpReader {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    self.file.read(buf)
  }
}
