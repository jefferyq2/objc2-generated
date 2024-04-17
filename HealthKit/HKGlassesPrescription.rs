//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "HKObject",
        feature = "HKSample",
        feature = "HKVisionPrescription"
    ))]
    pub struct HKGlassesPrescription;

    #[cfg(all(
        feature = "HKObject",
        feature = "HKSample",
        feature = "HKVisionPrescription"
    ))]
    unsafe impl ClassType for HKGlassesPrescription {
        #[inherits(HKSample, HKObject, NSObject)]
        type Super = HKVisionPrescription;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl NSCoding for HKGlassesPrescription {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl NSCopying for HKGlassesPrescription {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl NSObjectProtocol for HKGlassesPrescription {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl NSSecureCoding for HKGlassesPrescription {}

extern_methods!(
    #[cfg(all(
        feature = "HKObject",
        feature = "HKSample",
        feature = "HKVisionPrescription"
    ))]
    unsafe impl HKGlassesPrescription {
        #[cfg(all(
            feature = "HKGlassesLensSpecification",
            feature = "HKLensSpecification"
        ))]
        #[method_id(@__retain_semantics Other rightEye)]
        pub unsafe fn rightEye(&self) -> Option<Id<HKGlassesLensSpecification>>;

        #[cfg(all(
            feature = "HKGlassesLensSpecification",
            feature = "HKLensSpecification"
        ))]
        #[method_id(@__retain_semantics Other leftEye)]
        pub unsafe fn leftEye(&self) -> Option<Id<HKGlassesLensSpecification>>;

        #[cfg(all(
            feature = "HKDevice",
            feature = "HKGlassesLensSpecification",
            feature = "HKLensSpecification"
        ))]
        #[method_id(@__retain_semantics Other prescriptionWithRightEyeSpecification:leftEyeSpecification:dateIssued:expirationDate:device:metadata:)]
        pub unsafe fn prescriptionWithRightEyeSpecification_leftEyeSpecification_dateIssued_expirationDate_device_metadata(
            right_eye_specification: Option<&HKGlassesLensSpecification>,
            left_eye_specification: Option<&HKGlassesLensSpecification>,
            date_issued: &NSDate,
            expiration_date: Option<&NSDate>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "HKDevice")]
        #[method_id(@__retain_semantics Other prescriptionWithType:dateIssued:expirationDate:device:metadata:)]
        pub unsafe fn prescriptionWithType_dateIssued_expirationDate_device_metadata(
            r#type: HKVisionPrescriptionType,
            date_issued: &NSDate,
            expiration_date: Option<&NSDate>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;
    }
);
