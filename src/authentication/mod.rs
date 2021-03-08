mod password;

pub use password::SshAuthenticationPassword;

#[derive(Clone, Debug)]
pub enum SshAuthenticationType {
  Anonymous,
  Password(SshAuthenticationPassword),
  KeyFile(String),
}
