use crate::error::Result;
use ssh2::{FileStat, FileType, Session};
use std::path::{Path, PathBuf};

pub fn list(session: &Session, path: &str) -> Result<Vec<SftpEntry>> {
  let files = session
    .sftp()?
    .readdir(Path::new(path))?
    .iter()
    .map(SftpEntry::from)
    .collect();

  Ok(files)
}

pub struct SftpEntry {
  path: String,
  size: Option<u64>,
  user_id: Option<u32>,
  group_id: Option<u32>,
  permissions: Option<u32>,
  last_access_time: Option<u64>,
  last_modification_time: Option<u64>,
  kind: SftpEntryKind,
}

impl SftpEntry {
  pub fn path(&self) -> &str {
    &self.path
  }
  pub fn size(&self) -> &Option<u64> {
    &self.size
  }
  pub fn user_id(&self) -> &Option<u32> {
    &self.user_id
  }
  pub fn group_id(&self) -> &Option<u32> {
    &self.group_id
  }
  pub fn permissions(&self) -> &Option<u32> {
    &self.permissions
  }
  pub fn last_access_time(&self) -> &Option<u64> {
    &self.last_access_time
  }
  pub fn last_modification_time(&self) -> &Option<u64> {
    &self.last_modification_time
  }
  pub fn kind(&self) -> &SftpEntryKind {
    &self.kind
  }
}

impl From<&(PathBuf, FileStat)> for SftpEntry {
  fn from((path, file_stat): &(PathBuf, FileStat)) -> Self {
    Self {
      path: path.to_str().unwrap_or_default().to_string(),
      size: file_stat.size,
      user_id: file_stat.uid,
      group_id: file_stat.gid,
      permissions: file_stat.perm,
      last_access_time: file_stat.atime,
      last_modification_time: file_stat.mtime,
      kind: SftpEntryKind::from(file_stat.file_type()),
    }
  }
}

pub enum SftpEntryKind {
  File,
  Directory,
  SymLink,
  Other,
}

impl From<FileType> for SftpEntryKind {
  fn from(file_type: FileType) -> Self {
    match file_type {
      FileType::Directory => SftpEntryKind::Directory,
      FileType::RegularFile => SftpEntryKind::File,
      FileType::Symlink => SftpEntryKind::SymLink,
      _ => SftpEntryKind::Other,
    }
  }
}
