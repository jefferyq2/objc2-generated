//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKElectrocardiogramLead {
        #[doc(alias = "HKElectrocardiogramLeadAppleWatchSimilarToLeadI")]
        AppleWatchSimilarToLeadI = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKElectrocardiogramClassification {
        #[doc(alias = "HKElectrocardiogramClassificationNotSet")]
        NotSet = 0,
        #[doc(alias = "HKElectrocardiogramClassificationSinusRhythm")]
        SinusRhythm = 1,
        #[doc(alias = "HKElectrocardiogramClassificationAtrialFibrillation")]
        AtrialFibrillation = 2,
        #[doc(alias = "HKElectrocardiogramClassificationInconclusiveLowHeartRate")]
        InconclusiveLowHeartRate = 3,
        #[doc(alias = "HKElectrocardiogramClassificationInconclusiveHighHeartRate")]
        InconclusiveHighHeartRate = 4,
        #[doc(alias = "HKElectrocardiogramClassificationInconclusivePoorReading")]
        InconclusivePoorReading = 5,
        #[doc(alias = "HKElectrocardiogramClassificationInconclusiveOther")]
        InconclusiveOther = 6,
        #[doc(alias = "HKElectrocardiogramClassificationUnrecognized")]
        Unrecognized = 100,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKElectrocardiogramSymptomsStatus {
        #[doc(alias = "HKElectrocardiogramSymptomsStatusNotSet")]
        NotSet = 0,
        #[doc(alias = "HKElectrocardiogramSymptomsStatusNone")]
        None = 1,
        #[doc(alias = "HKElectrocardiogramSymptomsStatusPresent")]
        Present = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKElectrocardiogram")]
    pub struct HKElectrocardiogram;

    #[cfg(feature = "HealthKit_HKElectrocardiogram")]
    unsafe impl ClassType for HKElectrocardiogram {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKElectrocardiogram")]
unsafe impl NSCoding for HKElectrocardiogram {}

#[cfg(feature = "HealthKit_HKElectrocardiogram")]
unsafe impl NSObjectProtocol for HKElectrocardiogram {}

#[cfg(feature = "HealthKit_HKElectrocardiogram")]
unsafe impl NSSecureCoding for HKElectrocardiogram {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKElectrocardiogram")]
    unsafe impl HKElectrocardiogram {
        #[method(numberOfVoltageMeasurements)]
        pub unsafe fn numberOfVoltageMeasurements(&self) -> NSInteger;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other samplingFrequency)]
        pub unsafe fn samplingFrequency(&self) -> Option<Id<HKQuantity>>;

        #[method(classification)]
        pub unsafe fn classification(&self) -> HKElectrocardiogramClassification;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other averageHeartRate)]
        pub unsafe fn averageHeartRate(&self) -> Option<Id<HKQuantity>>;

        #[method(symptomsStatus)]
        pub unsafe fn symptomsStatus(&self) -> HKElectrocardiogramSymptomsStatus;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(feature = "HealthKit_HKElectrocardiogram")]
    unsafe impl HKElectrocardiogram {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKElectrocardiogram")]
    unsafe impl HKElectrocardiogram {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKPredicateKeyPathAverageHeartRate: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKPredicateKeyPathECGClassification: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKPredicateKeyPathECGSymptomsStatus: &'static NSString);
