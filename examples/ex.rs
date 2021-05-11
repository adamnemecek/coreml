use coreml::prelude::*;

fn a(t: &MLMultiArrayRef) {}

fn main() {
    let n = NSNumber::new(64);

    let t = MLMultiArray::new_with_shape(&[1, 2, 3], MLMultiArrayDataType::Float32).unwrap();
    println!("{:?}", t.count());
    println!("{:?}", n);
    println!("here");
}
