//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_protocol!(
    pub struct GKViewController;

    unsafe impl ProtocolType for GKViewController {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKDialogController")]
    pub struct GKDialogController;

    #[cfg(feature = "GameKit_GKDialogController")]
    unsafe impl ClassType for GKDialogController {
        #[inherits(NSObject)]
        type Super = NSResponder;
    }
);

extern_methods!(
    #[cfg(feature = "GameKit_GKDialogController")]
    unsafe impl GKDialogController {
        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other parentWindow)]
        pub unsafe fn parentWindow(&self) -> Option<Id<NSWindow, Shared>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(setParentWindow:)]
        pub unsafe fn setParentWindow(&self, parent_window: Option<&NSWindow>);

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(presentViewController:)]
        pub unsafe fn presentViewController(&self, view_controller: &NSViewController) -> bool;

        #[method(dismiss:)]
        pub unsafe fn dismiss(&self, sender: &Object);
    }
);

extern_methods!(
    /// SharedDialogController
    #[cfg(feature = "GameKit_GKDialogController")]
    unsafe impl GKDialogController {
        #[method_id(@__retain_semantics Other sharedDialogController)]
        pub unsafe fn sharedDialogController() -> Id<GKDialogController, Shared>;
    }
);