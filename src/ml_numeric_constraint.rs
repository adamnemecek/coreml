use crate::prelude::*;
// #import <CoreML/CoreML.h>

// NS_ASSUME_NONNULL_BEGIN

// /*!
//  * Allows enforcement of constraints on the values of update parameters.
//  */
// ML_EXPORT
// API_AVAILABLE(macos(10.15), ios(13.0), tvos(14.0))
// @interface MLNumericConstraint : NSObject<NSSecureCoding>

pub enum MLNumericConstraintFFI {}

foreign_obj_type! {
    type CType = MLNumericConstraintFFI;
    pub struct MLNumericConstraint;
    pub struct MLNumericConstraintRef;
}

impl MLNumericConstraintRef {
    // // Minimum value of the parameter can take.
    // @property (readonly, nonatomic) NSNumber *minNumber;
    pub fn min_number(&self) -> &NSNumberRef {
        unsafe { msg_send![self, minNumber] }
    }

    // // Maximum value of the parameter can take.
    // @property (readonly, nonatomic) NSNumber *maxNumber;
    pub fn max_number(&self) -> &NSNumberRef {
        unsafe { msg_send![self, maxNumber] }
    }

    // // If not nil, list of restricted set of values the parameter can take.
    // @property (readonly, nonatomic, nullable) NSSet<NSNumber *> *enumeratedNumbers;

    // @end

    // NS_ASSUME_NONNULL_END
}
