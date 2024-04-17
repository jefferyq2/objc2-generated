//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(all(feature = "MKMapSnapshot", feature = "block2"))]
pub type MKMapSnapshotCompletionHandler = *mut Block<dyn Fn(*mut MKMapSnapshot, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKMapSnapshotter;

    unsafe impl ClassType for MKMapSnapshotter {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MKMapSnapshotter {}

extern_methods!(
    unsafe impl MKMapSnapshotter {
        #[cfg(feature = "MKMapSnapshotOptions")]
        #[method_id(@__retain_semantics Init initWithOptions:)]
        pub unsafe fn initWithOptions(
            this: Allocated<Self>,
            options: &MKMapSnapshotOptions,
        ) -> Id<Self>;

        #[cfg(all(feature = "MKMapSnapshot", feature = "block2"))]
        #[method(startWithCompletionHandler:)]
        pub unsafe fn startWithCompletionHandler(
            &self,
            completion_handler: MKMapSnapshotCompletionHandler,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(isLoading)]
        pub unsafe fn isLoading(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKMapSnapshotter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
