use crate::prelude::*;
// #import <Foundation/Foundation.h>
// #import <CoreML/MLExport.h>

// NS_ASSUME_NONNULL_BEGIN

// /*!
//  * All possible states an MLTask can be in.
//  */
// API_AVAILABLE(macos(10.15), ios(13.0), tvos(14.0))
// typedef NS_ENUM(NSInteger, MLTaskState) {
//     MLTaskStateSuspended = 1,
//     MLTaskStateRunning = 2,
//     MLTaskStateCancelling = 3,
//     MLTaskStateCompleted = 4,
//     MLTaskStateFailed = 5,
// };

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MLTaskState {
    Suspended = 1,
    Running = 2,
    Cancelling = 3,
    Completed = 4,
    Failed = 5,
}

// /*!
//  * Class that abstracts state transitions and basic task controls.
//  */
// ML_EXPORT
// API_AVAILABLE(macos(10.15), ios(13.0), tvos(14.0))
// @interface MLTask : NSObject

pub enum MLTaskFFI {}

foreign_obj_type! {
    type CType = MLTaskFFI;
    pub struct MLTask;
    pub struct MLTaskRef;
}

impl MLTaskRef {
    // Unique identifier for the task.
    // @property (readonly, copy, nonatomic) NSString *taskIdentifier;

    pub fn task_identifier(&self) -> &str {
        unsafe {
            let s = msg_send![self, taskIdentifier];
            crate::nsstring_as_str(s)
        }
    }

    // Represents the current state of task.
    // @property (readonly, assign, atomic) MLTaskState state;
    pub fn state(&self) -> MLTaskState {
        unsafe { msg_send![self, state] }
    }

    // Indicates error if the task failed for any reason.
    // @property (readonly, copy, atomic, nullable) NSError *error;
    pub fn error(&self) -> Option<&NSErrorRef> {
        unsafe { msg_send![self, error] }
    }

    // When called, resumes the task and changes state to "Running".
    // - (void)resume;
    pub fn resume(&self) {
        unsafe { msg_send![self, resume] }
    }

    // When called, starts cancelling the task and changes the state to "Cancelling".
    // - (void)cancel;
    pub fn cancel(&self) {
        unsafe { msg_send![self, cancel] }
    }

    // // cannot construct MLTask without parameters.
    // - (instancetype)init NS_UNAVAILABLE;

    // // cannot construct MLTask without parameters.
    // + (id)new NS_UNAVAILABLE;

    // @end

    // NS_ASSUME_NONNULL_END
}
