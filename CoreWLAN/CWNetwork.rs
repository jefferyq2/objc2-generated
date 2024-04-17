//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CWNetwork;

    unsafe impl ClassType for CWNetwork {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CWNetwork {}

unsafe impl NSCopying for CWNetwork {}

unsafe impl NSObjectProtocol for CWNetwork {}

unsafe impl NSSecureCoding for CWNetwork {}

extern_methods!(
    unsafe impl CWNetwork {
        #[method_id(@__retain_semantics Other ssid)]
        pub unsafe fn ssid(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other ssidData)]
        pub unsafe fn ssidData(&self) -> Option<Id<NSData>>;

        #[method_id(@__retain_semantics Other bssid)]
        pub unsafe fn bssid(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "CWChannel")]
        #[method_id(@__retain_semantics Other wlanChannel)]
        pub unsafe fn wlanChannel(&self) -> Option<Id<CWChannel>>;

        #[method(rssiValue)]
        pub unsafe fn rssiValue(&self) -> NSInteger;

        #[method(noiseMeasurement)]
        pub unsafe fn noiseMeasurement(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other informationElementData)]
        pub unsafe fn informationElementData(&self) -> Option<Id<NSData>>;

        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Option<Id<NSString>>;

        #[method(beaconInterval)]
        pub unsafe fn beaconInterval(&self) -> NSInteger;

        #[method(ibss)]
        pub unsafe fn ibss(&self) -> bool;

        #[method(isEqualToNetwork:)]
        pub unsafe fn isEqualToNetwork(&self, network: &CWNetwork) -> bool;

        #[cfg(feature = "CoreWLANTypes")]
        #[method(supportsSecurity:)]
        pub unsafe fn supportsSecurity(&self, security: CWSecurity) -> bool;

        #[cfg(feature = "CoreWLANTypes")]
        #[method(supportsPHYMode:)]
        pub unsafe fn supportsPHYMode(&self, phy_mode: CWPHYMode) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CWNetwork {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
