//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_static!(MKMapCameraZoomDefault: CLLocationDistance);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
    pub struct MKMapCameraZoomRange;

    #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
    unsafe impl ClassType for MKMapCameraZoomRange {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKMapCameraZoomRange")]
unsafe impl NSCoding for MKMapCameraZoomRange {}

#[cfg(feature = "MapKit_MKMapCameraZoomRange")]
unsafe impl NSObjectProtocol for MKMapCameraZoomRange {}

#[cfg(feature = "MapKit_MKMapCameraZoomRange")]
unsafe impl NSSecureCoding for MKMapCameraZoomRange {}

extern_methods!(
    #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
    unsafe impl MKMapCameraZoomRange {
        #[method_id(@__retain_semantics Init initWithMinCenterCoordinateDistance:maxCenterCoordinateDistance:)]
        pub unsafe fn initWithMinCenterCoordinateDistance_maxCenterCoordinateDistance(
            this: Option<Allocated<Self>>,
            min_distance: CLLocationDistance,
            max_distance: CLLocationDistance,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithMinCenterCoordinateDistance:)]
        pub unsafe fn initWithMinCenterCoordinateDistance(
            this: Option<Allocated<Self>>,
            min_distance: CLLocationDistance,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithMaxCenterCoordinateDistance:)]
        pub unsafe fn initWithMaxCenterCoordinateDistance(
            this: Option<Allocated<Self>>,
            max_distance: CLLocationDistance,
        ) -> Option<Id<Self>>;

        #[method(minCenterCoordinateDistance)]
        pub unsafe fn minCenterCoordinateDistance(&self) -> CLLocationDistance;

        #[method(maxCenterCoordinateDistance)]
        pub unsafe fn maxCenterCoordinateDistance(&self) -> CLLocationDistance;
    }
);
