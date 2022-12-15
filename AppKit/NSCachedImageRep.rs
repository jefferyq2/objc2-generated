//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCachedImageRep;

    unsafe impl ClassType for NSCachedImageRep {
        #[inherits(NSObject)]
        type Super = NSImageRep;
    }
);

extern_methods!(
    unsafe impl NSCachedImageRep {
        #[method_id(@__retain_semantics Init initWithWindow:rect:)]
        pub unsafe fn initWithWindow_rect(
            this: Option<Allocated<Self>>,
            win: Option<&NSWindow>,
            rect: NSRect,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithSize:depth:separate:alpha:)]
        pub unsafe fn initWithSize_depth_separate_alpha(
            this: Option<Allocated<Self>>,
            size: NSSize,
            depth: NSWindowDepth,
            flag: bool,
            alpha: bool,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other window)]
        pub unsafe fn window(&self) -> Option<Id<NSWindow, Shared>>;

        #[method(rect)]
        pub unsafe fn rect(&self) -> NSRect;
    }
);
