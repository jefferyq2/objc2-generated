//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CARenderer;

    unsafe impl ClassType for CARenderer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CARenderer {}

extern_methods!(
    unsafe impl CARenderer {
        #[deprecated = "+rendererWithMTLTexture"]
        #[method_id(@__retain_semantics Other rendererWithCGLContext:options:)]
        pub unsafe fn rendererWithCGLContext_options(
            ctx: NonNull<c_void>,
            dict: Option<&NSDictionary>,
        ) -> Id<CARenderer>;

        #[cfg(feature = "CALayer")]
        #[method_id(@__retain_semantics Other layer)]
        pub fn layer(&self) -> Option<Id<CALayer>>;

        #[cfg(feature = "CALayer")]
        #[method(setLayer:)]
        pub fn setLayer(&self, layer: Option<&CALayer>);

        #[method(bounds)]
        pub fn bounds(&self) -> CGRect;

        #[method(setBounds:)]
        pub fn setBounds(&self, bounds: CGRect);

        #[method(updateBounds)]
        pub fn updateBounds(&self) -> CGRect;

        #[method(addUpdateRect:)]
        pub fn addUpdateRect(&self, r: CGRect);

        #[method(render)]
        pub fn render(&self);

        #[method(nextFrameTime)]
        pub fn nextFrameTime(&self) -> CFTimeInterval;

        #[method(endFrame)]
        pub fn endFrame(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CARenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    pub static kCARendererColorSpace: &'static NSString;
}

extern "C" {
    pub static kCARendererMetalCommandQueue: &'static NSString;
}
