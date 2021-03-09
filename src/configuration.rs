use crate::authentication::AuthenticationType;

#[derive(Clone, Debug)]
pub struct Configuration {
  pub hostname: String,
  pub port: u16,
  pub authentication: AuthenticationType,
  pub timeout: u32,
  pub compress: bool,
}

impl Configuration {
  pub fn new(
    hostname: &str,
    port: Option<u16>,
    authentication: AuthenticationType,
    timeout: Option<u32>,
    compress: Option<bool>,
  ) -> Self {
    Configuration {
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
