//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPathControl;

    unsafe impl ClassType for NSPathControl {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
    }
);

extern_methods!(
    unsafe impl NSPathControl {
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method_id(@__retain_semantics Other allowedTypes)]
        pub unsafe fn allowedTypes(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[method(setAllowedTypes:)]
        pub unsafe fn setAllowedTypes(&self, allowedTypes: Option<&NSArray<NSString>>);

        #[method_id(@__retain_semantics Other placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Id<NSString, Shared>>;

        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholderString: Option<&NSString>);

        #[method_id(@__retain_semantics Other placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholderAttributedString: Option<&NSAttributedString>,
        );

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;

        #[method(setURL:)]
        pub unsafe fn setURL(&self, URL: Option<&NSURL>);

        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;

        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, doubleAction: Option<Sel>);

        #[method(pathStyle)]
        pub unsafe fn pathStyle(&self) -> NSPathStyle;

        #[method(setPathStyle:)]
        pub unsafe fn setPathStyle(&self, pathStyle: NSPathStyle);

        #[method_id(@__retain_semantics Other clickedPathItem)]
        pub unsafe fn clickedPathItem(&self) -> Option<Id<NSPathControlItem, Shared>>;

        #[method_id(@__retain_semantics Other pathItems)]
        pub unsafe fn pathItems(&self) -> Id<NSArray<NSPathControlItem>, Shared>;

        #[method(setPathItems:)]
        pub unsafe fn setPathItems(&self, pathItems: &NSArray<NSPathControlItem>);

        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSPathControlDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSPathControlDelegate>);

        #[method(setDraggingSourceOperationMask:forLocal:)]
        pub unsafe fn setDraggingSourceOperationMask_forLocal(
            &self,
            mask: NSDragOperation,
            isLocal: bool,
        );

        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu, Shared>>;

        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);
    }
);

extern_protocol!(
    pub struct NSPathControlDelegate;

    unsafe impl ProtocolType for NSPathControlDelegate {
        #[optional]
        #[method(pathControl:shouldDragItem:withPasteboard:)]
        pub unsafe fn pathControl_shouldDragItem_withPasteboard(
            &self,
            pathControl: &NSPathControl,
            pathItem: &NSPathControlItem,
            pasteboard: &NSPasteboard,
        ) -> bool;

        #[optional]
        #[method(pathControl:shouldDragPathComponentCell:withPasteboard:)]
        pub unsafe fn pathControl_shouldDragPathComponentCell_withPasteboard(
            &self,
            pathControl: &NSPathControl,
            pathComponentCell: &NSPathComponentCell,
            pasteboard: &NSPasteboard,
        ) -> bool;

        #[optional]
        #[method(pathControl:validateDrop:)]
        pub unsafe fn pathControl_validateDrop(
            &self,
            pathControl: &NSPathControl,
            info: &NSDraggingInfo,
        ) -> NSDragOperation;

        #[optional]
        #[method(pathControl:acceptDrop:)]
        pub unsafe fn pathControl_acceptDrop(
            &self,
            pathControl: &NSPathControl,
            info: &NSDraggingInfo,
        ) -> bool;

        #[optional]
        #[method(pathControl:willDisplayOpenPanel:)]
        pub unsafe fn pathControl_willDisplayOpenPanel(
            &self,
            pathControl: &NSPathControl,
            openPanel: &NSOpenPanel,
        );

        #[optional]
        #[method(pathControl:willPopUpMenu:)]
        pub unsafe fn pathControl_willPopUpMenu(&self, pathControl: &NSPathControl, menu: &NSMenu);
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSPathControl {
        #[method_id(@__retain_semantics Other clickedPathComponentCell)]
        pub unsafe fn clickedPathComponentCell(&self) -> Option<Id<NSPathComponentCell, Shared>>;

        #[method_id(@__retain_semantics Other pathComponentCells)]
        pub unsafe fn pathComponentCells(&self) -> Id<NSArray<NSPathComponentCell>, Shared>;

        #[method(setPathComponentCells:)]
        pub unsafe fn setPathComponentCells(&self, cells: &NSArray<NSPathComponentCell>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    unsafe impl NSPathControl {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
