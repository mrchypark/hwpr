mod rversion;

use extendr_api::prelude::*;
use rversion::RVersion;
use hwp::HWP;

use std::fs;


#[derive(Clone)]
struct HWPReader {
    pub version: RVersion,
    pub sections: Vec<i32>,
    pub bin_data: Vec<i32>,
}

#[extendr]
impl HWPReader {
  fn new(path: String) -> extendr_api::Result<Self> {
      // TODO: (@hahnlee) 메모리에 있는 파일 읽기 등 더 좋은 방법 필요
      let file = fs::read(path).unwrap();

      let hwp = HWP::from_bytes(&file);

      let _body = if hwp.header.flags.distributed {
          hwp.view_texts.as_ref().unwrap()
      } else {
          &hwp.body_texts
      };


      Ok(Self {
          version: RVersion(hwp.header.version.clone()),
          sections: vec![1],
          bin_data: vec![1],
      })
  }
  fn find_all(&self) -> String {
    self.version.to_string()
  }
  fn find_all2(&self) -> Vec<i32> {
    self.sections.clone()
  }
  fn find_all3(&self) -> Vec<i32> {
    self.bin_data.clone()
  }
}

extendr_module! {
    mod hwpr;
    impl HWPReader;
}