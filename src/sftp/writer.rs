use crate::error::{Error::SftpError, Result};
use ssh2::{File, OpenFlags, OpenType, Session};
use std::{io::Write, path::Path};

pub struct SftpWriter {
  path: String,
  file: File,
}

impl SftpWriter {
  pub fn new(session: &Session, path: &str) -> Result<Self> {
    let file = session.sftp()?.open_mode(
      Path::new(path),
      OpenFlags::WRITE | OpenFlags::TRUNCATE,
      0o644,
      OpenType::File,
    )?;
    Ok(SftpWriter {
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

impl Write for SftpWriter {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    self.file.write(buf)
  }

  fn flush(&mut self) -> std::io::Result<()> {
    self.file.flush()
  }
}
