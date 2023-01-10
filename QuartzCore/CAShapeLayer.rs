//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CAShapeLayerFillRule = NSString;
);

typed_enum!(
    pub type CAShapeLayerLineJoin = NSString;
);

typed_enum!(
    pub type CAShapeLayerLineCap = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAShapeLayer;

    unsafe impl ClassType for CAShapeLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
    }
);

extern_methods!(
    #[cfg(feature = "QuartzCore_CAShapeLayer")]
    unsafe impl CAShapeLayer {
        #[cfg(feature = "QuartzCore_CAShapeLayerFillRule")]
        #[method_id(@__retain_semantics Other fillRule)]
        pub unsafe fn fillRule(&self) -> Id<CAShapeLayerFillRule, Shared>;

        #[cfg(feature = "QuartzCore_CAShapeLayerFillRule")]
        #[method(setFillRule:)]
        pub unsafe fn setFillRule(&self, fillRule: &CAShapeLayerFillRule);

        #[method(strokeStart)]
        pub unsafe fn strokeStart(&self) -> CGFloat;

        #[method(setStrokeStart:)]
        pub unsafe fn setStrokeStart(&self, strokeStart: CGFloat);

        #[method(strokeEnd)]
        pub unsafe fn strokeEnd(&self) -> CGFloat;

        #[method(setStrokeEnd:)]
        pub unsafe fn setStrokeEnd(&self, strokeEnd: CGFloat);

        #[method(lineWidth)]
        pub unsafe fn lineWidth(&self) -> CGFloat;

        #[method(setLineWidth:)]
        pub unsafe fn setLineWidth(&self, lineWidth: CGFloat);

        #[method(miterLimit)]
        pub unsafe fn miterLimit(&self) -> CGFloat;

        #[method(setMiterLimit:)]
        pub unsafe fn setMiterLimit(&self, miterLimit: CGFloat);

        #[cfg(feature = "QuartzCore_CAShapeLayerLineCap")]
        #[method_id(@__retain_semantics Other lineCap)]
        pub unsafe fn lineCap(&self) -> Id<CAShapeLayerLineCap, Shared>;

        #[cfg(feature = "QuartzCore_CAShapeLayerLineCap")]
        #[method(setLineCap:)]
        pub unsafe fn setLineCap(&self, lineCap: &CAShapeLayerLineCap);

        #[cfg(feature = "QuartzCore_CAShapeLayerLineJoin")]
        #[method_id(@__retain_semantics Other lineJoin)]
        pub unsafe fn lineJoin(&self) -> Id<CAShapeLayerLineJoin, Shared>;

        #[cfg(feature = "QuartzCore_CAShapeLayerLineJoin")]
        #[method(setLineJoin:)]
        pub unsafe fn setLineJoin(&self, lineJoin: &CAShapeLayerLineJoin);

        #[method(lineDashPhase)]
        pub unsafe fn lineDashPhase(&self) -> CGFloat;

        #[method(setLineDashPhase:)]
        pub unsafe fn setLineDashPhase(&self, lineDashPhase: CGFloat);

        #[method_id(@__retain_semantics Other lineDashPattern)]
        pub unsafe fn lineDashPattern(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;

        #[method(setLineDashPattern:)]
        pub unsafe fn setLineDashPattern(&self, lineDashPattern: Option<&NSArray<NSNumber>>);
    }
);

extern_static!(kCAFillRuleNonZero: &'static CAShapeLayerFillRule);

extern_static!(kCAFillRuleEvenOdd: &'static CAShapeLayerFillRule);

extern_static!(kCALineJoinMiter: &'static CAShapeLayerLineJoin);

extern_static!(kCALineJoinRound: &'static CAShapeLayerLineJoin);

extern_static!(kCALineJoinBevel: &'static CAShapeLayerLineJoin);

extern_static!(kCALineCapButt: &'static CAShapeLayerLineCap);

extern_static!(kCALineCapRound: &'static CAShapeLayerLineCap);

extern_static!(kCALineCapSquare: &'static CAShapeLayerLineCap);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "QuartzCore_CAShapeLayer")]
    unsafe impl CAShapeLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(
            this: Option<Allocated<Self>>,
            layer: &Object,
        ) -> Id<Self, Shared>;
    }
);