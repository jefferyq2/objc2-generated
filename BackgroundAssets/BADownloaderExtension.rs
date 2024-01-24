//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::BackgroundAssets::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait BADownloaderExtension: NSObjectProtocol {
        #[cfg(all(
            feature = "BackgroundAssets_BAAppExtensionInfo",
            feature = "BackgroundAssets_BADownload",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSURL"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other downloadsForRequest:manifestURL:extensionInfo:)]
        unsafe fn downloadsForRequest_manifestURL_extensionInfo(
            &self,
            content_request: BAContentRequest,
            manifest_url: &NSURL,
            extension_info: &BAAppExtensionInfo,
        ) -> Id<NSSet<BADownload>>;

        #[cfg(all(
            feature = "BackgroundAssets_BADownload",
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "Foundation_NSURLCredential"
        ))]
        #[optional]
        #[method(backgroundDownload:didReceiveChallenge:completionHandler:)]
        unsafe fn backgroundDownload_didReceiveChallenge_completionHandler(
            &self,
            download: &BADownload,
            challenge: &NSURLAuthenticationChallenge,
            completion_handler: &Block<
                dyn Fn(NSURLSessionAuthChallengeDisposition, *mut NSURLCredential),
            >,
        );

        #[cfg(all(
            feature = "BackgroundAssets_BADownload",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(backgroundDownload:failedWithError:)]
        unsafe fn backgroundDownload_failedWithError(&self, download: &BADownload, error: &NSError);

        #[cfg(all(feature = "BackgroundAssets_BADownload", feature = "Foundation_NSURL"))]
        #[optional]
        #[method(backgroundDownload:finishedWithFileURL:)]
        unsafe fn backgroundDownload_finishedWithFileURL(
            &self,
            download: &BADownload,
            file_url: &NSURL,
        );

        #[deprecated = "extensionWillTerminate will not be invoked in all applicable circumstances and should not be relied upon."]
        #[optional]
        #[method(extensionWillTerminate)]
        unsafe fn extensionWillTerminate(&self);
    }

    unsafe impl ProtocolType for dyn BADownloaderExtension {}
);
