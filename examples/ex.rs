use coreml::prelude::*;

fn a(t: &MLMultiArrayRef) {}

fn main() {
    let n = NSNumber::new(64);

    let s = MLSequence::empty_with_type(MLFeatureType::Sequence);
    println!("{:?}", s.type_());

    let t = MLMultiArray::new_with_shape(&[1, 2, 3], MLMultiArrayDataType::Float32).unwrap();
    println!("{:?}", t.count());
    println!("{:?}", n);
    println!("here");

    let v = MLFeatureValue::with_f64(60.0);
    println!("{:?}", v.double_value());
}
