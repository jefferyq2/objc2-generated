//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMEntity")]
    #[deprecated]
    pub struct DOMEntity;

    #[cfg(feature = "WebKit_DOMEntity")]
    unsafe impl ClassType for DOMEntity {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMNode;
    }
);

#[cfg(feature = "WebKit_DOMEntity")]
unsafe impl DOMEventTarget for DOMEntity {}

#[cfg(feature = "WebKit_DOMEntity")]
unsafe impl NSObjectProtocol for DOMEntity {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMEntity")]
    unsafe impl DOMEntity {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other publicId)]
        pub unsafe fn publicId(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other systemId)]
        pub unsafe fn systemId(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other notationName)]
        pub unsafe fn notationName(&self) -> Id<NSString>;
    }
);
