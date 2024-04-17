//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GCTouchState(pub NSInteger);
impl GCTouchState {
    #[doc(alias = "GCTouchStateUp")]
    pub const Up: Self = Self(0);
    #[doc(alias = "GCTouchStateDown")]
    pub const Down: Self = Self(1);
    #[doc(alias = "GCTouchStateMoving")]
    pub const Moving: Self = Self(2);
}

unsafe impl Encode for GCTouchState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GCTouchState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(all(feature = "GCControllerElement", feature = "block2"))]
pub type GCControllerTouchpadHandler =
    *mut Block<dyn Fn(NonNull<GCControllerTouchpad>, c_float, c_float, c_float, Bool)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GCControllerElement")]
    pub struct GCControllerTouchpad;

    #[cfg(feature = "GCControllerElement")]
    unsafe impl ClassType for GCControllerTouchpad {
        #[inherits(NSObject)]
        type Super = GCControllerElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GCControllerElement")]
unsafe impl NSObjectProtocol for GCControllerTouchpad {}

extern_methods!(
    #[cfg(feature = "GCControllerElement")]
    unsafe impl GCControllerTouchpad {
        #[cfg(feature = "GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other button)]
        pub unsafe fn button(&self) -> Id<GCControllerButtonInput>;

        #[cfg(feature = "block2")]
        #[method(touchDown)]
        pub unsafe fn touchDown(&self) -> GCControllerTouchpadHandler;

        #[cfg(feature = "block2")]
        #[method(setTouchDown:)]
        pub unsafe fn setTouchDown(&self, touch_down: GCControllerTouchpadHandler);

        #[cfg(feature = "block2")]
        #[method(touchMoved)]
        pub unsafe fn touchMoved(&self) -> GCControllerTouchpadHandler;

        #[cfg(feature = "block2")]
        #[method(setTouchMoved:)]
        pub unsafe fn setTouchMoved(&self, touch_moved: GCControllerTouchpadHandler);

        #[cfg(feature = "block2")]
        #[method(touchUp)]
        pub unsafe fn touchUp(&self) -> GCControllerTouchpadHandler;

        #[cfg(feature = "block2")]
        #[method(setTouchUp:)]
        pub unsafe fn setTouchUp(&self, touch_up: GCControllerTouchpadHandler);

        #[cfg(feature = "GCControllerDirectionPad")]
        #[method_id(@__retain_semantics Other touchSurface)]
        pub unsafe fn touchSurface(&self) -> Id<GCControllerDirectionPad>;

        #[method(touchState)]
        pub unsafe fn touchState(&self) -> GCTouchState;

        #[method(reportsAbsoluteTouchSurfaceValues)]
        pub unsafe fn reportsAbsoluteTouchSurfaceValues(&self) -> bool;

        #[method(setReportsAbsoluteTouchSurfaceValues:)]
        pub unsafe fn setReportsAbsoluteTouchSurfaceValues(
            &self,
            reports_absolute_touch_surface_values: bool,
        );

        #[method(setValueForXAxis:yAxis:touchDown:buttonValue:)]
        pub unsafe fn setValueForXAxis_yAxis_touchDown_buttonValue(
            &self,
            x_axis: c_float,
            y_axis: c_float,
            touch_down: bool,
            button_value: c_float,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GCControllerElement")]
    unsafe impl GCControllerTouchpad {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
