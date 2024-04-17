//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    #[cfg(feature = "objc2-core-location")]
    pub static MKPointsOfInterestRequestMaxRadius: CLLocationDistance;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKLocalPointsOfInterestRequest;

    unsafe impl ClassType for MKLocalPointsOfInterestRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MKLocalPointsOfInterestRequest {}

unsafe impl NSObjectProtocol for MKLocalPointsOfInterestRequest {}

extern_methods!(
    unsafe impl MKLocalPointsOfInterestRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Init initWithCenterCoordinate:radius:)]
        pub unsafe fn initWithCenterCoordinate_radius(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            radius: CLLocationDistance,
        ) -> Id<Self>;

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method_id(@__retain_semantics Init initWithCoordinateRegion:)]
        pub unsafe fn initWithCoordinateRegion(
            this: Allocated<Self>,
            region: MKCoordinateRegion,
        ) -> Id<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[cfg(feature = "objc2-core-location")]
        #[method(radius)]
        pub unsafe fn radius(&self) -> CLLocationDistance;

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Id<MKPointOfInterestFilter>>;

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKLocalPointsOfInterestRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
