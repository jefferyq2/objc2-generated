//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFontPanelModeMask(pub NSUInteger);
impl NSFontPanelModeMask {
    #[doc(alias = "NSFontPanelModeMaskFace")]
    pub const Face: Self = Self(1 << 0);
    #[doc(alias = "NSFontPanelModeMaskSize")]
    pub const Size: Self = Self(1 << 1);
    #[doc(alias = "NSFontPanelModeMaskCollection")]
    pub const Collection: Self = Self(1 << 2);
    #[doc(alias = "NSFontPanelModeMaskUnderlineEffect")]
    pub const UnderlineEffect: Self = Self(1 << 8);
    #[doc(alias = "NSFontPanelModeMaskStrikethroughEffect")]
    pub const StrikethroughEffect: Self = Self(1 << 9);
    #[doc(alias = "NSFontPanelModeMaskTextColorEffect")]
    pub const TextColorEffect: Self = Self(1 << 10);
    #[doc(alias = "NSFontPanelModeMaskDocumentColorEffect")]
    pub const DocumentColorEffect: Self = Self(1 << 11);
    #[doc(alias = "NSFontPanelModeMaskShadowEffect")]
    pub const ShadowEffect: Self = Self(1 << 12);
    #[doc(alias = "NSFontPanelModeMaskAllEffects")]
    pub const AllEffects: Self = Self(0xFFF00);
    pub const NSFontPanelModesMaskStandardModes: Self = Self(0xFFFF);
    pub const NSFontPanelModesMaskAllModes: Self = Self(0xFFFFFFFF);
}

unsafe impl Encode for NSFontPanelModeMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFontPanelModeMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait NSFontChanging: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(feature = "NSFontManager")]
        #[optional]
        #[method(changeFont:)]
        unsafe fn changeFont(&self, sender: Option<&NSFontManager>);

        #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
        #[optional]
        #[method(validModesForFontPanel:)]
        unsafe fn validModesForFontPanel(&self, font_panel: &NSFontPanel) -> NSFontPanelModeMask;
    }

    unsafe impl ProtocolType for dyn NSFontChanging {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
    pub struct NSFontPanel;

    #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl ClassType for NSFontPanel {
        #[inherits(NSWindow, NSResponder, NSObject)]
        type Super = NSPanel;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSAccessibility for NSFontPanel {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSAccessibilityElementProtocol for NSFontPanel {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSAnimatablePropertyContainer for NSFontPanel {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSAppearanceCustomization for NSFontPanel {}

#[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
unsafe impl NSCoding for NSFontPanel {}

#[cfg(all(
    feature = "NSMenu",
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSWindow"
))]
unsafe impl NSMenuItemValidation for NSFontPanel {}

#[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
unsafe impl NSObjectProtocol for NSFontPanel {}

#[cfg(all(
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSWindow"
))]
unsafe impl NSUserInterfaceItemIdentification for NSFontPanel {}

#[cfg(all(
    feature = "NSPanel",
    feature = "NSResponder",
    feature = "NSUserInterfaceValidation",
    feature = "NSWindow"
))]
unsafe impl NSUserInterfaceValidations for NSFontPanel {}

extern_methods!(
    #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSFontPanel {
        #[method_id(@__retain_semantics Other sharedFontPanel)]
        pub unsafe fn sharedFontPanel(mtm: MainThreadMarker) -> Id<NSFontPanel>;

        #[method(sharedFontPanelExists)]
        pub unsafe fn sharedFontPanelExists(mtm: MainThreadMarker) -> bool;

        #[cfg(feature = "NSView")]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "NSView")]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[cfg(feature = "NSFont")]
        #[method(setPanelFont:isMultiple:)]
        pub unsafe fn setPanelFont_isMultiple(&self, font_obj: &NSFont, flag: bool);

        #[cfg(feature = "NSFont")]
        #[method_id(@__retain_semantics Other panelConvertFont:)]
        pub unsafe fn panelConvertFont(&self, font_obj: &NSFont) -> Id<NSFont>;

        #[method(worksWhenModal)]
        pub unsafe fn worksWhenModal(&self) -> bool;

        #[method(setWorksWhenModal:)]
        pub unsafe fn setWorksWhenModal(&self, works_when_modal: bool);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(reloadDefaultFontFamilies)]
        pub unsafe fn reloadDefaultFontFamilies(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSWindow`
    #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSFontPanel {
        #[cfg(feature = "NSGraphics")]
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer(
            this: Allocated<Self>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(all(feature = "NSGraphics", feature = "NSScreen"))]
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:screen:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer_screen(
            this: Allocated<Self>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
            screen: Option<&NSScreen>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "NSViewController")]
        #[method_id(@__retain_semantics Other windowWithContentViewController:)]
        pub unsafe fn windowWithContentViewController(
            content_view_controller: &NSViewController,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSFontPanel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSPanel", feature = "NSResponder", feature = "NSWindow"))]
    unsafe impl NSFontPanel {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

pub const NSFontPanelFaceModeMask: c_uint = 1 << 0;
pub const NSFontPanelSizeModeMask: c_uint = 1 << 1;
pub const NSFontPanelCollectionModeMask: c_uint = 1 << 2;
pub const NSFontPanelUnderlineEffectModeMask: c_uint = 1 << 8;
pub const NSFontPanelStrikethroughEffectModeMask: c_uint = 1 << 9;
pub const NSFontPanelTextColorEffectModeMask: c_uint = 1 << 10;
pub const NSFontPanelDocumentColorEffectModeMask: c_uint = 1 << 11;
pub const NSFontPanelShadowEffectModeMask: c_uint = 1 << 12;
pub const NSFontPanelAllEffectsModeMask: c_uint = 0xFFF00;
pub const NSFontPanelStandardModesMask: c_uint = 0xFFFF;
pub const NSFontPanelAllModesMask: c_uint = 0xFFFFFFFF;

pub const NSFPPreviewButton: c_uint = 131;
pub const NSFPRevertButton: c_uint = 130;
pub const NSFPSetButton: c_uint = 132;
pub const NSFPPreviewField: c_uint = 128;
pub const NSFPSizeField: c_uint = 129;
pub const NSFPSizeTitle: c_uint = 133;
pub const NSFPCurrentField: c_uint = 134;
