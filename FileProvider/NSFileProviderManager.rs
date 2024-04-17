//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderDomainRemovalMode(pub NSInteger);
impl NSFileProviderDomainRemovalMode {
    #[doc(alias = "NSFileProviderDomainRemovalModeRemoveAll")]
    pub const RemoveAll: Self = Self(0);
    #[doc(alias = "NSFileProviderDomainRemovalModePreserveDirtyUserData")]
    pub const PreserveDirtyUserData: Self = Self(1);
    #[doc(alias = "NSFileProviderDomainRemovalModePreserveDownloadedUserData")]
    pub const PreserveDownloadedUserData: Self = Self(2);
}

unsafe impl Encode for NSFileProviderDomainRemovalMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderDomainRemovalMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFileProviderManager;

    unsafe impl ClassType for NSFileProviderManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSFileProviderManager {}

extern_methods!(
    unsafe impl NSFileProviderManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other defaultManager)]
        pub unsafe fn defaultManager() -> Id<NSFileProviderManager>;

        #[cfg(feature = "NSFileProviderDomain")]
        #[method_id(@__retain_semantics Other managerForDomain:)]
        pub unsafe fn managerForDomain(domain: &NSFileProviderDomain) -> Option<Id<Self>>;

        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(signalEnumeratorForContainerItemIdentifier:completionHandler:)]
        pub unsafe fn signalEnumeratorForContainerItemIdentifier_completionHandler(
            &self,
            container_item_identifier: &NSFileProviderItemIdentifier,
            completion: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(getUserVisibleURLForItemIdentifier:completionHandler:)]
        pub unsafe fn getUserVisibleURLForItemIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<dyn Fn(*mut NSURL, *mut NSError)>,
        );

        #[cfg(all(
            feature = "NSFileProviderDomain",
            feature = "NSFileProviderItem",
            feature = "block2"
        ))]
        #[method(getIdentifierForUserVisibleFileAtURL:completionHandler:)]
        pub unsafe fn getIdentifierForUserVisibleFileAtURL_completionHandler(
            url: &NSURL,
            completion_handler: &Block<
                dyn Fn(
                    *mut NSFileProviderItemIdentifier,
                    *mut NSFileProviderDomainIdentifier,
                    *mut NSError,
                ),
            >,
        );

        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(registerURLSessionTask:forItemWithIdentifier:completionHandler:)]
        pub unsafe fn registerURLSessionTask_forItemWithIdentifier_completionHandler(
            &self,
            task: &NSURLSessionTask,
            identifier: &NSFileProviderItemIdentifier,
            completion: &Block<dyn Fn(*mut NSError)>,
        );

        #[method_id(@__retain_semantics Other providerIdentifier)]
        pub unsafe fn providerIdentifier(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other documentStorageURL)]
        pub unsafe fn documentStorageURL(&self) -> Id<NSURL>;

        #[method_id(@__retain_semantics Other temporaryDirectoryURLWithError:_)]
        pub unsafe fn temporaryDirectoryURLWithError(&self) -> Result<Id<NSURL>, Id<NSError>>;

        #[cfg(feature = "NSFileProviderItem")]
        #[method(writePlaceholderAtURL:withMetadata:error:_)]
        pub unsafe fn writePlaceholderAtURL_withMetadata_error(
            placeholder_url: &NSURL,
            metadata: &NSFileProviderItem,
        ) -> Result<(), Id<NSError>>;

        #[method_id(@__retain_semantics Other placeholderURLForURL:)]
        pub unsafe fn placeholderURLForURL(url: &NSURL) -> Id<NSURL>;

        #[cfg(all(feature = "NSFileProviderDomain", feature = "block2"))]
        #[method(addDomain:completionHandler:)]
        pub unsafe fn addDomain_completionHandler(
            domain: &NSFileProviderDomain,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "NSFileProviderDomain", feature = "block2"))]
        #[method(removeDomain:completionHandler:)]
        pub unsafe fn removeDomain_completionHandler(
            domain: &NSFileProviderDomain,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "NSFileProviderDomain", feature = "block2"))]
        #[method(removeDomain:mode:completionHandler:)]
        pub unsafe fn removeDomain_mode_completionHandler(
            domain: &NSFileProviderDomain,
            mode: NSFileProviderDomainRemovalMode,
            completion_handler: &Block<dyn Fn(*mut NSURL, *mut NSError)>,
        );

        #[cfg(all(feature = "NSFileProviderDomain", feature = "block2"))]
        #[method(getDomainsWithCompletionHandler:)]
        pub unsafe fn getDomainsWithCompletionHandler(
            completion_handler: &Block<
                dyn Fn(NonNull<NSArray<NSFileProviderDomain>>, *mut NSError),
            >,
        );

        #[cfg(feature = "block2")]
        #[method(removeAllDomainsWithCompletionHandler:)]
        pub unsafe fn removeAllDomainsWithCompletionHandler(
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(signalErrorResolved:completionHandler:)]
        pub unsafe fn signalErrorResolved_completionHandler(
            &self,
            error: &NSError,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[method_id(@__retain_semantics Other globalProgressForKind:)]
        pub unsafe fn globalProgressForKind(
            &self,
            kind: &NSProgressFileOperationKind,
        ) -> Id<NSProgress>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFileProviderManager {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    pub static NSFileProviderMaterializedSetDidChange: &'static NSNotificationName;
}

extern_methods!(
    /// MaterializedSet
    unsafe impl NSFileProviderManager {
        #[cfg(feature = "NSFileProviderEnumerating")]
        #[method_id(@__retain_semantics Other enumeratorForMaterializedItems)]
        pub unsafe fn enumeratorForMaterializedItems(
            &self,
        ) -> Id<ProtocolObject<dyn NSFileProviderEnumerator>>;
    }
);

extern "C" {
    pub static NSFileProviderPendingSetDidChange: &'static NSNotificationName;
}

extern_protocol!(
    #[cfg(feature = "NSFileProviderEnumerating")]
    pub unsafe trait NSFileProviderPendingSetEnumerator: NSFileProviderEnumerator {
        #[cfg(feature = "NSFileProviderDomain")]
        #[method_id(@__retain_semantics Other domainVersion)]
        unsafe fn domainVersion(&self) -> Option<Id<NSFileProviderDomainVersion>>;

        #[method(refreshInterval)]
        unsafe fn refreshInterval(&self) -> NSTimeInterval;

        #[method(isMaximumSizeReached)]
        unsafe fn isMaximumSizeReached(&self) -> bool;
    }

    #[cfg(feature = "NSFileProviderEnumerating")]
    unsafe impl ProtocolType for dyn NSFileProviderPendingSetEnumerator {}
);

extern_methods!(
    /// PendingSet
    unsafe impl NSFileProviderManager {
        #[cfg(feature = "NSFileProviderEnumerating")]
        #[method_id(@__retain_semantics Other enumeratorForPendingItems)]
        pub unsafe fn enumeratorForPendingItems(
            &self,
        ) -> Id<ProtocolObject<dyn NSFileProviderPendingSetEnumerator>>;
    }
);

extern_methods!(
    /// Import
    unsafe impl NSFileProviderManager {
        #[cfg(all(feature = "NSFileProviderDomain", feature = "block2"))]
        #[method(importDomain:fromDirectoryAtURL:completionHandler:)]
        pub unsafe fn importDomain_fromDirectoryAtURL_completionHandler(
            domain: &NSFileProviderDomain,
            url: &NSURL,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(reimportItemsBelowItemWithIdentifier:completionHandler:)]
        pub unsafe fn reimportItemsBelowItemWithIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(
            feature = "NSFileProviderItem",
            feature = "NSFileProviderModifyItemOptions",
            feature = "block2"
        ))]
        #[method(requestModificationOfFields:forItemWithIdentifier:options:completionHandler:)]
        pub unsafe fn requestModificationOfFields_forItemWithIdentifier_options_completionHandler(
            &self,
            fields: NSFileProviderItemFields,
            item_identifier: &NSFileProviderItemIdentifier,
            options: NSFileProviderModifyItemOptions,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// Eviction
    unsafe impl NSFileProviderManager {
        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(evictItemWithIdentifier:completionHandler:)]
        pub unsafe fn evictItemWithIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// Barrier
    unsafe impl NSFileProviderManager {
        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(waitForChangesOnItemsBelowItemWithIdentifier:completionHandler:)]
        pub unsafe fn waitForChangesOnItemsBelowItemWithIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// Stabilization
    unsafe impl NSFileProviderManager {
        #[cfg(feature = "block2")]
        #[method(waitForStabilizationWithCompletionHandler:)]
        pub unsafe fn waitForStabilizationWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );
    }
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderManagerDisconnectionOptions(pub NSUInteger);
impl NSFileProviderManagerDisconnectionOptions {
    #[doc(alias = "NSFileProviderManagerDisconnectionOptionsTemporary")]
    pub const Temporary: Self = Self(1 << 0);
}

unsafe impl Encode for NSFileProviderManagerDisconnectionOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderManagerDisconnectionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// Disconnection
    unsafe impl NSFileProviderManager {
        #[cfg(feature = "block2")]
        #[method(disconnectWithReason:options:completionHandler:)]
        pub unsafe fn disconnectWithReason_options_completionHandler(
            &self,
            localized_reason: &NSString,
            options: NSFileProviderManagerDisconnectionOptions,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(reconnectWithCompletionHandler:)]
        pub unsafe fn reconnectWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// Materialize
    unsafe impl NSFileProviderManager {
        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(requestDownloadForItemWithIdentifier:requestedRange:completionHandler:)]
        pub unsafe fn requestDownloadForItemWithIdentifier_requestedRange_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            range_to_materialize: NSRange,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );
    }
);
