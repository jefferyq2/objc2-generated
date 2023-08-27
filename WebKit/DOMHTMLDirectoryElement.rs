//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLDirectoryElement")]
    #[deprecated]
    pub struct DOMHTMLDirectoryElement;

    #[cfg(feature = "WebKit_DOMHTMLDirectoryElement")]
    unsafe impl ClassType for DOMHTMLDirectoryElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLDirectoryElement")]
unsafe impl DOMEventTarget for DOMHTMLDirectoryElement {}

#[cfg(feature = "WebKit_DOMHTMLDirectoryElement")]
unsafe impl NSCopying for DOMHTMLDirectoryElement {}

#[cfg(feature = "WebKit_DOMHTMLDirectoryElement")]
unsafe impl NSObjectProtocol for DOMHTMLDirectoryElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLDirectoryElement")]
    unsafe impl DOMHTMLDirectoryElement {
        #[deprecated]
        #[method(compact)]
        pub unsafe fn compact(&self) -> bool;

        #[deprecated]
        #[method(setCompact:)]
        pub unsafe fn setCompact(&self, compact: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLDirectoryElement")]
    unsafe impl DOMHTMLDirectoryElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLDirectoryElement")]
    unsafe impl DOMHTMLDirectoryElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
