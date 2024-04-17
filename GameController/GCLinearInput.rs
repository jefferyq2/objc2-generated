//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait GCLinearInput: NSObjectProtocol {
        #[cfg(all(feature = "GCPhysicalInputElement", feature = "block2"))]
        #[method(valueDidChangeHandler)]
        unsafe fn valueDidChangeHandler(
            &self,
        ) -> *mut Block<
            dyn Fn(
                NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                NonNull<ProtocolObject<dyn GCLinearInput>>,
                c_float,
            ),
        >;

        #[cfg(all(feature = "GCPhysicalInputElement", feature = "block2"))]
        #[method(setValueDidChangeHandler:)]
        unsafe fn setValueDidChangeHandler(
            &self,
            value_did_change_handler: Option<
                &Block<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                        NonNull<ProtocolObject<dyn GCLinearInput>>,
                        c_float,
                    ),
                >,
            >,
        );

        #[method(value)]
        unsafe fn value(&self) -> c_float;

        #[method(isAnalog)]
        unsafe fn isAnalog(&self) -> bool;

        #[method(canWrap)]
        unsafe fn canWrap(&self) -> bool;

        #[method(lastValueTimestamp)]
        unsafe fn lastValueTimestamp(&self) -> NSTimeInterval;

        #[method(lastValueLatency)]
        unsafe fn lastValueLatency(&self) -> NSTimeInterval;

        #[cfg(feature = "GCPhysicalInputSource")]
        #[method_id(@__retain_semantics Other sources)]
        unsafe fn sources(&self) -> Id<NSSet<ProtocolObject<dyn GCPhysicalInputSource>>>;
    }

    unsafe impl ProtocolType for dyn GCLinearInput {}
);
