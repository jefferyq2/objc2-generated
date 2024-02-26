//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Accessibility::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum AXHearingDeviceEar {
        #[doc(alias = "AXHearingDeviceEarNone")]
        None = 0,
        #[doc(alias = "AXHearingDeviceEarLeft")]
        Left = 1 << 1,
        #[doc(alias = "AXHearingDeviceEarRight")]
        Right = 1 << 2,
        #[doc(alias = "AXHearingDeviceEarBoth")]
        Both = AXHearingDeviceEar::Left.0 | AXHearingDeviceEar::Right.0,
    }
);

extern_fn!(
    pub unsafe fn AXMFiHearingDeviceStreamingEar() -> AXHearingDeviceEar;
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(AXMFiHearingDeviceStreamingEarDidChangeNotification: &'static NSNotificationName);

extern_fn!(
    pub unsafe fn AXSupportsBidirectionalAXMFiHearingDeviceStreaming() -> Bool;
);

extern_fn!(
    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSUUID"))]
    pub unsafe fn AXMFiHearingDevicePairedUUIDs() -> NonNull<NSArray<NSUUID>>;
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(AXMFiHearingDevicePairedUUIDsDidChangeNotification: &'static NSNotificationName);
