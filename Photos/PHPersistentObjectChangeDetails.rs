//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHPersistentObjectChangeDetails;

    unsafe impl ClassType for PHPersistentObjectChangeDetails {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for PHPersistentObjectChangeDetails {}

unsafe impl Sync for PHPersistentObjectChangeDetails {}

unsafe impl NSObjectProtocol for PHPersistentObjectChangeDetails {}

extern_methods!(
    unsafe impl PHPersistentObjectChangeDetails {
        #[cfg(feature = "PhotosTypes")]
        #[method(objectType)]
        pub unsafe fn objectType(&self) -> PHObjectType;

        #[method_id(@__retain_semantics Other insertedLocalIdentifiers)]
        pub unsafe fn insertedLocalIdentifiers(&self) -> Id<NSSet<NSString>>;

        #[method_id(@__retain_semantics Other updatedLocalIdentifiers)]
        pub unsafe fn updatedLocalIdentifiers(&self) -> Id<NSSet<NSString>>;

        #[method_id(@__retain_semantics Other deletedLocalIdentifiers)]
        pub unsafe fn deletedLocalIdentifiers(&self) -> Id<NSSet<NSString>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
