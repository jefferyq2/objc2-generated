//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSwitch;

    unsafe impl ClassType for NSSwitch {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
    }
);

extern_methods!(
    unsafe impl NSSwitch {
        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;

        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);
    }
);
