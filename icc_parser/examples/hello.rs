use icc_parser::*;
use std::fs;

pub fn main() {
    let contents = fs::read("examples/sRGB Profile.icc").expect("Could not find file");
    icc_parser::parse_bytes(&contents).unwrap();
}
