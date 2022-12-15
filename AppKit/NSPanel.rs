//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPanel;

    unsafe impl ClassType for NSPanel {
        #[inherits(NSResponder, NSObject)]
        type Super = NSWindow;
    }
);

extern_methods!(
    unsafe impl NSPanel {
        #[method(isFloatingPanel)]
        pub unsafe fn isFloatingPanel(&self) -> bool;

        #[method(setFloatingPanel:)]
        pub unsafe fn setFloatingPanel(&self, floatingPanel: bool);

        #[method(becomesKeyOnlyIfNeeded)]
        pub unsafe fn becomesKeyOnlyIfNeeded(&self) -> bool;

        #[method(setBecomesKeyOnlyIfNeeded:)]
        pub unsafe fn setBecomesKeyOnlyIfNeeded(&self, becomesKeyOnlyIfNeeded: bool);

        #[method(worksWhenModal)]
        pub unsafe fn worksWhenModal(&self) -> bool;

        #[method(setWorksWhenModal:)]
        pub unsafe fn setWorksWhenModal(&self, worksWhenModal: bool);
    }
);

extern_fn!(
    pub unsafe fn NSReleaseAlertPanel(panel: Option<&Object>);
);

extern_enum!(
    #[underlying(c_int)]
    pub enum {
        NSAlertDefaultReturn = 1,
        NSAlertAlternateReturn = 0,
        NSAlertOtherReturn = -1,
        NSAlertErrorReturn = -2,
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
    }
);
