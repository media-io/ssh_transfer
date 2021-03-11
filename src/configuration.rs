use crate::authentication::AuthenticationType;

#[derive(Clone, Debug)]
pub struct Configuration {
  pub hostname: String,
  pub port: u16,
  pub username: String,
  pub authentication: AuthenticationType,
  pub timeout: u32,
  pub compress: bool,
  pub trust_host: bool,
}

impl Configuration {
  pub fn new(hostname: &str) -> Self {
    Configuration {
      hostname: hostname.to_string(),
      // set default port to 22
      port: 22,
      // get current username
      username: whoami::username(),
      // ask for password
      authentication: AuthenticationType::Interactive,
      // set default timeout to 10 seconds
      timeout: 10000,
      // attempt to negotiate compression
      compress: true,
      // do not accept connection to unknown host
      trust_host: false,
    }
  }

  pub fn with_port(mut self, port: u16) -> Self {
    self.port = port;
    self
  }

  pub fn with_username(mut self, username: &str) -> Self {
    self.username = username.to_string();
    self
  }

  pub fn with_authentication(mut self, authentication: AuthenticationType) -> Self {
    self.authentication = authentication;
    self
  }

  pub fn with_timeout_ms(mut self, timeout: u32) -> Self {
    self.timeout = timeout;
    self
  }

  pub fn with_compression(mut self, compress: bool) -> Self {
    self.compress = compress;
    self
  }

  pub fn with_host_trust(mut self, trust_host: bool) -> Self {
    self.trust_host = trust_host;
    self
  }
}

#[test]
pub fn test_configuration() {
  let configuration = Configuration::new("localhost");

  assert_eq!("localhost", configuration.hostname);
  assert_eq!(22, configuration.port);
  assert_eq!(whoami::username(), configuration.username);
  assert_eq!(
    AuthenticationType::Interactive,
    configuration.authentication
  );
  assert_eq!(10000, configuration.timeout);
  assert_eq!(true, configuration.compress);
  assert_eq!(false, configuration.trust_host);

  let configuration = configuration
    .with_port(12345)
    .with_username("user_name")
    .with_authentication(AuthenticationType::Password("user_password".to_string()))
    .with_timeout_ms(54321)
    .with_compression(false)
    .with_host_trust(true);
  assert_eq!("localhost", configuration.hostname);
  assert_eq!(12345, configuration.port);
  assert_eq!("user_name", configuration.username);
  assert_eq!(
    AuthenticationType::Password("user_password".to_string()),
    configuration.authentication
  );
  assert_eq!(54321, configuration.timeout);
  assert_eq!(false, configuration.compress);
  assert_eq!(true, configuration.trust_host);
}
