mod password;

pub use password::AuthenticationPassword;

#[derive(Clone, Debug)]
pub enum AuthenticationType {
  Anonymous,
  Password(AuthenticationPassword),
  KeyFile(String),
}
