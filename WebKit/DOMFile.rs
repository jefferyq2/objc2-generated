//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMFile")]
    #[deprecated]
    pub struct DOMFile;

    #[cfg(feature = "WebKit_DOMFile")]
    unsafe impl ClassType for DOMFile {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMBlob;
    }
);

#[cfg(feature = "WebKit_DOMFile")]
unsafe impl NSObjectProtocol for DOMFile {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMFile")]
    unsafe impl DOMFile {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;
    }
);
