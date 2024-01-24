//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKCorrelationQuery")]
    pub struct HKCorrelationQuery;

    #[cfg(feature = "HealthKit_HKCorrelationQuery")]
    unsafe impl ClassType for HKCorrelationQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKCorrelationQuery")]
unsafe impl NSObjectProtocol for HKCorrelationQuery {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKCorrelationQuery")]
    unsafe impl HKCorrelationQuery {
        #[cfg(feature = "HealthKit_HKCorrelationType")]
        #[method_id(@__retain_semantics Other correlationType)]
        pub unsafe fn correlationType(&self) -> Id<HKCorrelationType>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSPredicate",
            feature = "HealthKit_HKSampleType"
        ))]
        #[method_id(@__retain_semantics Other samplePredicates)]
        pub unsafe fn samplePredicates(
            &self,
        ) -> Option<Id<NSDictionary<HKSampleType, NSPredicate>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "HealthKit_HKCorrelation",
            feature = "HealthKit_HKCorrelationType",
            feature = "HealthKit_HKSampleType"
        ))]
        #[method_id(@__retain_semantics Init initWithType:predicate:samplePredicates:completion:)]
        pub unsafe fn initWithType_predicate_samplePredicates_completion(
            this: Allocated<Self>,
            correlation_type: &HKCorrelationType,
            predicate: Option<&NSPredicate>,
            sample_predicates: Option<&NSDictionary<HKSampleType, NSPredicate>>,
            completion: &Block<
                dyn Fn(NonNull<HKCorrelationQuery>, *mut NSArray<HKCorrelation>, *mut NSError),
            >,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HealthKit_HKCorrelationQuery")]
    unsafe impl HKCorrelationQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKCorrelationQuery")]
    unsafe impl HKCorrelationQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
