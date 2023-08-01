use extendr_api::prelude::*;
use hwp::hwp::bin_data::File;

#[derive(Clone)]
pub struct RFile {
    pub name: String,
    pub data: Robj,
}

#[extendr]
impl RFile {
    pub fn from_rust(file: &File) -> Self {
        let data: Robj = r!([1.0]);

        Self {
            name: file.name.clone(),
            data,
        }
    }
}