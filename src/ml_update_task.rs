use crate::prelude::*;
// #import <CoreML/CoreML.h>

// NS_ASSUME_NONNULL_BEGIN

// @class MLUpdateContext;
// @class MLUpdateProgressHandlers;

// /*!
//  * Main class for setting up and controlling a model update. It provides some utility class methods for performing an update synchronously as well as class constructors for configuring an update and give developers control for the execution of that update.
//  */
// ML_EXPORT
// API_AVAILABLE(macos(10.15), ios(13.0), tvos(14.0))
// @interface MLUpdateTask : MLTask

pub enum MLUpdateTaskFFI {}

foreign_obj_type! {
    type CType = MLUpdateTaskFFI;
    pub struct MLUpdateTask;
    pub struct MLUpdateTaskRef;
    type ParentType = MLTaskRef;
}

impl MLUpdateTask {
    // // Update via task control with completion handler
    // + (nullable instancetype)updateTaskForModelAtURL:(NSURL *)modelURL
    //                                     trainingData:(id<MLBatchProvider>)trainingData
    //                                    configuration:(nullable MLModelConfiguration *)configuration
    //                                completionHandler:(void (^)(MLUpdateContext * context))completionHandler
    //                                            error:(NSError * _Nullable * _Nullable)error;

    pub fn update_task_for_model() -> Self {
        todo!()
    }
    // // Update via task control with completion handler supplying default configuration
    // + (nullable instancetype)updateTaskForModelAtURL:(NSURL *)modelURL
    //                            trainingData:(id<MLBatchProvider>)trainingData
    //                       completionHandler:(void (^)(MLUpdateContext * _Nonnull))completionHandler
    //                                   error:(NSError * _Nullable __autoreleasing *)error API_AVAILABLE(macos(11.0), ios(14.0), watchos(7.0), tvos(14.0));
    // pub fn update_task_for_model() -> Self {
    //     todo!()
    // }
    // // Update via task control and custom progress callbacks
    // + (nullable instancetype)updateTaskForModelAtURL:(NSURL *)modelURL
    //                                     trainingData:(id<MLBatchProvider>)trainingData
    //                                    configuration:(nullable MLModelConfiguration *)configuration
    //                                 progressHandlers:(MLUpdateProgressHandlers *)progressHandlers
    //                                            error:(NSError * _Nullable * _Nullable)error;

    // // Update via task control and custom progress callbacks supplying default configuration
    // + (nullable instancetype)updateTaskForModelAtURL:(NSURL *)modelURL
    //                            trainingData:(id<MLBatchProvider>)trainingData
    //                        progressHandlers:(MLUpdateProgressHandlers *)progressHandlers
    //                                   error:(NSError * _Nullable __autoreleasing *)error API_AVAILABLE(macos(11.0), ios(14.0), watchos(7.0), tvos(14.0));
}

impl MLUpdateTaskRef {
    // // Request a resume with new parameters. Should be used within a progressHandler
    // - (void)resumeWithParameters:(NSDictionary<MLParameterKey *, id> *)updateParameters;
    pub fn resume_with_parameters(&self) {
        todo!()
    }

    // // cannot construct MLUpdateTask without parameters.
    // - (instancetype)init NS_UNAVAILABLE;

    // // cannot construct MLUpdateTask without parameters.
    // + (id)new NS_UNAVAILABLE;

    // @end

    // NS_ASSUME_NONNULL_END
}
