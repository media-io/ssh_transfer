use ssh2::Session;

#[derive(Clone, Debug)]
pub struct SshConfiguration {
  pub hostname: String,
  pub port: u16,
  pub authentication: SshAuthenticationType,
  pub timeout: u32,
  pub compress: bool,
}

impl SshConfiguration {
  pub fn new(
    hostname: &str,
    port: Option<u16>,
    authentication: SshAuthenticationType,
    timeout: Option<u32>,
    compress: Option<bool>,
  ) -> Self {
    SshConfiguration {
      hostname: hostname.to_string(),
      // set default port to 22
      port: port.unwrap_or_else(|| 22),
      authentication,
      // set default timeout to 10 seconds
      timeout: timeout.unwrap_or_else(|| 10000),
      // attempt to negotiate compression
      compress: compress.unwrap_or_else(|| true),
    }
  }
}

#[derive(Clone, Debug)]
pub enum SshAuthenticationType {
  Anonymous,
  Password(SshPasswordAuthentication),
  KeyFile(String),
}

#[derive(Clone, Debug)]
pub struct SshPasswordAuthentication {
  pub username: String,
  pub password: String,
}

impl SshPasswordAuthentication {
  pub fn new(username: &str, password: &str) -> Self {
    SshPasswordAuthentication {
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
