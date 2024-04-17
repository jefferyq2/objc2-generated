//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// WebDOMNodeOperations
    #[cfg(all(
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMNode {
        #[cfg(feature = "WebArchive")]
        #[deprecated]
        #[method_id(@__retain_semantics Other webArchive)]
        pub unsafe fn webArchive(&self) -> Option<Id<WebArchive>>;
    }
);

extern_methods!(
    /// WebDOMDocumentOperations
    #[cfg(all(
        feature = "DOMDocument",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMDocument {
        #[cfg(feature = "WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other webFrame)]
        pub unsafe fn webFrame(&self) -> Option<Id<WebFrame>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other URLWithAttributeString:)]
        pub unsafe fn URLWithAttributeString(&self, string: Option<&NSString>)
            -> Option<Id<NSURL>>;
    }
);

extern_methods!(
    /// WebDOMRangeOperations
    #[cfg(all(
        feature = "DOMObject",
        feature = "DOMRange",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMRange {
        #[cfg(feature = "WebArchive")]
        #[deprecated]
        #[method_id(@__retain_semantics Other webArchive)]
        pub unsafe fn webArchive(&self) -> Option<Id<WebArchive>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other markupString)]
        pub unsafe fn markupString(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// WebDOMHTMLFrameElementOperations
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMHTMLFrameElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLFrameElement {
        #[cfg(feature = "WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other contentFrame)]
        pub unsafe fn contentFrame(&self) -> Option<Id<WebFrame>>;
    }
);

extern_methods!(
    /// WebDOMHTMLIFrameElementOperations
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMHTMLIFrameElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLIFrameElement {
        #[cfg(feature = "WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other contentFrame)]
        pub unsafe fn contentFrame(&self) -> Option<Id<WebFrame>>;
    }
);

extern_methods!(
    /// WebDOMHTMLObjectElementOperations
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMHTMLObjectElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLObjectElement {
        #[cfg(feature = "WebFrame")]
        #[deprecated]
        #[method_id(@__retain_semantics Other contentFrame)]
        pub unsafe fn contentFrame(&self) -> Option<Id<WebFrame>>;
    }
);
