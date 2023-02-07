//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMFileList")]
    #[deprecated]
    pub struct DOMFileList;

    #[cfg(feature = "WebKit_DOMFileList")]
    unsafe impl ClassType for DOMFileList {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
    }
);

#[cfg(feature = "WebKit_DOMFileList")]
unsafe impl NSObjectProtocol for DOMFileList {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMFileList")]
    unsafe impl DOMFileList {
        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[cfg(feature = "WebKit_DOMFile")]
        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Id<DOMFile>>;
    }
);
