//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKSnapshotConfiguration")]
    pub struct WKSnapshotConfiguration;

    #[cfg(feature = "WebKit_WKSnapshotConfiguration")]
    unsafe impl ClassType for WKSnapshotConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WKSnapshotConfiguration")]
unsafe impl NSObjectProtocol for WKSnapshotConfiguration {}

extern_methods!(
    #[cfg(feature = "WebKit_WKSnapshotConfiguration")]
    unsafe impl WKSnapshotConfiguration {
        #[method(rect)]
        pub unsafe fn rect(&self) -> CGRect;

        #[method(setRect:)]
        pub unsafe fn setRect(&self, rect: CGRect);

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other snapshotWidth)]
        pub unsafe fn snapshotWidth(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setSnapshotWidth:)]
        pub unsafe fn setSnapshotWidth(&self, snapshot_width: Option<&NSNumber>);

        #[method(afterScreenUpdates)]
        pub unsafe fn afterScreenUpdates(&self) -> bool;

        #[method(setAfterScreenUpdates:)]
        pub unsafe fn setAfterScreenUpdates(&self, after_screen_updates: bool);
    }
);
