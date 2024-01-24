//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSFilePresenter: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other presentedItemURL)]
        unsafe fn presentedItemURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSOperationQueue")]
        #[method_id(@__retain_semantics Other presentedItemOperationQueue)]
        unsafe fn presentedItemOperationQueue(&self) -> Id<NSOperationQueue>;

        #[cfg(feature = "Foundation_NSURL")]
        #[optional]
        #[method_id(@__retain_semantics Other primaryPresentedItemURL)]
        unsafe fn primaryPresentedItemURL(&self) -> Option<Id<NSURL>>;

        #[optional]
        #[method(relinquishPresentedItemToReader:)]
        unsafe fn relinquishPresentedItemToReader(
            &self,
            reader: &Block<dyn Fn(*mut Block<dyn Fn()>)>,
        );

        #[optional]
        #[method(relinquishPresentedItemToWriter:)]
        unsafe fn relinquishPresentedItemToWriter(
            &self,
            writer: &Block<dyn Fn(*mut Block<dyn Fn()>)>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[optional]
        #[method(savePresentedItemChangesWithCompletionHandler:)]
        unsafe fn savePresentedItemChangesWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[optional]
        #[method(accommodatePresentedItemDeletionWithCompletionHandler:)]
        unsafe fn accommodatePresentedItemDeletionWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[optional]
        #[method(presentedItemDidMoveToURL:)]
        unsafe fn presentedItemDidMoveToURL(&self, new_url: &NSURL);

        #[optional]
        #[method(presentedItemDidChange)]
        unsafe fn presentedItemDidChange(&self);

        #[cfg(feature = "Foundation_NSSet")]
        #[optional]
        #[method(presentedItemDidChangeUbiquityAttributes:)]
        unsafe fn presentedItemDidChangeUbiquityAttributes(
            &self,
            attributes: &NSSet<NSURLResourceKey>,
        );

        #[cfg(feature = "Foundation_NSSet")]
        #[optional]
        #[method_id(@__retain_semantics Other observedPresentedItemUbiquityAttributes)]
        unsafe fn observedPresentedItemUbiquityAttributes(&self) -> Id<NSSet<NSURLResourceKey>>;

        #[cfg(feature = "Foundation_NSFileVersion")]
        #[optional]
        #[method(presentedItemDidGainVersion:)]
        unsafe fn presentedItemDidGainVersion(&self, version: &NSFileVersion);

        #[cfg(feature = "Foundation_NSFileVersion")]
        #[optional]
        #[method(presentedItemDidLoseVersion:)]
        unsafe fn presentedItemDidLoseVersion(&self, version: &NSFileVersion);

        #[cfg(feature = "Foundation_NSFileVersion")]
        #[optional]
        #[method(presentedItemDidResolveConflictVersion:)]
        unsafe fn presentedItemDidResolveConflictVersion(&self, version: &NSFileVersion);

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[optional]
        #[method(accommodatePresentedSubitemDeletionAtURL:completionHandler:)]
        unsafe fn accommodatePresentedSubitemDeletionAtURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[optional]
        #[method(presentedSubitemDidAppearAtURL:)]
        unsafe fn presentedSubitemDidAppearAtURL(&self, url: &NSURL);

        #[cfg(feature = "Foundation_NSURL")]
        #[optional]
        #[method(presentedSubitemAtURL:didMoveToURL:)]
        unsafe fn presentedSubitemAtURL_didMoveToURL(&self, old_url: &NSURL, new_url: &NSURL);

        #[cfg(feature = "Foundation_NSURL")]
        #[optional]
        #[method(presentedSubitemDidChangeAtURL:)]
        unsafe fn presentedSubitemDidChangeAtURL(&self, url: &NSURL);

        #[cfg(all(feature = "Foundation_NSFileVersion", feature = "Foundation_NSURL"))]
        #[optional]
        #[method(presentedSubitemAtURL:didGainVersion:)]
        unsafe fn presentedSubitemAtURL_didGainVersion(&self, url: &NSURL, version: &NSFileVersion);

        #[cfg(all(feature = "Foundation_NSFileVersion", feature = "Foundation_NSURL"))]
        #[optional]
        #[method(presentedSubitemAtURL:didLoseVersion:)]
        unsafe fn presentedSubitemAtURL_didLoseVersion(&self, url: &NSURL, version: &NSFileVersion);

        #[cfg(all(feature = "Foundation_NSFileVersion", feature = "Foundation_NSURL"))]
        #[optional]
        #[method(presentedSubitemAtURL:didResolveConflictVersion:)]
        unsafe fn presentedSubitemAtURL_didResolveConflictVersion(
            &self,
            url: &NSURL,
            version: &NSFileVersion,
        );
    }

    unsafe impl ProtocolType for dyn NSFilePresenter {}
);
