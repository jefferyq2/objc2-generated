//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSButtonTouchBarItem;

    unsafe impl ClassType for NSButtonTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
    }
);

extern_methods!(
    unsafe impl NSButtonTouchBarItem {
        #[method_id(@__retain_semantics Other buttonTouchBarItemWithIdentifier:title:target:action:)]
        pub unsafe fn buttonTouchBarItemWithIdentifier_title_target_action(
            identifier: &NSTouchBarItemIdentifier,
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other buttonTouchBarItemWithIdentifier:image:target:action:)]
        pub unsafe fn buttonTouchBarItemWithIdentifier_image_target_action(
            identifier: &NSTouchBarItemIdentifier,
            image: &NSImage,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other buttonTouchBarItemWithIdentifier:title:image:target:action:)]
        pub unsafe fn buttonTouchBarItemWithIdentifier_title_image_target_action(
            identifier: &NSTouchBarItemIdentifier,
            title: &NSString,
            image: &NSImage,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method_id(@__retain_semantics Other bezelColor)]
        pub unsafe fn bezelColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setBezelColor:)]
        pub unsafe fn setBezelColor(&self, bezelColor: Option<&NSColor>);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;

        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customizationLabel: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    unsafe impl NSButtonTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
    }
);
