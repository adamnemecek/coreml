use cocoa_foundation::foundation::NSInteger;

use crate::prelude::*;
// #import <Foundation/Foundation.h>
// #import <CoreML/MLImageSizeConstraint.h>
// #import <CoreML/MLExport.h>

// NS_ASSUME_NONNULL_BEGIN

// /*!
//  * MLImageConstraint
//  *
//  * Constraint on image properties.
//  */
// API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0))
// ML_EXPORT
// @interface MLImageConstraint : NSObject <NSSecureCoding>

pub enum MLImageConstraintFFI {}
foreign_obj_type! {
    type CType = MLImageConstraintFFI;
    pub struct MLImageConstraint;
    pub struct MLImageConstraintRef;
}

// /// The required or default height of the image
// @property (readonly, nonatomic) NSInteger pixelsHigh;

impl MLImageConstraintRef {
    // /// The required or default width of the image
    // @property (readonly, nonatomic) NSInteger pixelsWide;
    pub fn pixels_wide(&self) -> NSInteger {
        unsafe { msg_send![self, pixelsWide] }
    }

    // /// The accepted kCVPixelFormatType for the image.
    // @property (readonly, nonatomic) OSType pixelFormatType;

    // /// Detailed image size constraint
    // @property (readonly, nonatomic) MLImageSizeConstraint *sizeConstraint API_AVAILABLE(macos(10.14), ios(12.0), watchos(5.0), tvos(12.0));

    // // cannot construct object without parameters.
    // - (instancetype)init NS_UNAVAILABLE NS_SWIFT_UNAVAILABLE("");

    // @end

    // NS_ASSUME_NONNULL_END
}
