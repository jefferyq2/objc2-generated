//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKQueryDescriptor;

    unsafe impl ClassType for HKQueryDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for HKQueryDescriptor {}

unsafe impl NSCopying for HKQueryDescriptor {}

unsafe impl NSObjectProtocol for HKQueryDescriptor {}

unsafe impl NSSecureCoding for HKQueryDescriptor {}

extern_methods!(
    unsafe impl HKQueryDescriptor {
        #[cfg(feature = "HKObjectType")]
        #[method_id(@__retain_semantics Other sampleType)]
        pub unsafe fn sampleType(&self) -> Id<HKSampleType>;

        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Id<NSPredicate>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "HKObjectType")]
        #[method_id(@__retain_semantics Init initWithSampleType:predicate:)]
        pub unsafe fn initWithSampleType_predicate(
            this: Allocated<Self>,
            sample_type: &HKSampleType,
            predicate: Option<&NSPredicate>,
        ) -> Id<Self>;
    }
);
