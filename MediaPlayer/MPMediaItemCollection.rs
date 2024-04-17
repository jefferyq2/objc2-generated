//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPMediaEntity")]
    pub struct MPMediaItemCollection;

    #[cfg(feature = "MPMediaEntity")]
    unsafe impl ClassType for MPMediaItemCollection {
        #[inherits(NSObject)]
        type Super = MPMediaEntity;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MPMediaEntity")]
unsafe impl NSCoding for MPMediaItemCollection {}

#[cfg(feature = "MPMediaEntity")]
unsafe impl NSObjectProtocol for MPMediaItemCollection {}

#[cfg(feature = "MPMediaEntity")]
unsafe impl NSSecureCoding for MPMediaItemCollection {}

extern_methods!(
    #[cfg(feature = "MPMediaEntity")]
    unsafe impl MPMediaItemCollection {
        #[cfg(feature = "MPMediaItem")]
        #[method_id(@__retain_semantics Other collectionWithItems:)]
        pub unsafe fn collectionWithItems(
            items: &NSArray<MPMediaItem>,
        ) -> Id<MPMediaItemCollection>;

        #[cfg(feature = "MPMediaItem")]
        #[method_id(@__retain_semantics Init initWithItems:)]
        pub unsafe fn initWithItems(
            this: Allocated<Self>,
            items: &NSArray<MPMediaItem>,
        ) -> Id<Self>;

        #[cfg(feature = "MPMediaItem")]
        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Id<NSArray<MPMediaItem>>;

        #[cfg(feature = "MPMediaItem")]
        #[method_id(@__retain_semantics Other representativeItem)]
        pub unsafe fn representativeItem(&self) -> Option<Id<MPMediaItem>>;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[cfg(feature = "MPMediaItem")]
        #[method(mediaTypes)]
        pub unsafe fn mediaTypes(&self) -> MPMediaType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPMediaEntity")]
    unsafe impl MPMediaItemCollection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
