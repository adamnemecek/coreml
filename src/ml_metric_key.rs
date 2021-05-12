use crate::prelude::*;
// #import <CoreML/CoreML.h>

// NS_ASSUME_NONNULL_BEGIN

// /*!
//  * A class to specify list of supported model update metrics.
//  */
// ML_EXPORT
// API_AVAILABLE(macos(10.15), ios(13.0), tvos(14.0))
// @interface MLMetricKey : MLKey

pub enum MLMetricKeyFFI {}

foreign_obj_type! {
    type CType = MLMetricKeyFFI;
    pub struct MLMetricKey;
    pub struct MLMetricKeyRef;
    type ParentType = MLKeyRef;
}

impl MLMetricKeyRef {
    // // Float metric indicating the current loss
    // @property (class, readonly, nonatomic) MLMetricKey *lossValue;
    pub fn loss_value(&self) -> &MLMetricKeyRef {
        unsafe { msg_send![self, lossValue] }
    }

    // // Int64 metric indicating the index of the epoch
    // @property (class, readonly, nonatomic) MLMetricKey *epochIndex;
    pub fn epoch_index(&self) -> &MLMetricKeyRef {
        unsafe { msg_send![self, epochIndex] }
    }

    // // Int64 metric indicating the index of mini batches in the current epoch
    // @property (class, readonly, nonatomic) MLMetricKey *miniBatchIndex;
    pub fn mini_batch_index(&self) -> &MLMetricKeyRef {
        unsafe { msg_send![self, miniBatchIndex] }
    }

    // // cannot construct MLMetricKey without parameters.
    // - (instancetype)init NS_UNAVAILABLE;

    // // cannot construct MLMetricKey without parameters.
    // + (id)new NS_UNAVAILABLE;

    // @end
}

// NS_ASSUME_NONNULL_END
