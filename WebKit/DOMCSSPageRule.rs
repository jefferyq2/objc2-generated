//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMCSSRule",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMCSSPageRule;

    #[cfg(all(
        feature = "DOMCSSRule",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMCSSPageRule {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCSSRule;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "DOMCSSRule",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMCSSPageRule {}

#[cfg(all(
    feature = "DOMCSSRule",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMCSSPageRule {}

extern_methods!(
    #[cfg(all(
        feature = "DOMCSSRule",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSPageRule {
        #[deprecated]
        #[method_id(@__retain_semantics Other selectorText)]
        pub unsafe fn selectorText(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setSelectorText:)]
        pub unsafe fn setSelectorText(&self, selector_text: Option<&NSString>);

        #[cfg(feature = "DOMCSSStyleDeclaration")]
        #[deprecated]
        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Id<DOMCSSStyleDeclaration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMCSSRule",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSPageRule {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMCSSRule",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMCSSPageRule {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
