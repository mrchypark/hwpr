use hwp::hwp::paragraph::Paragraph;
use extendr_api::prelude::*;

#[derive(Clone)]
pub struct RParagraph {
    paragraph: Paragraph,
}

#[extendr]
impl RParagraph {
  pub fn from_rust(paragraph: &Paragraph) -> Self {
      Self {
          paragraph: paragraph.clone(),
      }
  }
}


extendr_module! {
  mod paragraph;
  impl RParagraph;
}