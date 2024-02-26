//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

#[cfg(feature = "Foundation_NSString")]
extern_static!(GCMouseDidConnectNotification: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(GCMouseDidDisconnectNotification: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(GCMouseDidBecomeCurrentNotification: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(GCMouseDidStopBeingCurrentNotification: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCMouse")]
    pub struct GCMouse;

    #[cfg(feature = "GameController_GCMouse")]
    unsafe impl ClassType for GCMouse {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameController_GCMouse")]
unsafe impl GCDevice for GCMouse {}

#[cfg(feature = "GameController_GCMouse")]
unsafe impl NSObjectProtocol for GCMouse {}

extern_methods!(
    #[cfg(feature = "GameController_GCMouse")]
    unsafe impl GCMouse {
        #[cfg(feature = "GameController_GCMouseInput")]
        #[method_id(@__retain_semantics Other mouseInput)]
        pub unsafe fn mouseInput(&self) -> Option<Id<GCMouseInput>>;

        #[method_id(@__retain_semantics Other current)]
        pub unsafe fn current() -> Option<Id<GCMouse>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other mice)]
        pub unsafe fn mice() -> Id<NSArray<GCMouse>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameController_GCMouse")]
    unsafe impl GCMouse {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
