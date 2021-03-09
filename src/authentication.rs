use crate::error::{Error, Error::AuthenticationError};
use ssh2::Session;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub enum AuthenticationType {
  Interactive,
  KeyFile(PathBuf),
  Password(String),
}

impl AuthenticationType {
  pub(crate) fn authenticate(&self, session: &Session, username: &str) -> Result<(), Error> {
    if session.authenticated() {
      return Ok(());
    }

    let authentication_methods: Vec<String> = session
      .auth_methods(username)?
      .split(',')
      .map(String::from)
      .collect();

    match &self {
      AuthenticationType::Interactive => {
        unimplemented!()
      }
      AuthenticationType::KeyFile(_key_file_path) => {
        unimplemented!()
      }
      AuthenticationType::Password(password) => {
        if authentication_methods.contains(&"password".to_string()) {
          session.userauth_password(username, password)?;
        }
      }
    }

    if !session.authenticated() {
      return Err(AuthenticationError(format!(
        "Could not authenticate user: {}.",
        username
      )));
    }

    Ok(())
  }
}
