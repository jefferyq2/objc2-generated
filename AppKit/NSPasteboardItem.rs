//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPasteboardItem")]
    pub struct NSPasteboardItem;

    #[cfg(feature = "AppKit_NSPasteboardItem")]
    unsafe impl ClassType for NSPasteboardItem {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSPasteboardItem")]
unsafe impl NSObjectProtocol for NSPasteboardItem {}

#[cfg(feature = "AppKit_NSPasteboardItem")]
unsafe impl NSPasteboardReading for NSPasteboardItem {}

#[cfg(feature = "AppKit_NSPasteboardItem")]
unsafe impl NSPasteboardWriting for NSPasteboardItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPasteboardItem")]
    unsafe impl NSPasteboardItem {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other types)]
        pub unsafe fn types(&self) -> Id<NSArray<NSPasteboardType>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other availableTypeFromArray:)]
        pub unsafe fn availableTypeFromArray(
            &self,
            types: &NSArray<NSPasteboardType>,
        ) -> Option<Id<NSPasteboardType>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setDataProvider:forTypes:)]
        pub unsafe fn setDataProvider_forTypes(
            &self,
            data_provider: &ProtocolObject<dyn NSPasteboardItemDataProvider>,
            types: &NSArray<NSPasteboardType>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setData:forType:)]
        pub unsafe fn setData_forType(&self, data: &NSData, r#type: &NSPasteboardType) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setString:forType:)]
        pub unsafe fn setString_forType(
            &self,
            string: &NSString,
            r#type: &NSPasteboardType,
        ) -> bool;

        #[method(setPropertyList:forType:)]
        pub unsafe fn setPropertyList_forType(
            &self,
            property_list: &Object,
            r#type: &NSPasteboardType,
        ) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other dataForType:)]
        pub unsafe fn dataForType(&self, r#type: &NSPasteboardType) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringForType:)]
        pub unsafe fn stringForType(&self, r#type: &NSPasteboardType) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other propertyListForType:)]
        pub unsafe fn propertyListForType(&self, r#type: &NSPasteboardType) -> Option<Id<Object>>;
    }
);

extern_protocol!(
    pub unsafe trait NSPasteboardItemDataProvider: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSPasteboard", feature = "AppKit_NSPasteboardItem"))]
        #[method(pasteboard:item:provideDataForType:)]
        unsafe fn pasteboard_item_provideDataForType(
            &self,
            pasteboard: Option<&NSPasteboard>,
            item: &NSPasteboardItem,
            r#type: &NSPasteboardType,
        );

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[optional]
        #[method(pasteboardFinishedWithDataProvider:)]
        unsafe fn pasteboardFinishedWithDataProvider(&self, pasteboard: &NSPasteboard);
    }

    unsafe impl ProtocolType for dyn NSPasteboardItemDataProvider {}
);
