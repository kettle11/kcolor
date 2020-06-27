use crate::*;
use std::fs;

#[test]
fn basic_run() {
    let contents = fs::read("examples/sRGB Profile.icc").expect("Could not find file");
    let mut parser = ICCParser::new(&contents).unwrap();
    let header = parser.header().unwrap();

    while let Ok(tag) = parser.next_tag() {
        println!("Tag: {:?}", tag);
    }

    println!("Profile: {:?}", header);
}
