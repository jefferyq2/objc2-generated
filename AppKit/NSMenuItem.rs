//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMenuItem;

    unsafe impl ClassType for NSMenuItem {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "NSAccessibilityProtocols")]
unsafe impl NSAccessibility for NSMenuItem {}

#[cfg(feature = "NSAccessibilityProtocols")]
unsafe impl NSAccessibilityElementProtocol for NSMenuItem {}

unsafe impl NSCoding for NSMenuItem {}

unsafe impl NSCopying for NSMenuItem {}

unsafe impl NSObjectProtocol for NSMenuItem {}

#[cfg(feature = "NSUserInterfaceItemIdentification")]
unsafe impl NSUserInterfaceItemIdentification for NSMenuItem {}

#[cfg(feature = "NSUserInterfaceValidation")]
unsafe impl NSValidatedUserInterfaceItem for NSMenuItem {}

extern_methods!(
    unsafe impl NSMenuItem {
        #[method(usesUserKeyEquivalents)]
        pub unsafe fn usesUserKeyEquivalents(mtm: MainThreadMarker) -> bool;

        #[method(setUsesUserKeyEquivalents:)]
        pub unsafe fn setUsesUserKeyEquivalents(
            uses_user_key_equivalents: bool,
            mtm: MainThreadMarker,
        );

        #[method_id(@__retain_semantics Other separatorItem)]
        pub fn separatorItem(mtm: MainThreadMarker) -> Id<NSMenuItem>;

        #[method_id(@__retain_semantics Other sectionHeaderWithTitle:)]
        pub unsafe fn sectionHeaderWithTitle(title: &NSString, mtm: MainThreadMarker) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithTitle:action:keyEquivalent:)]
        pub unsafe fn initWithTitle_action_keyEquivalent(
            this: Allocated<Self>,
            string: &NSString,
            selector: Option<Sel>,
            char_code: &NSString,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[method(hasSubmenu)]
        pub unsafe fn hasSubmenu(&self) -> bool;

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other submenu)]
        pub unsafe fn submenu(&self) -> Option<Id<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        #[method(setSubmenu:)]
        pub fn setSubmenu(&self, submenu: Option<&NSMenu>);

        #[method_id(@__retain_semantics Other parentItem)]
        pub unsafe fn parentItem(&self) -> Option<Id<NSMenuItem>>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Id<NSAttributedString>>;

        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: Option<&NSAttributedString>);

        #[method(isSeparatorItem)]
        pub unsafe fn isSeparatorItem(&self) -> bool;

        #[method(isSectionHeader)]
        pub unsafe fn isSectionHeader(&self) -> bool;

        #[method_id(@__retain_semantics Other keyEquivalent)]
        pub unsafe fn keyEquivalent(&self) -> Id<NSString>;

        #[method(setKeyEquivalent:)]
        pub unsafe fn setKeyEquivalent(&self, key_equivalent: &NSString);

        #[cfg(feature = "NSEvent")]
        #[method(keyEquivalentModifierMask)]
        pub unsafe fn keyEquivalentModifierMask(&self) -> NSEventModifierFlags;

        #[cfg(feature = "NSEvent")]
        #[method(setKeyEquivalentModifierMask:)]
        pub fn setKeyEquivalentModifierMask(
            &self,
            key_equivalent_modifier_mask: NSEventModifierFlags,
        );

        #[method_id(@__retain_semantics Other userKeyEquivalent)]
        pub unsafe fn userKeyEquivalent(&self) -> Id<NSString>;

        #[method(allowsKeyEquivalentWhenHidden)]
        pub unsafe fn allowsKeyEquivalentWhenHidden(&self) -> bool;

        #[method(setAllowsKeyEquivalentWhenHidden:)]
        pub unsafe fn setAllowsKeyEquivalentWhenHidden(
            &self,
            allows_key_equivalent_when_hidden: bool,
        );

        #[method(allowsAutomaticKeyEquivalentLocalization)]
        pub unsafe fn allowsAutomaticKeyEquivalentLocalization(&self) -> bool;

        #[method(setAllowsAutomaticKeyEquivalentLocalization:)]
        pub unsafe fn setAllowsAutomaticKeyEquivalentLocalization(
            &self,
            allows_automatic_key_equivalent_localization: bool,
        );

        #[method(allowsAutomaticKeyEquivalentMirroring)]
        pub unsafe fn allowsAutomaticKeyEquivalentMirroring(&self) -> bool;

        #[method(setAllowsAutomaticKeyEquivalentMirroring:)]
        pub unsafe fn setAllowsAutomaticKeyEquivalentMirroring(
            &self,
            allows_automatic_key_equivalent_mirroring: bool,
        );

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "NSCell")]
        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;

        #[cfg(feature = "NSCell")]
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other onStateImage)]
        pub unsafe fn onStateImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setOnStateImage:)]
        pub unsafe fn setOnStateImage(&self, on_state_image: Option<&NSImage>);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other offStateImage)]
        pub unsafe fn offStateImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setOffStateImage:)]
        pub unsafe fn setOffStateImage(&self, off_state_image: Option<&NSImage>);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other mixedStateImage)]
        pub unsafe fn mixedStateImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setMixedStateImage:)]
        pub unsafe fn setMixedStateImage(&self, mixed_state_image: Option<&NSImage>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(isAlternate)]
        pub unsafe fn isAlternate(&self) -> bool;

        #[method(setAlternate:)]
        pub unsafe fn setAlternate(&self, alternate: bool);

        #[method(indentationLevel)]
        pub unsafe fn indentationLevel(&self) -> NSInteger;

        #[method(setIndentationLevel:)]
        pub unsafe fn setIndentationLevel(&self, indentation_level: NSInteger);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<AnyObject>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<AnyObject>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, represented_object: Option<&AnyObject>);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[method(isHiddenOrHasHiddenAncestor)]
        pub unsafe fn isHiddenOrHasHiddenAncestor(&self) -> bool;

        #[method_id(@__retain_semantics Other toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Id<NSString>>;

        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, tool_tip: Option<&NSString>);

        #[cfg(feature = "NSMenuItemBadge")]
        #[method_id(@__retain_semantics Other badge)]
        pub unsafe fn badge(&self) -> Option<Id<NSMenuItemBadge>>;

        #[cfg(feature = "NSMenuItemBadge")]
        #[method(setBadge:)]
        pub unsafe fn setBadge(&self, badge: Option<&NSMenuItemBadge>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMenuItem {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// NSViewEnclosingMenuItem
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[method_id(@__retain_semantics Other enclosingMenuItem)]
        pub unsafe fn enclosingMenuItem(&self) -> Option<Id<NSMenuItem>>;
    }
);

extern "C" {
    #[cfg(feature = "NSUserInterfaceItemIdentification")]
    pub static NSMenuItemImportFromDeviceIdentifier: &'static NSUserInterfaceItemIdentifier;
}

extern_methods!(
    /// NSDeprecated
    unsafe impl NSMenuItem {
        #[deprecated]
        #[method(setMnemonicLocation:)]
        pub unsafe fn setMnemonicLocation(&self, location: NSUInteger);

        #[deprecated]
        #[method(mnemonicLocation)]
        pub unsafe fn mnemonicLocation(&self) -> NSUInteger;

        #[deprecated]
        #[method_id(@__retain_semantics Other mnemonic)]
        pub unsafe fn mnemonic(&self) -> Option<Id<NSString>>;

        #[deprecated]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: &NSString);
    }
);
