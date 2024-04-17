//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MKShape")]
    pub struct MKPointAnnotation;

    #[cfg(feature = "MKShape")]
    unsafe impl ClassType for MKPointAnnotation {
        #[inherits(NSObject)]
        type Super = MKShape;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "MKAnnotation", feature = "MKShape"))]
unsafe impl MKAnnotation for MKPointAnnotation {}

#[cfg(feature = "MKShape")]
unsafe impl NSObjectProtocol for MKPointAnnotation {}

extern_methods!(
    #[cfg(feature = "MKShape")]
    unsafe impl MKPointAnnotation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Init initWithCoordinate:)]
        pub unsafe fn initWithCoordinate(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
        ) -> Id<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Init initWithCoordinate:title:subtitle:)]
        pub unsafe fn initWithCoordinate_title_subtitle(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            title: Option<&NSString>,
            subtitle: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[cfg(feature = "objc2-core-location")]
        #[method(setCoordinate:)]
        pub unsafe fn setCoordinate(&self, coordinate: CLLocationCoordinate2D);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MKShape")]
    unsafe impl MKPointAnnotation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
