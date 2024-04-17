//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNContactProperty;

    unsafe impl ClassType for CNContactProperty {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CNContactProperty {}

unsafe impl NSCopying for CNContactProperty {}

unsafe impl NSObjectProtocol for CNContactProperty {}

unsafe impl NSSecureCoding for CNContactProperty {}

extern_methods!(
    unsafe impl CNContactProperty {
        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other contact)]
        pub unsafe fn contact(&self) -> Id<CNContact>;

        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNContactProperty {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
