//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "MKOverlayPathRenderer",
        feature = "MKOverlayRenderer",
        feature = "MKPolylineRenderer"
    ))]
    pub struct MKGradientPolylineRenderer;

    #[cfg(all(
        feature = "MKOverlayPathRenderer",
        feature = "MKOverlayRenderer",
        feature = "MKPolylineRenderer"
    ))]
    unsafe impl ClassType for MKGradientPolylineRenderer {
        #[inherits(MKOverlayPathRenderer, MKOverlayRenderer, NSObject)]
        type Super = MKPolylineRenderer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "MKOverlayPathRenderer",
    feature = "MKOverlayRenderer",
    feature = "MKPolylineRenderer"
))]
unsafe impl NSObjectProtocol for MKGradientPolylineRenderer {}

extern_methods!(
    #[cfg(all(
        feature = "MKOverlayPathRenderer",
        feature = "MKOverlayRenderer",
        feature = "MKPolylineRenderer"
    ))]
    unsafe impl MKGradientPolylineRenderer {
        #[method_id(@__retain_semantics Other locations)]
        pub unsafe fn locations(&self) -> Id<NSArray<NSNumber>>;

        #[cfg(feature = "objc2-app-kit")]
        #[method_id(@__retain_semantics Other colors)]
        pub unsafe fn colors(&self) -> Id<NSArray<NSColor>>;

        #[cfg(feature = "objc2-app-kit")]
        #[method(setColors:atLocations:)]
        pub unsafe fn setColors_atLocations(
            &self,
            colors: &NSArray<NSColor>,
            locations: &NSArray<NSNumber>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MKPolylineRenderer`
    #[cfg(all(
        feature = "MKOverlayPathRenderer",
        feature = "MKOverlayRenderer",
        feature = "MKPolylineRenderer"
    ))]
    unsafe impl MKGradientPolylineRenderer {
        #[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline", feature = "MKShape"))]
        #[method_id(@__retain_semantics Init initWithPolyline:)]
        pub unsafe fn initWithPolyline(this: Allocated<Self>, polyline: &MKPolyline) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(all(
        feature = "MKOverlayPathRenderer",
        feature = "MKOverlayRenderer",
        feature = "MKPolylineRenderer"
    ))]
    unsafe impl MKGradientPolylineRenderer {
        #[cfg(all(feature = "MKAnnotation", feature = "MKOverlay"))]
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Allocated<Self>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "MKOverlayPathRenderer",
        feature = "MKOverlayRenderer",
        feature = "MKPolylineRenderer"
    ))]
    unsafe impl MKGradientPolylineRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
