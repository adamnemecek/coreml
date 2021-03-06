use crate::prelude::*;
// #import <Foundation/Foundation.h>
// #import <CoreML/MLExport.h>

// NS_ASSUME_NONNULL_BEGIN

// /*!
//  * A class representing key used to store any value against
//  */
// ML_EXPORT
// API_AVAILABLE(macos(10.15), ios(13.0), tvos(14.0))
// @interface MLKey : NSObject <NSCopying, NSSecureCoding>

// // Name of the key
// @property (readonly, nonatomic) NSString *name;

// // scope of the key
// @property (readonly, nonatomic, nullable) NSString *scope;

// // cannot construct MLKey without parameters.
// - (instancetype)init NS_UNAVAILABLE;

// // cannot construct MLKey without parameters.
// + (id)new NS_UNAVAILABLE;

// @end

// NS_ASSUME_NONNULL_END

pub enum MLKeyFFI {}

foreign_obj_type! {
    type CType = MLKeyFFI;
    pub struct MLKey;
    pub struct MLKeyRef;
}

impl MLKeyRef {
    pub fn name(&self) -> &str {
        unsafe { crate::nsstring_as_str(msg_send![self, name]) }
    }
}
