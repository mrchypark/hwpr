use extendr_api::prelude::*;
use hwp::hwp::version::Version;

#[derive(Clone)]
pub struct RVersion(pub Version);

#[extendr]
impl RVersion {
  fn from_str(version: &str) -> Self {
    RVersion(Version::from_str(version)).clone()
  }
  fn major(&self) -> u8 {
    self.0.major
  }
  fn minor(&self) -> u8 {
    self.0.minor
  }
  fn micro(&self) -> u8 {
    self.0.micro
  }
  fn build_number(&self) -> u8 {
    self.0.build_number
  }
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

extendr_module! {
  mod version;
  impl RVersion;
}
