//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAMetalDisplayLinkUpdate;

    unsafe impl ClassType for CAMetalDisplayLinkUpdate {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CAMetalDisplayLinkUpdate {}

extern_methods!(
    unsafe impl CAMetalDisplayLinkUpdate {
        #[method(targetTimestamp)]
        pub unsafe fn targetTimestamp(&self) -> CFTimeInterval;

        #[method(targetPresentationTimestamp)]
        pub unsafe fn targetPresentationTimestamp(&self) -> CFTimeInterval;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAMetalDisplayLinkUpdate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait CAMetalDisplayLinkDelegate {
        #[method(metalDisplayLink:needsUpdate:)]
        unsafe fn metalDisplayLink_needsUpdate(
            &self,
            link: &CAMetalDisplayLink,
            update: &CAMetalDisplayLinkUpdate,
        );
    }

    unsafe impl ProtocolType for dyn CAMetalDisplayLinkDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAMetalDisplayLink;

    unsafe impl ClassType for CAMetalDisplayLink {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CAMetalDisplayLink {}

extern_methods!(
    unsafe impl CAMetalDisplayLink {
        #[method(addToRunLoop:forMode:)]
        pub unsafe fn addToRunLoop_forMode(&self, runloop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(&self, runloop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Id<ProtocolObject<dyn CAMetalDisplayLinkDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CAMetalDisplayLinkDelegate>>,
        );

        #[method(preferredFrameLatency)]
        pub unsafe fn preferredFrameLatency(&self) -> c_float;

        #[method(setPreferredFrameLatency:)]
        pub unsafe fn setPreferredFrameLatency(&self, preferred_frame_latency: c_float);

        #[cfg(feature = "CAFrameRateRange")]
        #[method(preferredFrameRateRange)]
        pub unsafe fn preferredFrameRateRange(&self) -> CAFrameRateRange;

        #[cfg(feature = "CAFrameRateRange")]
        #[method(setPreferredFrameRateRange:)]
        pub unsafe fn setPreferredFrameRateRange(
            &self,
            preferred_frame_rate_range: CAFrameRateRange,
        );

        #[method(isPaused)]
        pub unsafe fn isPaused(&self) -> bool;

        #[method(setPaused:)]
        pub unsafe fn setPaused(&self, paused: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAMetalDisplayLink {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
