//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MKOverlayRenderer")]
    pub struct MKOverlayPathRenderer;

    #[cfg(feature = "MKOverlayRenderer")]
    unsafe impl ClassType for MKOverlayPathRenderer {
        #[inherits(NSObject)]
        type Super = MKOverlayRenderer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MKOverlayRenderer")]
unsafe impl NSObjectProtocol for MKOverlayPathRenderer {}

extern_methods!(
    #[cfg(feature = "MKOverlayRenderer")]
    unsafe impl MKOverlayPathRenderer {
        #[cfg(feature = "objc2-app-kit")]
        #[method_id(@__retain_semantics Other fillColor)]
        pub unsafe fn fillColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "objc2-app-kit")]
        #[method(setFillColor:)]
        pub unsafe fn setFillColor(&self, fill_color: Option<&NSColor>);

        #[cfg(feature = "objc2-app-kit")]
        #[method_id(@__retain_semantics Other strokeColor)]
        pub unsafe fn strokeColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "objc2-app-kit")]
        #[method(setStrokeColor:)]
        pub unsafe fn setStrokeColor(&self, stroke_color: Option<&NSColor>);

        #[method(lineWidth)]
        pub unsafe fn lineWidth(&self) -> CGFloat;

        #[method(setLineWidth:)]
        pub unsafe fn setLineWidth(&self, line_width: CGFloat);

        #[method(miterLimit)]
        pub unsafe fn miterLimit(&self) -> CGFloat;

        #[method(setMiterLimit:)]
        pub unsafe fn setMiterLimit(&self, miter_limit: CGFloat);

        #[method(lineDashPhase)]
        pub unsafe fn lineDashPhase(&self) -> CGFloat;

        #[method(setLineDashPhase:)]
        pub unsafe fn setLineDashPhase(&self, line_dash_phase: CGFloat);

        #[method_id(@__retain_semantics Other lineDashPattern)]
        pub unsafe fn lineDashPattern(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[method(setLineDashPattern:)]
        pub unsafe fn setLineDashPattern(&self, line_dash_pattern: Option<&NSArray<NSNumber>>);

        #[method(shouldRasterize)]
        pub unsafe fn shouldRasterize(&self) -> bool;

        #[method(setShouldRasterize:)]
        pub unsafe fn setShouldRasterize(&self, should_rasterize: bool);

        #[method(createPath)]
        pub unsafe fn createPath(&self);

        #[method(invalidatePath)]
        pub unsafe fn invalidatePath(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(feature = "MKOverlayRenderer")]
    unsafe impl MKOverlayPathRenderer {
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
    #[cfg(feature = "MKOverlayRenderer")]
    unsafe impl MKOverlayPathRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
