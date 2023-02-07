//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WebResource")]
    pub struct WebResource;

    #[cfg(feature = "WebKit_WebResource")]
    unsafe impl ClassType for WebResource {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WebResource")]
unsafe impl NSCoding for WebResource {}

#[cfg(feature = "WebKit_WebResource")]
unsafe impl NSObjectProtocol for WebResource {}

extern_methods!(
    #[cfg(feature = "WebKit_WebResource")]
    unsafe impl WebResource {
        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithData:URL:MIMEType:textEncodingName:frameName:)]
        pub unsafe fn initWithData_URL_MIMEType_textEncodingName_frameName(
            this: Option<Allocated<Self>>,
            data: Option<&NSData>,
            url: Option<&NSURL>,
            mime_type: Option<&NSString>,
            text_encoding_name: Option<&NSString>,
            frame_name: Option<&NSString>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other MIMEType)]
        pub unsafe fn MIMEType(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textEncodingName)]
        pub unsafe fn textEncodingName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other frameName)]
        pub unsafe fn frameName(&self) -> Id<NSString>;
    }
);
