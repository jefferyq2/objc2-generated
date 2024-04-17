//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    pub struct HKAudiogramSample;

    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl ClassType for HKAudiogramSample {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSCoding for HKAudiogramSample {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSObjectProtocol for HKAudiogramSample {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSSecureCoding for HKAudiogramSample {}

extern_methods!(
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKAudiogramSample {
        #[method_id(@__retain_semantics Other sensitivityPoints)]
        pub unsafe fn sensitivityPoints(&self) -> Id<NSArray<HKAudiogramSensitivityPoint>>;

        #[method_id(@__retain_semantics Other audiogramSampleWithSensitivityPoints:startDate:endDate:metadata:)]
        pub unsafe fn audiogramSampleWithSensitivityPoints_startDate_endDate_metadata(
            sensitivity_points: &NSArray<HKAudiogramSensitivityPoint>,
            start_date: &NSDate,
            end_date: &NSDate,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKAudiogramSample {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKAudiogramSample {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKAudiogramSensitivityPoint;

    unsafe impl ClassType for HKAudiogramSensitivityPoint {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for HKAudiogramSensitivityPoint {}

extern_methods!(
    unsafe impl HKAudiogramSensitivityPoint {
        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other frequency)]
        pub unsafe fn frequency(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other leftEarSensitivity)]
        pub unsafe fn leftEarSensitivity(&self) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other rightEarSensitivity)]
        pub unsafe fn rightEarSensitivity(&self) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other sensitivityPointWithFrequency:leftEarSensitivity:rightEarSensitivity:error:_)]
        pub unsafe fn sensitivityPointWithFrequency_leftEarSensitivity_rightEarSensitivity_error(
            frequency: &HKQuantity,
            left_ear_sensitivity: Option<&HKQuantity>,
            right_ear_sensitivity: Option<&HKQuantity>,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKAudiogramSensitivityPoint {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
