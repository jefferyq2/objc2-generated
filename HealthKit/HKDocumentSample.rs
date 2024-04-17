//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    pub struct HKDocumentSample;

    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl ClassType for HKDocumentSample {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSCoding for HKDocumentSample {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSObjectProtocol for HKDocumentSample {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSSecureCoding for HKDocumentSample {}

extern_methods!(
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKDocumentSample {
        #[cfg(feature = "HKObjectType")]
        #[method_id(@__retain_semantics Other documentType)]
        pub unsafe fn documentType(&self) -> Id<HKDocumentType>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKDocumentSample {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKDocumentSample {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
