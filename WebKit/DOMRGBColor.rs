//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMRGBColor")]
    #[deprecated]
    pub struct DOMRGBColor;

    #[cfg(feature = "WebKit_DOMRGBColor")]
    unsafe impl ClassType for DOMRGBColor {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMRGBColor")]
unsafe impl NSCopying for DOMRGBColor {}

#[cfg(feature = "WebKit_DOMRGBColor")]
unsafe impl NSObjectProtocol for DOMRGBColor {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMRGBColor")]
    unsafe impl DOMRGBColor {
        #[cfg(feature = "WebKit_DOMCSSPrimitiveValue")]
        #[deprecated]
        #[method_id(@__retain_semantics Other red)]
        pub unsafe fn red(&self) -> Option<Id<DOMCSSPrimitiveValue>>;

        #[cfg(feature = "WebKit_DOMCSSPrimitiveValue")]
        #[deprecated]
        #[method_id(@__retain_semantics Other green)]
        pub unsafe fn green(&self) -> Option<Id<DOMCSSPrimitiveValue>>;

        #[cfg(feature = "WebKit_DOMCSSPrimitiveValue")]
        #[deprecated]
        #[method_id(@__retain_semantics Other blue)]
        pub unsafe fn blue(&self) -> Option<Id<DOMCSSPrimitiveValue>>;

        #[cfg(feature = "WebKit_DOMCSSPrimitiveValue")]
        #[deprecated]
        #[method_id(@__retain_semantics Other alpha)]
        pub unsafe fn alpha(&self) -> Option<Id<DOMCSSPrimitiveValue>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Id<NSColor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMRGBColor")]
    unsafe impl DOMRGBColor {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMRGBColor")]
    unsafe impl DOMRGBColor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
