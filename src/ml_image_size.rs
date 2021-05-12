use cocoa_foundation::foundation::NSInteger;

use crate::prelude::*;
// #import <Foundation/Foundation.h>
// #import <CoreML/MLExport.h>

// NS_ASSUME_NONNULL_BEGIN

// API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0))
// ML_EXPORT
// @interface MLImageSize : NSObject <NSSecureCoding>

pub enum MLImageSizeFFI {}

foreign_obj_type! {
    type CType = MLImageSizeFFI;
    pub struct MLImageSize;
    pub struct MLImageSizeRef;
}

impl MLImageSizeRef {
    // @property (readonly) NSInteger pixelsWide;
    pub fn pixels_wide(&self) -> NSInteger {
        unsafe { msg_send![self, pixelsWide] }
    }

    // @property (readonly) NSInteger pixelsHigh;
    pub fn pixels_high(&self) -> NSInteger {
        unsafe { msg_send![self, pixelsHigh] }
    }

    // @end

    // NS_ASSUME_NONNULL_END
}
