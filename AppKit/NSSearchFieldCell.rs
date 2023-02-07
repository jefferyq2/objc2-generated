//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSSearchFieldRecentsTitleMenuItemTag: NSInteger = 1000);

extern_static!(NSSearchFieldRecentsMenuItemTag: NSInteger = 1001);

extern_static!(NSSearchFieldClearRecentsMenuItemTag: NSInteger = 1002);

extern_static!(NSSearchFieldNoRecentsMenuItemTag: NSInteger = 1003);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSearchFieldCell")]
    pub struct NSSearchFieldCell;

    #[cfg(feature = "AppKit_NSSearchFieldCell")]
    unsafe impl ClassType for NSSearchFieldCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
    }
);

#[cfg(feature = "AppKit_NSSearchFieldCell")]
unsafe impl NSAccessibility for NSSearchFieldCell {}

#[cfg(feature = "AppKit_NSSearchFieldCell")]
unsafe impl NSAccessibilityElementProtocol for NSSearchFieldCell {}

#[cfg(feature = "AppKit_NSSearchFieldCell")]
unsafe impl NSCoding for NSSearchFieldCell {}

#[cfg(feature = "AppKit_NSSearchFieldCell")]
unsafe impl NSObjectProtocol for NSSearchFieldCell {}

#[cfg(feature = "AppKit_NSSearchFieldCell")]
unsafe impl NSUserInterfaceItemIdentification for NSSearchFieldCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSearchFieldCell")]
    unsafe impl NSSearchFieldCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSButtonCell")]
        #[method_id(@__retain_semantics Other searchButtonCell)]
        pub unsafe fn searchButtonCell(&self) -> Option<Id<NSButtonCell>>;

        #[cfg(feature = "AppKit_NSButtonCell")]
        #[method(setSearchButtonCell:)]
        pub unsafe fn setSearchButtonCell(&self, search_button_cell: Option<&NSButtonCell>);

        #[cfg(feature = "AppKit_NSButtonCell")]
        #[method_id(@__retain_semantics Other cancelButtonCell)]
        pub unsafe fn cancelButtonCell(&self) -> Option<Id<NSButtonCell>>;

        #[cfg(feature = "AppKit_NSButtonCell")]
        #[method(setCancelButtonCell:)]
        pub unsafe fn setCancelButtonCell(&self, cancel_button_cell: Option<&NSButtonCell>);

        #[method(resetSearchButtonCell)]
        pub unsafe fn resetSearchButtonCell(&self);

        #[method(resetCancelButtonCell)]
        pub unsafe fn resetCancelButtonCell(&self);

        #[method(searchTextRectForBounds:)]
        pub unsafe fn searchTextRectForBounds(&self, rect: NSRect) -> NSRect;

        #[method(searchButtonRectForBounds:)]
        pub unsafe fn searchButtonRectForBounds(&self, rect: NSRect) -> NSRect;

        #[method(cancelButtonRectForBounds:)]
        pub unsafe fn cancelButtonRectForBounds(&self, rect: NSRect) -> NSRect;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other searchMenuTemplate)]
        pub unsafe fn searchMenuTemplate(&self) -> Option<Id<NSMenu>>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setSearchMenuTemplate:)]
        pub unsafe fn setSearchMenuTemplate(&self, search_menu_template: Option<&NSMenu>);

        #[method(sendsWholeSearchString)]
        pub unsafe fn sendsWholeSearchString(&self) -> bool;

        #[method(setSendsWholeSearchString:)]
        pub unsafe fn setSendsWholeSearchString(&self, sends_whole_search_string: bool);

        #[method(maximumRecents)]
        pub unsafe fn maximumRecents(&self) -> NSInteger;

        #[method(setMaximumRecents:)]
        pub unsafe fn setMaximumRecents(&self, maximum_recents: NSInteger);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other recentSearches)]
        pub unsafe fn recentSearches(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setRecentSearches:)]
        pub unsafe fn setRecentSearches(&self, recent_searches: Option<&NSArray<NSString>>);

        #[method_id(@__retain_semantics Other recentsAutosaveName)]
        pub unsafe fn recentsAutosaveName(&self) -> Option<Id<NSSearchFieldRecentsAutosaveName>>;

        #[method(setRecentsAutosaveName:)]
        pub unsafe fn setRecentsAutosaveName(
            &self,
            recents_autosave_name: Option<&NSSearchFieldRecentsAutosaveName>,
        );

        #[method(sendsSearchStringImmediately)]
        pub unsafe fn sendsSearchStringImmediately(&self) -> bool;

        #[method(setSendsSearchStringImmediately:)]
        pub unsafe fn setSendsSearchStringImmediately(&self, sends_search_string_immediately: bool);
    }
);
