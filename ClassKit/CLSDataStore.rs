//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait CLSDataStoreDelegate: NSObjectProtocol {
        #[cfg(all(feature = "CLSContext", feature = "CLSObject"))]
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
    pub struct CLSDataStore;

    unsafe impl ClassType for CLSDataStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CLSDataStore {}

extern_methods!(
    unsafe impl CLSDataStore {
        #[method_id(@__retain_semantics Other shared)]
        pub unsafe fn shared() -> Id<CLSDataStore>;

        #[cfg(all(feature = "CLSContext", feature = "CLSObject"))]
        #[method_id(@__retain_semantics Other mainAppContext)]
        pub unsafe fn mainAppContext(&self) -> Id<CLSContext>;

        #[cfg(all(feature = "CLSContext", feature = "CLSObject"))]
        #[method_id(@__retain_semantics Other activeContext)]
        pub unsafe fn activeContext(&self) -> Option<Id<CLSContext>>;

        #[cfg(all(feature = "CLSActivity", feature = "CLSObject"))]
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
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "block2")]
        #[method(saveWithCompletion:)]
        pub unsafe fn saveWithCompletion(&self, completion: Option<&Block<dyn Fn(*mut NSError)>>);

        #[method(completeAllAssignedActivitiesMatching:)]
        pub unsafe fn completeAllAssignedActivitiesMatching(
            &self,
            context_path: &NSArray<NSString>,
        );
    }
);

extern_methods!(
    /// Contexts
    unsafe impl CLSDataStore {
        #[cfg(all(feature = "CLSContext", feature = "CLSObject", feature = "block2"))]
        #[method(contextsMatchingPredicate:completion:)]
        pub unsafe fn contextsMatchingPredicate_completion(
            &self,
            predicate: &NSPredicate,
            completion: &Block<dyn Fn(NonNull<NSArray<CLSContext>>, *mut NSError)>,
        );

        #[cfg(all(feature = "CLSContext", feature = "CLSObject", feature = "block2"))]
        #[method(contextsMatchingIdentifierPath:completion:)]
        pub unsafe fn contextsMatchingIdentifierPath_completion(
            &self,
            identifier_path: &NSArray<NSString>,
            completion: &Block<dyn Fn(NonNull<NSArray<CLSContext>>, *mut NSError)>,
        );

        #[cfg(all(feature = "CLSContext", feature = "CLSObject"))]
        #[method(removeContext:)]
        pub unsafe fn removeContext(&self, context: &CLSContext);

        #[cfg(all(feature = "CLSActivity", feature = "CLSObject", feature = "block2"))]
        #[method(fetchActivityForURL:completion:)]
        pub unsafe fn fetchActivityForURL_completion(
            &self,
            url: &NSURL,
            completion: &Block<dyn Fn(*mut CLSActivity, *mut NSError)>,
        );
    }
);
