use crate::prelude::*;
// #import <CoreML/CoreML.h>

// NS_ASSUME_NONNULL_BEGIN

// @class MLNumericConstraint;

// /*!
//  * Describes a model parameter along with a default value and any applicable constaint on the values.
//  */
// ML_EXPORT
// API_AVAILABLE(macos(10.15), ios(13.0), tvos(14.0))
// @interface MLParameterDescription : NSObject<NSSecureCoding>
pub enum MLParameterDescriptionFFI {}

foreign_obj_type! {
    type CType = MLParameterDescriptionFFI;
    pub struct MLParameterDescription;
    pub struct MLParameterDescriptionRef;
}

impl MLParameterDescriptionRef {
    // // Name and type of the parameter
    // @property (readonly, nonatomic) MLParameterKey *key;
    pub fn key(&self) -> &MLParameterKeyRef {
        unsafe { msg_send![self, key] }
    }

    // // Default value of the parameter
    // @property (readonly, nonatomic) id defaultValue;

    // // Any applicable constraint on the parameter value
    // @property (readonly, nonatomic, nullable) MLNumericConstraint *numericConstraint;
    pub fn numeric_constraint(&self) -> &MLNumericConstraintRef {
        unsafe { msg_send![self, numericConstraint] }
    }

    // @end

    // NS_ASSUME_NONNULL_END
}
