use crate::prelude::*;

// NS_ASSUME_NONNULL_BEGIN

// /*!
//  * Description of a feature
//  */
// API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0))
// ML_EXPORT
// @interface MLFeatureDescription : NSObject<NSCopying, NSSecureCoding>

pub enum MLFeatureDescriptionFFI {}
foreign_obj_type! {
    type CType = MLFeatureDescriptionFFI;
    pub struct MLFeatureDescription;
    pub struct MLFeatureDescriptionRef;
}

impl MLFeatureDescriptionRef {
    /// Name of feature
    // @property (readonly, nonatomic, copy) NSString *name;
    pub fn name(&self) -> &str {
        unsafe {
            let name = msg_send![self, name];
            crate::nsstring_as_str(name)
        }
    }

    /// Type of data
    // @property (readonly, nonatomic) MLFeatureType type;
    pub fn type_(&self) -> MLFeatureType {
        unsafe { msg_send![self, type] }
    }

    /// Whether this feature can take an undefined value or not
    // @property (readonly, nonatomic, getter=isOptional) BOOL optional;
    pub fn optional(&self) -> bool {
        unsafe { msg_send![self, optional] }
    }

    /// Check if MLFeatureValue is valid based on this description
    // - (BOOL)isAllowedValue:(MLFeatureValue *)value;
    pub fn is_allowed_value(&self, value: &MLFeatureValueRef) -> bool {
        unsafe { msg_send![self, isAllowedValue: value] }
    }

    // @end
}

// /*!
//  * Feature value constraints for specific types
//  */
// API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0))
// @interface MLFeatureDescription (MLFeatureValueConstraints)

// /// Constraint when type == MLFeatureTypeMultiArray, nil otherwise
// @property (readonly, nullable, nonatomic) MLMultiArrayConstraint *multiArrayConstraint;

// /// Constraint when type == MLFeatureTypeImage, nil otherwise
// @property (readonly, nullable, nonatomic) MLImageConstraint *imageConstraint;

// /// Constraint when type == MLFeatureTypeDictionary, nil otherwise
// @property (readonly, nullable, nonatomic) MLDictionaryConstraint *dictionaryConstraint;

// /// Constraint when type == MLFeatureTypeSequence, nil otherwise
// @property (readonly, nullable, nonatomic) MLSequenceConstraint *sequenceConstraint API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0));

// @end

// NS_ASSUME_NONNULL_END
