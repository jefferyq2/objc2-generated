//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ClassKit::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait CLSDataStoreDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "ClassKit_CLSContext",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other createContextForIdentifier:parentContext:parentIdentifierPath:)]
        unsafe fn createContextForIdentifier_parentContext_parentIdentifierPath(
            &self,
            identifier: &NSString,
            parent_context: &CLSContext,
            parent_identifier_path: &NSArray<NSString>,
        ) -> Option<Id<CLSContext>>;
    }

    unsafe impl ProtocolType for dyn CLSDataStoreDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ClassKit_CLSDataStore")]
    pub struct CLSDataStore;

    #[cfg(feature = "ClassKit_CLSDataStore")]
    unsafe impl ClassType for CLSDataStore {
        type Super = NSObject;
    }
);

#[cfg(feature = "ClassKit_CLSDataStore")]
unsafe impl NSObjectProtocol for CLSDataStore {}

extern_methods!(
    #[cfg(feature = "ClassKit_CLSDataStore")]
    unsafe impl CLSDataStore {
        #[method_id(@__retain_semantics Other shared)]
        pub unsafe fn shared() -> Id<CLSDataStore>;

        #[cfg(feature = "ClassKit_CLSContext")]
        #[method_id(@__retain_semantics Other mainAppContext)]
        pub unsafe fn mainAppContext(&self) -> Id<CLSContext>;

        #[cfg(feature = "ClassKit_CLSContext")]
        #[method_id(@__retain_semantics Other activeContext)]
        pub unsafe fn activeContext(&self) -> Option<Id<CLSContext>>;

        #[cfg(feature = "ClassKit_CLSActivity")]
        #[method_id(@__retain_semantics Other runningActivity)]
        pub unsafe fn runningActivity(&self) -> Option<Id<CLSActivity>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn CLSDataStoreDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CLSDataStoreDelegate>>,
        );

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(saveWithCompletion:)]
        pub unsafe fn saveWithCompletion(&self, completion: Option<&Block<(*mut NSError,), ()>>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(completeAllAssignedActivitiesMatching:)]
        pub unsafe fn completeAllAssignedActivitiesMatching(
            &self,
            context_path: &NSArray<NSString>,
        );
    }
);

extern_methods!(
    /// Contexts
    #[cfg(feature = "ClassKit_CLSDataStore")]
    unsafe impl CLSDataStore {
        #[cfg(all(
            feature = "ClassKit_CLSContext",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate"
        ))]
        #[method(contextsMatchingPredicate:completion:)]
        pub unsafe fn contextsMatchingPredicate_completion(
            &self,
            predicate: &NSPredicate,
            completion: &Block<(NonNull<NSArray<CLSContext>>, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "ClassKit_CLSContext",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(contextsMatchingIdentifierPath:completion:)]
        pub unsafe fn contextsMatchingIdentifierPath_completion(
            &self,
            identifier_path: &NSArray<NSString>,
            completion: &Block<(NonNull<NSArray<CLSContext>>, *mut NSError), ()>,
        );

        #[cfg(feature = "ClassKit_CLSContext")]
        #[method(removeContext:)]
        pub unsafe fn removeContext(&self, context: &CLSContext);

        #[cfg(all(
            feature = "ClassKit_CLSActivity",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(fetchActivityForURL:completion:)]
        pub unsafe fn fetchActivityForURL_completion(
            &self,
            url: &NSURL,
            completion: &Block<(*mut CLSActivity, *mut NSError), ()>,
        );
    }
);
