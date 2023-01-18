//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLHeadElement")]
    #[deprecated]
    pub struct DOMHTMLHeadElement;

    #[cfg(feature = "WebKit_DOMHTMLHeadElement")]
    unsafe impl ClassType for DOMHTMLHeadElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLHeadElement")]
    unsafe impl DOMHTMLHeadElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other profile)]
        pub unsafe fn profile(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setProfile:)]
        pub unsafe fn setProfile(&self, profile: Option<&NSString>);
    }
);