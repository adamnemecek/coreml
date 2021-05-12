use crate::prelude::*;
// #import <Foundation/Foundation.h>
// #import <CoreML/MLExport.h>

// NS_ASSUME_NONNULL_BEGIN

// /*!
//  * MLPredictionOptions
//  *
//  * An object to hold options / controls / parameters of how
//  * model prediction is performed
//  */
// API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0))
// ML_EXPORT
// @interface MLPredictionOptions : NSObject

// // Set to YES to force computation to be on the CPU only
// @property (readwrite, nonatomic) BOOL usesCPUOnly;

// @end

pub enum MLPredictionOptionsFFI {}

foreign_obj_type! {
    type CType = MLPredictionOptionsFFI;
    pub struct MLPredictionOptions;
    pub struct MLPredictionOptionsRef;
}

impl MLPredictionOptionsRef {
    pub fn uses_cpu_only(&self) -> bool {
        unsafe { msg_send![self, usesCPUOnly] }
    }
}

// NS_ASSUME_NONNULL_END
