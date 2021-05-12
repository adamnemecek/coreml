use crate::prelude::*;
// #import <Foundation/Foundation.h>
// #import <CoreML/MLExport.h>

// NS_ASSUME_NONNULL_BEGIN

// @class MLFeatureDescription;

// /*!
//  * Constraint describing expected MLSequence properties
//  */
// API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
// ML_EXPORT
// @interface MLSequenceConstraint : NSObject <NSSecureCoding>

pub enum MLSequenceConstraintFFI {}

foreign_obj_type! {
    type CType = MLSequenceConstraintFFI;
    pub struct MLSequenceConstraint;
    pub struct MLSequenceConstraintRef;
}

impl MLSequenceConstraintRef {
    // // Description all sequence elements / values must match
    // @property (readonly, nonatomic) MLFeatureDescription *valueDescription;
    pub fn value_description(&self) -> &MLFeatureDescriptionRef {
        unsafe { msg_send![self, valueDescription] }
    }

    // // Restriction on the length of the sequence
    // @property (readonly, nonatomic) NSRange countRange;
    pub fn count_range(&self) -> NSRange {
        unsafe { msg_send![self, countRange] }
    }

    // @end
}
// NS_ASSUME_NONNULL_END
