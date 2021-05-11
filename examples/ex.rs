use coreml::prelude::*;

fn a(t: &MLMultiArrayRef) {}

fn main() {
    let n = NSNumber::new(64);
    println!("{:?}", n);
    println!("here");
}
