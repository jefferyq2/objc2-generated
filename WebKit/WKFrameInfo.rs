//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKFrameInfo")]
    pub struct WKFrameInfo;

    #[cfg(feature = "WebKit_WKFrameInfo")]
    unsafe impl ClassType for WKFrameInfo {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WKFrameInfo")]
unsafe impl NSObjectProtocol for WKFrameInfo {}

extern_methods!(
    #[cfg(feature = "WebKit_WKFrameInfo")]
    unsafe impl WKFrameInfo {
        #[method(isMainFrame)]
        pub unsafe fn isMainFrame(&self) -> bool;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Id<NSURLRequest>;

        #[cfg(feature = "WebKit_WKSecurityOrigin")]
        #[method_id(@__retain_semantics Other securityOrigin)]
        pub unsafe fn securityOrigin(&self) -> Id<WKSecurityOrigin>;

        #[cfg(feature = "WebKit_WKWebView")]
        #[method_id(@__retain_semantics Other webView)]
        pub unsafe fn webView(&self) -> Option<Id<WKWebView>>;
    }
);
