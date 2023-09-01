// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `DataDetection` framework

#[cfg_attr(feature = "apple", link(name = "DataDetection", kind = "framework"))]
extern "C" {}

#[path = "DDMatch.rs"]
mod __DDMatch;
#[path = "DataDetectionBase.rs"]
mod __DataDetectionBase;

#[cfg(feature = "DataDetection_DDMatch")]
pub use self::__DDMatch::DDMatch;
#[cfg(feature = "DataDetection_DDMatchCalendarEvent")]
pub use self::__DDMatch::DDMatchCalendarEvent;
#[cfg(feature = "DataDetection_DDMatchEmailAddress")]
pub use self::__DDMatch::DDMatchEmailAddress;
#[cfg(feature = "DataDetection_DDMatchFlightNumber")]
pub use self::__DDMatch::DDMatchFlightNumber;
#[cfg(feature = "DataDetection_DDMatchLink")]
pub use self::__DDMatch::DDMatchLink;
#[cfg(feature = "DataDetection_DDMatchMoneyAmount")]
pub use self::__DDMatch::DDMatchMoneyAmount;
#[cfg(feature = "DataDetection_DDMatchPhoneNumber")]
pub use self::__DDMatch::DDMatchPhoneNumber;
#[cfg(feature = "DataDetection_DDMatchPostalAddress")]
pub use self::__DDMatch::DDMatchPostalAddress;
#[cfg(feature = "DataDetection_DDMatchShipmentTrackingNumber")]
pub use self::__DDMatch::DDMatchShipmentTrackingNumber;
