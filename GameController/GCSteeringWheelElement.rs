//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCSteeringWheelElement;

    unsafe impl ClassType for GCSteeringWheelElement {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "GCAxisElement", feature = "GCPhysicalInputElement"))]
unsafe impl GCAxisElement for GCSteeringWheelElement {}

#[cfg(feature = "GCPhysicalInputElement")]
unsafe impl GCPhysicalInputElement for GCSteeringWheelElement {}

unsafe impl NSObjectProtocol for GCSteeringWheelElement {}

extern_methods!(
    unsafe impl GCSteeringWheelElement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method(maximumDegreesOfRotation)]
        pub unsafe fn maximumDegreesOfRotation(&self) -> c_float;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCSteeringWheelElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
