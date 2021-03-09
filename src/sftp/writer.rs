use ssh2::{File, OpenFlags, OpenType, Session};
use std::{io::Write, path::Path};

pub struct SftpWriter {
  path: String,
  file: File,
}

impl SftpWriter {
  pub fn new(session: &Session, path: &str) -> Result<Self, String> {
    let file = session
      .sftp()
      .map_err(|e| e.message().to_string())?
      .open_mode(
        Path::new(path),
        OpenFlags::WRITE | OpenFlags::TRUNCATE,
        0o644,
        OpenType::File,
      )
      .map_err(|e| e.message().to_string())?;
    Ok(SftpWriter {
      path: path.to_string(),
      file,
    })
  }

  pub fn get_size(&mut self) -> Result<u64, String> {
    self
      .file
      .stat()
      .map_err(|e| e.message().to_string())?
      .size
      .ok_or_else(|| format!("Cannot retrieve size for path: {}", self.path))
  }
}

impl Write for SftpWriter {
  fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
    self.file.write(buf)
  }

  fn flush(&mut self) -> Result<(), std::io::Error> {
    self.file.flush()
  }
}
