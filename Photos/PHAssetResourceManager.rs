//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

pub type PHAssetResourceDataRequestID = i32;

extern_static!(PHInvalidAssetResourceDataRequestID: PHAssetResourceDataRequestID = 0);

pub type PHAssetResourceProgressHandler = *mut Block<dyn Fn(c_double)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHAssetResourceRequestOptions")]
    pub struct PHAssetResourceRequestOptions;

    #[cfg(feature = "PhotoKit_PHAssetResourceRequestOptions")]
    unsafe impl ClassType for PHAssetResourceRequestOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "PhotoKit_PHAssetResourceRequestOptions")]
unsafe impl NSCopying for PHAssetResourceRequestOptions {}

#[cfg(feature = "PhotoKit_PHAssetResourceRequestOptions")]
unsafe impl NSObjectProtocol for PHAssetResourceRequestOptions {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHAssetResourceRequestOptions")]
    unsafe impl PHAssetResourceRequestOptions {
        #[method(isNetworkAccessAllowed)]
        pub unsafe fn isNetworkAccessAllowed(&self) -> bool;

        #[method(setNetworkAccessAllowed:)]
        pub unsafe fn setNetworkAccessAllowed(&self, network_access_allowed: bool);

        #[method(progressHandler)]
        pub unsafe fn progressHandler(&self) -> PHAssetResourceProgressHandler;

        #[method(setProgressHandler:)]
        pub unsafe fn setProgressHandler(&self, progress_handler: PHAssetResourceProgressHandler);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "PhotoKit_PHAssetResourceRequestOptions")]
    unsafe impl PHAssetResourceRequestOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHAssetResourceManager")]
    pub struct PHAssetResourceManager;

    #[cfg(feature = "PhotoKit_PHAssetResourceManager")]
    unsafe impl ClassType for PHAssetResourceManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "PhotoKit_PHAssetResourceManager")]
unsafe impl NSObjectProtocol for PHAssetResourceManager {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHAssetResourceManager")]
    unsafe impl PHAssetResourceManager {
        #[method_id(@__retain_semantics Other defaultManager)]
        pub unsafe fn defaultManager() -> Id<PHAssetResourceManager>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "PhotoKit_PHAssetResource",
            feature = "PhotoKit_PHAssetResourceRequestOptions"
        ))]
        #[method(requestDataForAssetResource:options:dataReceivedHandler:completionHandler:)]
        pub unsafe fn requestDataForAssetResource_options_dataReceivedHandler_completionHandler(
            &self,
            resource: &PHAssetResource,
            options: Option<&PHAssetResourceRequestOptions>,
            handler: &Block<dyn Fn(NonNull<NSData>)>,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        ) -> PHAssetResourceDataRequestID;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL",
            feature = "PhotoKit_PHAssetResource",
            feature = "PhotoKit_PHAssetResourceRequestOptions"
        ))]
        #[method(writeDataForAssetResource:toFile:options:completionHandler:)]
        pub unsafe fn writeDataForAssetResource_toFile_options_completionHandler(
            &self,
            resource: &PHAssetResource,
            file_url: &NSURL,
            options: Option<&PHAssetResourceRequestOptions>,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[method(cancelDataRequest:)]
        pub unsafe fn cancelDataRequest(&self, request_id: PHAssetResourceDataRequestID);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "PhotoKit_PHAssetResourceManager")]
    unsafe impl PHAssetResourceManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
