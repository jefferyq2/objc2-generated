//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLElement")]
    #[deprecated]
    pub struct DOMHTMLElement;

    #[cfg(feature = "WebKit_DOMHTMLElement")]
    unsafe impl ClassType for DOMHTMLElement {
        #[inherits(DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLElement")]
unsafe impl DOMEventTarget for DOMHTMLElement {}

#[cfg(feature = "WebKit_DOMHTMLElement")]
unsafe impl NSObjectProtocol for DOMHTMLElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLElement")]
    unsafe impl DOMHTMLElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other lang)]
        pub unsafe fn lang(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLang:)]
        pub unsafe fn setLang(&self, lang: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dir)]
        pub unsafe fn dir(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDir:)]
        pub unsafe fn setDir(&self, dir: Option<&NSString>);

        #[method(tabIndex)]
        pub unsafe fn tabIndex(&self) -> c_int;

        #[method(setTabIndex:)]
        pub unsafe fn setTabIndex(&self, tab_index: c_int);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other accessKey)]
        pub unsafe fn accessKey(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAccessKey:)]
        pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other innerText)]
        pub unsafe fn innerText(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setInnerText:)]
        pub unsafe fn setInnerText(&self, inner_text: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other outerText)]
        pub unsafe fn outerText(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setOuterText:)]
        pub unsafe fn setOuterText(&self, outer_text: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other contentEditable)]
        pub unsafe fn contentEditable(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setContentEditable:)]
        pub unsafe fn setContentEditable(&self, content_editable: Option<&NSString>);

        #[method(isContentEditable)]
        pub unsafe fn isContentEditable(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other idName)]
        pub unsafe fn idName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setIdName:)]
        pub unsafe fn setIdName(&self, id_name: Option<&NSString>);

        #[cfg(feature = "WebKit_DOMHTMLCollection")]
        #[method_id(@__retain_semantics Other children)]
        pub unsafe fn children(&self) -> Option<Id<DOMHTMLCollection>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other titleDisplayString)]
        pub unsafe fn titleDisplayString(&self) -> Id<NSString>;

        #[method(click)]
        pub unsafe fn click(&self);
    }
);
