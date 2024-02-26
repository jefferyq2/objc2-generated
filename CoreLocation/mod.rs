// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `CoreLocation` framework
#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "CoreLocation", kind = "framework")]
extern "C" {}

#[path = "CLAvailability.rs"]
mod __CLAvailability;
#[path = "CLBackgroundActivitySession.rs"]
mod __CLBackgroundActivitySession;
#[path = "CLBeaconIdentityCondition.rs"]
mod __CLBeaconIdentityCondition;
#[path = "CLBeaconIdentityConstraint.rs"]
mod __CLBeaconIdentityConstraint;
#[path = "CLBeaconRegion.rs"]
mod __CLBeaconRegion;
#[path = "CLCircularGeographicCondition.rs"]
mod __CLCircularGeographicCondition;
#[path = "CLCircularRegion.rs"]
mod __CLCircularRegion;
#[path = "CLCondition.rs"]
mod __CLCondition;
#[path = "CLError.rs"]
mod __CLError;
#[path = "CLErrorDomain.rs"]
mod __CLErrorDomain;
#[path = "CLGeocoder.rs"]
mod __CLGeocoder;
#[path = "CLHeading.rs"]
mod __CLHeading;
#[path = "CLLocation.rs"]
mod __CLLocation;
#[path = "CLLocationManager.rs"]
mod __CLLocationManager;
#[path = "CLLocationManagerDelegate.rs"]
mod __CLLocationManagerDelegate;
#[path = "CLLocationManager_CLVisitExtensions.rs"]
mod __CLLocationManager_CLVisitExtensions;
#[path = "CLLocationPushServiceError.rs"]
mod __CLLocationPushServiceError;
#[path = "CLLocationPushServiceExtension.rs"]
mod __CLLocationPushServiceExtension;
#[path = "CLLocationUpdater.rs"]
mod __CLLocationUpdater;
#[path = "CLMonitor.rs"]
mod __CLMonitor;
#[path = "CLMonitorConfiguration.rs"]
mod __CLMonitorConfiguration;
#[path = "CLMonitoringEvent.rs"]
mod __CLMonitoringEvent;
#[path = "CLMonitoringRecord.rs"]
mod __CLMonitoringRecord;
#[path = "CLPlacemark.rs"]
mod __CLPlacemark;
#[path = "CLRegion.rs"]
mod __CLRegion;
#[path = "CLVisit.rs"]
mod __CLVisit;

