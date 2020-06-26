use crate::*;
use std::fs;

#[test]
fn basic_run() {
    let contents = fs::read("examples/sRGB Profile.icc").expect("Could not find file");
    let profile = parse_bytes(&contents).unwrap();

    println!("Profile: {:?}", profile);
}
