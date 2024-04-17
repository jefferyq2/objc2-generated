//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMHTMLElement;

    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMHTMLElement {
        #[inherits(DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMEventTarget",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMHTMLElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLElement {
        #[deprecated]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other lang)]
        pub unsafe fn lang(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setLang:)]
        pub unsafe fn setLang(&self, lang: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other dir)]
        pub unsafe fn dir(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setDir:)]
        pub unsafe fn setDir(&self, dir: Option<&NSString>);

        #[deprecated]
        #[method(tabIndex)]
        pub unsafe fn tabIndex(&self) -> c_int;

        #[deprecated]
        #[method(setTabIndex:)]
        pub unsafe fn setTabIndex(&self, tab_index: c_int);

        #[method_id(@__retain_semantics Other accessKey)]
        pub unsafe fn accessKey(&self) -> Id<NSString>;

        #[method(setAccessKey:)]
        pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other innerText)]
        pub unsafe fn innerText(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setInnerText:)]
        pub unsafe fn setInnerText(&self, inner_text: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other outerText)]
        pub unsafe fn outerText(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setOuterText:)]
        pub unsafe fn setOuterText(&self, outer_text: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other contentEditable)]
        pub unsafe fn contentEditable(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setContentEditable:)]
        pub unsafe fn setContentEditable(&self, content_editable: Option<&NSString>);

        #[deprecated]
        #[method(isContentEditable)]
        pub unsafe fn isContentEditable(&self) -> bool;

        #[deprecated]
        #[method_id(@__retain_semantics Other idName)]
        pub unsafe fn idName(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setIdName:)]
        pub unsafe fn setIdName(&self, id_name: Option<&NSString>);

        #[cfg(feature = "DOMHTMLCollection")]
        #[deprecated]
        #[method_id(@__retain_semantics Other children)]
        pub unsafe fn children(&self) -> Option<Id<DOMHTMLCollection>>;

        #[method_id(@__retain_semantics Other titleDisplayString)]
        pub unsafe fn titleDisplayString(&self) -> Id<NSString>;

        #[method(click)]
        pub unsafe fn click(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
