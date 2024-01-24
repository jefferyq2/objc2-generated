//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "FileProvider_NSFileProviderExtension")]
    pub struct NSFileProviderExtension;

    #[cfg(feature = "FileProvider_NSFileProviderExtension")]
    unsafe impl ClassType for NSFileProviderExtension {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "FileProvider_NSFileProviderExtension")]
unsafe impl NSObjectProtocol for NSFileProviderExtension {}

extern_methods!(
    #[cfg(feature = "FileProvider_NSFileProviderExtension")]
    unsafe impl NSFileProviderExtension {
        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other itemForIdentifier:error:_)]
        pub unsafe fn itemForIdentifier_error(
            &self,
            identifier: &NSFileProviderItemIdentifier,
        ) -> Result<Id<NSFileProviderItem>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URLForItemWithPersistentIdentifier:)]
        pub unsafe fn URLForItemWithPersistentIdentifier(
            &self,
            identifier: &NSFileProviderItemIdentifier,
        ) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other persistentIdentifierForItemAtURL:)]
        pub unsafe fn persistentIdentifierForItemAtURL(
            &self,
            url: &NSURL,
        ) -> Option<Id<NSFileProviderItemIdentifier>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(providePlaceholderAtURL:completionHandler:)]
        pub unsafe fn providePlaceholderAtURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(startProvidingItemAtURL:completionHandler:)]
        pub unsafe fn startProvidingItemAtURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[method(stopProvidingItemAtURL:)]
        pub unsafe fn stopProvidingItemAtURL(&self, url: &NSURL);

        #[cfg(feature = "Foundation_NSURL")]
        #[method(itemChangedAtURL:)]
        pub unsafe fn itemChangedAtURL(&self, url: &NSURL);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "FileProvider_NSFileProviderExtension")]
    unsafe impl NSFileProviderExtension {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// Deprecated
    #[cfg(feature = "FileProvider_NSFileProviderExtension")]
    unsafe impl NSFileProviderExtension {
        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[deprecated = "Use the corresponding method on NSFileProviderManager instead"]
        #[method(writePlaceholderAtURL:withMetadata:error:_)]
        pub unsafe fn writePlaceholderAtURL_withMetadata_error(
            placeholder_url: &NSURL,
            metadata: &NSDictionary<NSURLResourceKey, AnyObject>,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[deprecated]
        #[method_id(@__retain_semantics Other placeholderURLForURL:)]
        pub unsafe fn placeholderURLForURL(url: &NSURL) -> Id<NSURL>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other providerIdentifier)]
        pub unsafe fn providerIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSURL")]
        #[deprecated]
        #[method_id(@__retain_semantics Other documentStorageURL)]
        pub unsafe fn documentStorageURL(&self) -> Id<NSURL>;
    }
);
