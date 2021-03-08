use ssh2::Session;

#[derive(Clone, Debug)]
pub struct SshAuthenticationPassword {
  pub username: String,
  pub password: String,
}

impl SshAuthenticationPassword {
  pub fn new(username: &str, password: &str) -> Self {
    SshAuthenticationPassword {
      username: username.to_string(),
      password: password.to_string(),
    }
  }

  pub fn authenticate(&self, session: &Session) -> Result<(), String> {
    if session.authenticated() {
      return Ok(());
    }

    if session
      .auth_methods(&self.username)
      .map_err(|e| e.to_string())?
      .contains("password")
    {
      session
        .userauth_password(&self.username, &self.password)
        .map_err(|e| e.to_string())?;
    }

    if !session.authenticated() {
      return Err(format!("Authentication failed for user: {}", self.username));
    }

    Ok(())
  }
}