#[cfg(feature = "CoreLocation_CLBackgroundActivitySession")]
pub use self::__CLBackgroundActivitySession::CLBackgroundActivitySession;
#[cfg(feature = "CoreLocation_CLBeaconIdentityCondition")]
pub use self::__CLBeaconIdentityCondition::CLBeaconIdentityCondition;
pub use self::__CLBeaconIdentityCondition::CLBeaconMajorValue;
pub use self::__CLBeaconIdentityCondition::CLBeaconMinorValue;
#[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
pub use self::__CLBeaconIdentityConstraint::CLBeaconIdentityConstraint;
#[cfg(feature = "CoreLocation_CLBeacon")]
pub use self::__CLBeaconRegion::CLBeacon;
#[cfg(feature = "CoreLocation_CLBeaconRegion")]
pub use self::__CLBeaconRegion::CLBeaconRegion;
#[cfg(feature = "CoreLocation_CLCircularGeographicCondition")]
pub use self::__CLCircularGeographicCondition::CLCircularGeographicCondition;
#[cfg(feature = "CoreLocation_CLCircularRegion")]
pub use self::__CLCircularRegion::CLCircularRegion;
#[cfg(feature = "CoreLocation_CLCondition")]
pub use self::__CLCondition::CLCondition;
#[cfg(feature = "Foundation_NSString")]
pub use self::__CLError::kCLErrorUserInfoAlternateRegionKey;
pub use self::__CLError::CLError;
#[cfg(feature = "Foundation_NSString")]
pub use self::__CLErrorDomain::kCLErrorDomain;
#[cfg(all(
    feature = "CoreLocation_CLPlacemark",
    feature = "Foundation_NSArray",
    feature = "Foundation_NSError"
))]
pub use self::__CLGeocoder::CLGeocodeCompletionHandler;
#[cfg(feature = "CoreLocation_CLGeocoder")]
pub use self::__CLGeocoder::CLGeocoder;
pub use self::__CLHeading::kCLHeadingFilterNone;
#[cfg(feature = "CoreLocation_CLHeading")]
pub use self::__CLHeading::CLHeading;
pub use self::__CLHeading::CLHeadingComponentValue;
pub use self::__CLLocation::kCLDistanceFilterNone;
pub use self::__CLLocation::kCLLocationAccuracyBest;
pub use self::__CLLocation::kCLLocationAccuracyBestForNavigation;
pub use self::__CLLocation::kCLLocationAccuracyHundredMeters;
pub use self::__CLLocation::kCLLocationAccuracyKilometer;
pub use self::__CLLocation::kCLLocationAccuracyNearestTenMeters;
pub use self::__CLLocation::kCLLocationAccuracyReduced;
pub use self::__CLLocation::kCLLocationAccuracyThreeKilometers;
pub use self::__CLLocation::kCLLocationCoordinate2DInvalid;
#[cfg(feature = "CoreLocation_CLFloor")]
pub use self::__CLLocation::CLFloor;
#[cfg(feature = "CoreLocation_CLLocation")]
pub use self::__CLLocation::CLLocation;
pub use self::__CLLocation::CLLocationAccuracy;
pub use self::__CLLocation::CLLocationCoordinate2D;
pub use self::__CLLocation::CLLocationCoordinate2DIsValid;
pub use self::__CLLocation::CLLocationCoordinate2DMake;
pub use self::__CLLocation::CLLocationDegrees;
pub use self::__CLLocation::CLLocationDirection;
pub use self::__CLLocation::CLLocationDirectionAccuracy;
pub use self::__CLLocation::CLLocationDistance;
pub use self::__CLLocation::CLLocationDistanceMax;
#[cfg(feature = "CoreLocation_CLLocationSourceInformation")]
pub use self::__CLLocation::CLLocationSourceInformation;
pub use self::__CLLocation::CLLocationSpeed;
pub use self::__CLLocation::CLLocationSpeedAccuracy;
pub use self::__CLLocation::CLTimeIntervalMax;
pub use self::__CLLocationManager::CLAccuracyAuthorization;
pub use self::__CLLocationManager::CLActivityType;
pub use self::__CLLocationManager::CLAuthorizationStatus;
pub use self::__CLLocationManager::CLDeviceOrientation;
#[cfg(feature = "CoreLocation_CLLocationManager")]
pub use self::__CLLocationManager::CLLocationManager;
pub use self::__CLLocationManagerDelegate::CLLocationManagerDelegate;
pub use self::__CLLocationPushServiceError::CLLocationPushServiceError;
#[cfg(feature = "Foundation_NSString")]
pub use self::__CLLocationPushServiceError::CLLocationPushServiceErrorDomain;
pub use self::__CLLocationPushServiceExtension::CLLocationPushServiceExtension;
pub use self::__CLLocationUpdater::CLLiveUpdateConfiguration;
#[cfg(feature = "CoreLocation_CLLocationUpdater")]
pub use self::__CLLocationUpdater::CLLocationUpdater;
#[cfg(feature = "CoreLocation_CLUpdate")]
pub use self::__CLLocationUpdater::CLUpdate;
#[cfg(feature = "CoreLocation_CLMonitor")]
pub use self::__CLMonitor::CLMonitor;
#[cfg(feature = "CoreLocation_CLMonitorConfiguration")]
pub use self::__CLMonitorConfiguration::CLMonitorConfiguration;
#[cfg(feature = "CoreLocation_CLMonitoringEvent")]
pub use self::__CLMonitoringEvent::CLMonitoringEvent;
pub use self::__CLMonitoringEvent::CLMonitoringState;
#[cfg(feature = "CoreLocation_CLMonitoringRecord")]
pub use self::__CLMonitoringRecord::CLMonitoringRecord;
#[cfg(feature = "CoreLocation_CLPlacemark")]
pub use self::__CLPlacemark::CLPlacemark;
pub use self::__CLRegion::CLProximity;
#[cfg(feature = "CoreLocation_CLRegion")]
pub use self::__CLRegion::CLRegion;
pub use self::__CLRegion::CLRegionState;
#[cfg(feature = "CoreLocation_CLVisit")]
pub use self::__CLVisit::CLVisit;
