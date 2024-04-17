//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHPersistentChange;

    unsafe impl ClassType for PHPersistentChange {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for PHPersistentChange {}

unsafe impl Sync for PHPersistentChange {}

unsafe impl NSObjectProtocol for PHPersistentChange {}

extern_methods!(
    unsafe impl PHPersistentChange {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "PHPersistentChangeToken")]
        #[method_id(@__retain_semantics Other changeToken)]
        pub unsafe fn changeToken(&self) -> Id<PHPersistentChangeToken>;

        #[cfg(all(feature = "PHPersistentObjectChangeDetails", feature = "PhotosTypes"))]
        #[method_id(@__retain_semantics Other changeDetailsForObjectType:error:_)]
        pub unsafe fn changeDetailsForObjectType_error(
            &self,
            object_type: PHObjectType,
        ) -> Result<Id<PHPersistentObjectChangeDetails>, Id<NSError>>;
    }
);
