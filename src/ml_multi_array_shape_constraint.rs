use crate::prelude::*;
// #import <Foundation/Foundation.h>
// #import <CoreML/MLMultiArrayShapeConstraintType.h>
// #import <CoreML/MLExport.h>

// NS_ASSUME_NONNULL_BEGIN

// API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
// ML_EXPORT
// @interface MLMultiArrayShapeConstraint : NSObject <NSSecureCoding>

pub enum MLMultiArrayShapeConstraintFFI {}

foreign_obj_type! {
    type CType = MLMultiArrayShapeConstraintFFI;
    pub struct MLMultiArrayShapeConstraint;
    pub struct MLMultiArrayShapeConstraintRef;
}

// @property (readonly, nonatomic) MLMultiArrayShapeConstraintType type;

// // Size of each dimension i must fall within sizeRangeForDimension[i].rangeValue
// @property (readonly, nonatomic) NSArray<NSValue *> * sizeRangeForDimension;

// // If type == MLMultiArrayShapeConstraintTypeEnumerated then
// // only shapes in this set are allowed
// @property (readonly, nonatomic) NSArray<NSArray<NSNumber *> *> * enumeratedShapes;

// @end

// NS_ASSUME_NONNULL_END
