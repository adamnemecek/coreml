use crate::prelude::*;
// #import <CoreML/CoreML.h>

// NS_ASSUME_NONNULL_BEGIN

// /*!
//  * Events on which update task is capable of invoking progress handler.
//  *
//  * @note MLUpdateProgressEventMiniBatchEnd may induce performance problems
//  *       during pipeline execution.
//  */
// API_AVAILABLE(macos(10.15), ios(13.0), tvos(14.0))
// typedef NS_OPTIONS(NSInteger, MLUpdateProgressEvent) {
//     MLUpdateProgressEventTrainingBegin = 1 << 0,
//     MLUpdateProgressEventEpochEnd = 1 << 1,
//     MLUpdateProgressEventMiniBatchEnd = 1 << 2,
// };

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MLUpdateProgressEvent {
    TrainingBegin = 1 << 0,
    EpochEnd = 1 << 1,
    MiniBatchEnd = 1 << 2,
}

// NS_ASSUME_NONNULL_END
