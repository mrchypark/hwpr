use extendr_api::prelude::*;
use hwp::hwp::version::Version;

#[derive(Clone)]
pub struct RVersion(pub Version);

#[extendr]
impl RVersion {
  fn majer(&self) -> u8 {
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
  fn to_string(&self) -> String {
    self.0.to_string()
  }
}

extendr_module! {
  mod version;
  impl RVersion;
}