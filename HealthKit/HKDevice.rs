//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKDevicePropertyKeyName: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKDevicePropertyKeyManufacturer: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKDevicePropertyKeyModel: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKDevicePropertyKeyHardwareVersion: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKDevicePropertyKeyFirmwareVersion: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKDevicePropertyKeySoftwareVersion: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKDevicePropertyKeyLocalIdentifier: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKDevicePropertyKeyUDIDeviceIdentifier: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKDevice")]
    pub struct HKDevice;

    #[cfg(feature = "HealthKit_HKDevice")]
    unsafe impl ClassType for HKDevice {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKDevice")]
unsafe impl NSCoding for HKDevice {}

#[cfg(feature = "HealthKit_HKDevice")]
unsafe impl NSCopying for HKDevice {}

#[cfg(feature = "HealthKit_HKDevice")]
unsafe impl NSObjectProtocol for HKDevice {}

#[cfg(feature = "HealthKit_HKDevice")]
unsafe impl NSSecureCoding for HKDevice {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKDevice")]
    unsafe impl HKDevice {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other manufacturer)]
        pub unsafe fn manufacturer(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other model)]
        pub unsafe fn model(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other hardwareVersion)]
        pub unsafe fn hardwareVersion(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other firmwareVersion)]
        pub unsafe fn firmwareVersion(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other softwareVersion)]
        pub unsafe fn softwareVersion(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localIdentifier)]
        pub unsafe fn localIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other UDIDeviceIdentifier)]
        pub unsafe fn UDIDeviceIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithName:manufacturer:model:hardwareVersion:firmwareVersion:softwareVersion:localIdentifier:UDIDeviceIdentifier:)]
        pub unsafe fn initWithName_manufacturer_model_hardwareVersion_firmwareVersion_softwareVersion_localIdentifier_UDIDeviceIdentifier(
            this: Allocated<Self>,
            name: Option<&NSString>,
            manufacturer: Option<&NSString>,
            model: Option<&NSString>,
            hardware_version: Option<&NSString>,
            firmware_version: Option<&NSString>,
            software_version: Option<&NSString>,
            local_identifier: Option<&NSString>,
            udi_device_identifier: Option<&NSString>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other localDevice)]
        pub unsafe fn localDevice() -> Id<HKDevice>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKDevice")]
    unsafe impl HKDevice {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
