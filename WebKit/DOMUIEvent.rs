//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMUIEvent;

    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMUIEvent {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMUIEvent {}

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMUIEvent {}

extern_methods!(
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMUIEvent {
        #[cfg(feature = "DOMAbstractView")]
        #[deprecated]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<DOMAbstractView>>;

        #[deprecated]
        #[method(detail)]
        pub unsafe fn detail(&self) -> c_int;

        #[method(keyCode)]
        pub unsafe fn keyCode(&self) -> c_int;

        #[method(charCode)]
        pub unsafe fn charCode(&self) -> c_int;

        #[deprecated]
        #[method(layerX)]
        pub unsafe fn layerX(&self) -> c_int;

        #[deprecated]
        #[method(layerY)]
        pub unsafe fn layerY(&self) -> c_int;

        #[method(pageX)]
        pub unsafe fn pageX(&self) -> c_int;

        #[method(pageY)]
        pub unsafe fn pageY(&self) -> c_int;

        #[method(which)]
        pub unsafe fn which(&self) -> c_int;

        #[cfg(feature = "DOMAbstractView")]
        #[method(initUIEvent:canBubble:cancelable:view:detail:)]
        pub unsafe fn initUIEvent_canBubble_cancelable_view_detail(
            &self,
            r#type: Option<&NSString>,
            can_bubble: bool,
            cancelable: bool,
            view: Option<&DOMAbstractView>,
            detail: c_int,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMUIEvent {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMUIEvent {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// DOMUIEventDeprecated
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMUIEvent {
        #[cfg(feature = "DOMAbstractView")]
        #[deprecated]
        #[method(initUIEvent:::::)]
        pub unsafe fn initUIEvent(
            &self,
            r#type: Option<&NSString>,
            can_bubble: bool,
            cancelable: bool,
            view: Option<&DOMAbstractView>,
            detail: c_int,
        );
    }
);
