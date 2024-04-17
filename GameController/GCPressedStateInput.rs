//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait GCPressedStateInput: NSObjectProtocol {
        #[cfg(all(feature = "GCPhysicalInputElement", feature = "block2"))]
        #[method(pressedDidChangeHandler)]
        unsafe fn pressedDidChangeHandler(
            &self,
        ) -> *mut Block<
            dyn Fn(
                NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                NonNull<ProtocolObject<dyn GCPressedStateInput>>,
                Bool,
            ),
        >;

        #[cfg(all(feature = "GCPhysicalInputElement", feature = "block2"))]
        #[method(setPressedDidChangeHandler:)]
        unsafe fn setPressedDidChangeHandler(
            &self,
            pressed_did_change_handler: Option<
                &Block<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                        NonNull<ProtocolObject<dyn GCPressedStateInput>>,
                        Bool,
                    ),
                >,
            >,
        );

        #[method(isPressed)]
        unsafe fn isPressed(&self) -> bool;

        #[method(lastPressedStateTimestamp)]
        unsafe fn lastPressedStateTimestamp(&self) -> NSTimeInterval;

        #[method(lastPressedStateLatency)]
        unsafe fn lastPressedStateLatency(&self) -> NSTimeInterval;

        #[cfg(feature = "GCPhysicalInputSource")]
        #[method_id(@__retain_semantics Other sources)]
        unsafe fn sources(&self) -> Id<NSSet<ProtocolObject<dyn GCPhysicalInputSource>>>;
    }

    unsafe impl ProtocolType for dyn GCPressedStateInput {}
);
