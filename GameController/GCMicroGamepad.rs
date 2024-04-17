//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static GCInputMicroGamepadDpad: &'static NSString;
}

extern "C" {
    pub static GCInputMicroGamepadButtonA: &'static NSString;
}

extern "C" {
    pub static GCInputMicroGamepadButtonX: &'static NSString;
}

extern "C" {
    pub static GCInputMicroGamepadButtonMenu: &'static NSString;
}

#[cfg(all(
    feature = "GCControllerElement",
    feature = "GCPhysicalInputProfile",
    feature = "block2"
))]
pub type GCMicroGamepadValueChangedHandler =
    *mut Block<dyn Fn(NonNull<GCMicroGamepad>, NonNull<GCControllerElement>)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GCPhysicalInputProfile")]
    pub struct GCMicroGamepad;

    #[cfg(feature = "GCPhysicalInputProfile")]
    unsafe impl ClassType for GCMicroGamepad {
        #[inherits(NSObject)]
        type Super = GCPhysicalInputProfile;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GCPhysicalInputProfile")]
unsafe impl NSObjectProtocol for GCMicroGamepad {}

extern_methods!(
    #[cfg(feature = "GCPhysicalInputProfile")]
    unsafe impl GCMicroGamepad {
        #[cfg(feature = "GCController")]
        #[method_id(@__retain_semantics Other controller)]
        pub unsafe fn controller(&self) -> Option<Id<GCController>>;

        #[cfg(all(feature = "GCControllerElement", feature = "block2"))]
        #[method(valueChangedHandler)]
        pub unsafe fn valueChangedHandler(&self) -> GCMicroGamepadValueChangedHandler;

        #[cfg(all(feature = "GCControllerElement", feature = "block2"))]
        #[method(setValueChangedHandler:)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCMicroGamepadValueChangedHandler,
        );

        #[cfg(feature = "GCMicroGamepadSnapshot")]
        #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController capture] instead"]
        #[method_id(@__retain_semantics Other saveSnapshot)]
        pub unsafe fn saveSnapshot(&self) -> Id<GCMicroGamepadSnapshot>;

        #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other dpad)]
        pub unsafe fn dpad(&self) -> Id<GCControllerDirectionPad>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other buttonA)]
        pub unsafe fn buttonA(&self) -> Id<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other buttonX)]
        pub unsafe fn buttonX(&self) -> Id<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other buttonMenu)]
        pub unsafe fn buttonMenu(&self) -> Id<GCControllerButtonInput>;

        #[method(reportsAbsoluteDpadValues)]
        pub unsafe fn reportsAbsoluteDpadValues(&self) -> bool;

        #[method(setReportsAbsoluteDpadValues:)]
        pub unsafe fn setReportsAbsoluteDpadValues(&self, reports_absolute_dpad_values: bool);

        #[method(allowsRotation)]
        pub unsafe fn allowsRotation(&self) -> bool;

        #[method(setAllowsRotation:)]
        pub unsafe fn setAllowsRotation(&self, allows_rotation: bool);

        #[method(setStateFromMicroGamepad:)]
        pub unsafe fn setStateFromMicroGamepad(&self, micro_gamepad: &GCMicroGamepad);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GCPhysicalInputProfile")]
    unsafe impl GCMicroGamepad {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
