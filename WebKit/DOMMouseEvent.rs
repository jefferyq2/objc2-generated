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
        feature = "DOMUIEvent",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMMouseEvent;

    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "DOMUIEvent",
        feature = "WebScriptObject"
    ))]
    unsafe impl ClassType for DOMMouseEvent {
        #[inherits(DOMEvent, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMUIEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "DOMUIEvent",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMMouseEvent {}

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "DOMUIEvent",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMMouseEvent {}

extern_methods!(
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "DOMUIEvent",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMMouseEvent {
        #[deprecated]
        #[method(screenX)]
        pub unsafe fn screenX(&self) -> c_int;

        #[deprecated]
        #[method(screenY)]
        pub unsafe fn screenY(&self) -> c_int;

        #[deprecated]
        #[method(clientX)]
        pub unsafe fn clientX(&self) -> c_int;

        #[deprecated]
        #[method(clientY)]
        pub unsafe fn clientY(&self) -> c_int;

        #[deprecated]
        #[method(ctrlKey)]
        pub unsafe fn ctrlKey(&self) -> bool;

        #[deprecated]
        #[method(shiftKey)]
        pub unsafe fn shiftKey(&self) -> bool;

        #[deprecated]
        #[method(altKey)]
        pub unsafe fn altKey(&self) -> bool;

        #[deprecated]
        #[method(metaKey)]
        pub unsafe fn metaKey(&self) -> bool;

        #[deprecated]
        #[method(button)]
        pub unsafe fn button(&self) -> c_short;

        #[cfg(feature = "DOMEventTarget")]
        #[deprecated]
        #[method_id(@__retain_semantics Other relatedTarget)]
        pub unsafe fn relatedTarget(&self) -> Option<Id<ProtocolObject<dyn DOMEventTarget>>>;

        #[method(offsetX)]
        pub unsafe fn offsetX(&self) -> c_int;

        #[method(offsetY)]
        pub unsafe fn offsetY(&self) -> c_int;

        #[method(x)]
        pub unsafe fn x(&self) -> c_int;

        #[method(y)]
        pub unsafe fn y(&self) -> c_int;

        #[cfg(feature = "DOMNode")]
        #[method_id(@__retain_semantics Other fromElement)]
        pub unsafe fn fromElement(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "DOMNode")]
        #[method_id(@__retain_semantics Other toElement)]
        pub unsafe fn toElement(&self) -> Option<Id<DOMNode>>;

        #[cfg(all(feature = "DOMAbstractView", feature = "DOMEventTarget"))]
        #[method(initMouseEvent:canBubble:cancelable:view:detail:screenX:screenY:clientX:clientY:ctrlKey:altKey:shiftKey:metaKey:button:relatedTarget:)]
        pub unsafe fn initMouseEvent_canBubble_cancelable_view_detail_screenX_screenY_clientX_clientY_ctrlKey_altKey_shiftKey_metaKey_button_relatedTarget(
            &self,
            r#type: Option<&NSString>,
            can_bubble: bool,
            cancelable: bool,
            view: Option<&DOMAbstractView>,
            detail: c_int,
            screen_x: c_int,
            screen_y: c_int,
            client_x: c_int,
            client_y: c_int,
            ctrl_key: bool,
            alt_key: bool,
            shift_key: bool,
            meta_key: bool,
            button: c_ushort,
            related_target: Option<&ProtocolObject<dyn DOMEventTarget>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "DOMUIEvent",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMMouseEvent {
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
        feature = "DOMUIEvent",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMMouseEvent {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// DOMMouseEventDeprecated
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "DOMUIEvent",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMMouseEvent {
        #[cfg(all(feature = "DOMAbstractView", feature = "DOMEventTarget"))]
        #[deprecated]
        #[method(initMouseEvent:::::::::::::::)]
        pub unsafe fn initMouseEvent(
            &self,
            r#type: Option<&NSString>,
            can_bubble: bool,
            cancelable: bool,
            view: Option<&DOMAbstractView>,
            detail: c_int,
            screen_x: c_int,
            screen_y: c_int,
            client_x: c_int,
            client_y: c_int,
            ctrl_key: bool,
            alt_key: bool,
            shift_key: bool,
            meta_key: bool,
            button: c_ushort,
            related_target: Option<&ProtocolObject<dyn DOMEventTarget>>,
        );
    }
);
