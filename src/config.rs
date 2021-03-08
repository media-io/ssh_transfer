use crate::authentication::SshAuthenticationType;

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
      port: port.unwrap_or(22),
      authentication,
      // set default timeout to 10 seconds
      timeout: timeout.unwrap_or(10000),
      // attempt to negotiate compression
      compress: compress.unwrap_or(true),
    }
  }
}
