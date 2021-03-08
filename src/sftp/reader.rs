use std::io::Read;
use std::path::Path;

use ssh2::{File, Session};

pub struct SftpReader {
  path: String,
  file: File,
}

impl SftpReader {
  pub fn new(session: &Session, path: &str) -> Result<Self, String> {
    let file = session
      .sftp()
      .map_err(|e| e.message().to_string())?
      .open(Path::new(path))
      .map_err(|e| e.message().to_string())?;
    Ok(SftpReader {
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

impl Read for SftpReader {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
    self.file.read(buf)
  }
}
