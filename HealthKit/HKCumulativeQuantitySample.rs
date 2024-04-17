//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "HKObject",
        feature = "HKQuantitySample",
        feature = "HKSample"
    ))]
    pub struct HKCumulativeQuantitySample;

    #[cfg(all(
        feature = "HKObject",
        feature = "HKQuantitySample",
        feature = "HKSample"
    ))]
    unsafe impl ClassType for HKCumulativeQuantitySample {
        #[inherits(HKSample, HKObject, NSObject)]
        type Super = HKQuantitySample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "HKObject",
    feature = "HKQuantitySample",
    feature = "HKSample"
))]
unsafe impl NSCoding for HKCumulativeQuantitySample {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKQuantitySample",
    feature = "HKSample"
))]
unsafe impl NSObjectProtocol for HKCumulativeQuantitySample {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKQuantitySample",
    feature = "HKSample"
))]
unsafe impl NSSecureCoding for HKCumulativeQuantitySample {}

extern_methods!(
    #[cfg(all(
        feature = "HKObject",
        feature = "HKQuantitySample",
        feature = "HKSample"
    ))]
    unsafe impl HKCumulativeQuantitySample {
        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other sumQuantity)]
        pub unsafe fn sumQuantity(&self) -> Id<HKQuantity>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuantitySample`
    #[cfg(all(
        feature = "HKObject",
        feature = "HKQuantitySample",
        feature = "HKSample"
    ))]
    unsafe impl HKCumulativeQuantitySample {
        #[cfg(all(feature = "HKObjectType", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other quantitySampleWithType:quantity:startDate:endDate:)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
        ) -> Id<Self>;

        #[cfg(all(feature = "HKObjectType", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other quantitySampleWithType:quantity:startDate:endDate:metadata:)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate_metadata(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "HKDevice", feature = "HKObjectType", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other quantitySampleWithType:quantity:startDate:endDate:device:metadata:)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate_device_metadata(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(all(
        feature = "HKObject",
        feature = "HKQuantitySample",
        feature = "HKSample"
    ))]
    unsafe impl HKCumulativeQuantitySample {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "HKObject",
        feature = "HKQuantitySample",
        feature = "HKSample"
    ))]
    unsafe impl HKCumulativeQuantitySample {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    pub static HKPredicateKeyPathSum: &'static NSString;
}
