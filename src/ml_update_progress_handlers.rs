use crate::prelude::*;
// #import <CoreML/CoreML.h>

// NS_ASSUME_NONNULL_BEGIN

// @class MLUpdateContext;

// /*!
//  * Allows applications to register for progress and completion handlers.
//  */
// ML_EXPORT
// API_AVAILABLE(macos(10.15), ios(13.0), tvos(14.0))
// @interface MLUpdateProgressHandlers : NSObject

pub enum MLUpdateProgressHandlersFFI {}

foreign_obj_type! {
    type CType = MLUpdateProgressHandlersFFI;
    pub struct MLUpdateProgressHandlers;
    pub struct MLUpdateProgressHandlersRef;
}

impl MLUpdateProgressHandlersRef {
    // - (instancetype)initForEvents:(MLUpdateProgressEvent)interestedEvents
    //               progressHandler:(nullable void (^)(MLUpdateContext * context))progressHandler
    //             completionHandler:(void (^)(MLUpdateContext * context))completionHandler;

    // // cannot construct MLUpdateTask without parameters.
    // - (instancetype)init NS_UNAVAILABLE;

    // // cannot construct MLUpdateTask without parameters.
    // + (id)new NS_UNAVAILABLE;

    // @end

    // NS_ASSUME_NONNULL_END
}
