use crate::prelude::*;

// /*!
//  * Supported data type enumeration
//  */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MLFeatureType {
    //
    Invalid = 0,

    /// Discrete values, sometimes used to hold numeric encoding of a categorical value
    Int64 = 1,

    /// Continuous values
    Double = 2,

    // Text or categorical strings
    String = 3,

    /// CVPixelBufferRef
    Image = 4,

    /// MLMultiArray
    MultiArray = 5,

    /// Numerically weighted hashable objects (e.g. word counts)
    Dictionary = 6,

    /// MLSequence. Ordered collection of feature values with the same type
    // API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
    Sequence = 7,
}
