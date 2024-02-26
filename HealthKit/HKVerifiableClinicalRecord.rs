//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

#[cfg(feature = "Foundation_NSString")]
typed_enum!(
    pub type HKVerifiableClinicalRecordSourceType = NSString;
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKVerifiableClinicalRecordSourceTypeSMARTHealthCard: &'static HKVerifiableClinicalRecordSourceType);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKVerifiableClinicalRecordSourceTypeEUDigitalCOVIDCertificate: &'static HKVerifiableClinicalRecordSourceType);

#[cfg(feature = "Foundation_NSString")]
typed_enum!(
    pub type HKVerifiableClinicalRecordCredentialType = NSString;
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKVerifiableClinicalRecordCredentialTypeCOVID19: &'static HKVerifiableClinicalRecordCredentialType);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKVerifiableClinicalRecordCredentialTypeImmunization: &'static HKVerifiableClinicalRecordCredentialType);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKVerifiableClinicalRecordCredentialTypeLaboratory: &'static HKVerifiableClinicalRecordCredentialType);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKVerifiableClinicalRecordCredentialTypeRecovery: &'static HKVerifiableClinicalRecordCredentialType);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKVerifiableClinicalRecord")]
    pub struct HKVerifiableClinicalRecord;

    #[cfg(feature = "HealthKit_HKVerifiableClinicalRecord")]
    unsafe impl ClassType for HKVerifiableClinicalRecord {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKVerifiableClinicalRecord")]
unsafe impl NSCoding for HKVerifiableClinicalRecord {}

#[cfg(feature = "HealthKit_HKVerifiableClinicalRecord")]
unsafe impl NSObjectProtocol for HKVerifiableClinicalRecord {}

#[cfg(feature = "HealthKit_HKVerifiableClinicalRecord")]
unsafe impl NSSecureCoding for HKVerifiableClinicalRecord {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKVerifiableClinicalRecord")]
    unsafe impl HKVerifiableClinicalRecord {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other recordTypes)]
        pub unsafe fn recordTypes(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other issuerIdentifier)]
        pub unsafe fn issuerIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "HealthKit_HKVerifiableClinicalRecordSubject")]
        #[method_id(@__retain_semantics Other subject)]
        pub unsafe fn subject(&self) -> Id<HKVerifiableClinicalRecordSubject>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other issuedDate)]
        pub unsafe fn issuedDate(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other relevantDate)]
        pub unsafe fn relevantDate(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other expirationDate)]
        pub unsafe fn expirationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other itemNames)]
        pub unsafe fn itemNames(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sourceType)]
        pub unsafe fn sourceType(&self) -> Option<Id<HKVerifiableClinicalRecordSourceType>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other dataRepresentation)]
        pub unsafe fn dataRepresentation(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated]
        #[method_id(@__retain_semantics Other JWSRepresentation)]
        pub unsafe fn JWSRepresentation(&self) -> Id<NSData>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
