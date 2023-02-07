//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMNodeIterator")]
    #[deprecated]
    pub struct DOMNodeIterator;

    #[cfg(feature = "WebKit_DOMNodeIterator")]
    unsafe impl ClassType for DOMNodeIterator {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
    }
);

#[cfg(feature = "WebKit_DOMNodeIterator")]
unsafe impl NSObjectProtocol for DOMNodeIterator {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMNodeIterator")]
    unsafe impl DOMNodeIterator {
        #[cfg(feature = "WebKit_DOMNode")]
        #[method_id(@__retain_semantics Other root)]
        pub unsafe fn root(&self) -> Option<Id<DOMNode>>;

        #[method(whatToShow)]
        pub unsafe fn whatToShow(&self) -> c_uint;

        #[method_id(@__retain_semantics Other filter)]
        pub unsafe fn filter(&self) -> Option<Id<ProtocolObject<dyn DOMNodeFilter>>>;

        #[method(expandEntityReferences)]
        pub unsafe fn expandEntityReferences(&self) -> bool;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method_id(@__retain_semantics Other referenceNode)]
        pub unsafe fn referenceNode(&self) -> Option<Id<DOMNode>>;

        #[method(pointerBeforeReferenceNode)]
        pub unsafe fn pointerBeforeReferenceNode(&self) -> bool;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method_id(@__retain_semantics Other nextNode)]
        pub unsafe fn nextNode(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method_id(@__retain_semantics Other previousNode)]
        pub unsafe fn previousNode(&self) -> Option<Id<DOMNode>>;

        #[method(detach)]
        pub unsafe fn detach(&self);
    }
);
