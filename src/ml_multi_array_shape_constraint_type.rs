use crate::prelude::*;
// #import <Foundation/Foundation.h>

// API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
// typedef NS_ENUM(NSInteger, MLMultiArrayShapeConstraintType) {

//     MLMultiArrayShapeConstraintTypeUnspecified = 1, // An unconstrained shape. Any multi array satisfies this constraint.

//     MLMultiArrayShapeConstraintTypeEnumerated = 2, // Limited to an enumerated set of shapes

//     MLMultiArrayShapeConstraintTypeRange = 3,      // Allow full specified range per dimension

// };
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MLMultiArrayShapeConstraintType {
    Unspecified = 1, // An unconstrained shape. Any multi array satisfies this constraint.
    Enumerated = 2, // Limited to an enumerated set of shapes
    Range = 3,      // Allow full specified range per dimension
}