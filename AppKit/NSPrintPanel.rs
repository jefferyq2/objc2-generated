//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPrintPanelResult {
        NSPrintPanelResultCancelled = 0,
        NSPrintPanelResultPrinted = 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPrintPanelOptions {
        NSPrintPanelShowsCopies = 1 << 0,
        NSPrintPanelShowsPageRange = 1 << 1,
        NSPrintPanelShowsPaperSize = 1 << 2,
        NSPrintPanelShowsOrientation = 1 << 3,
        NSPrintPanelShowsScaling = 1 << 4,
        NSPrintPanelShowsPrintSelection = 1 << 5,
        NSPrintPanelShowsPageSetupAccessory = 1 << 8,
        NSPrintPanelShowsPreview = 1 << 17,
    }
);

typed_enum!(
    pub type NSPrintPanelJobStyleHint = NSString;
);

extern_static!(NSPrintPhotoJobStyleHint: &'static NSPrintPanelJobStyleHint);

extern_static!(NSPrintAllPresetsJobStyleHint: &'static NSPrintPanelJobStyleHint);

extern_static!(NSPrintNoPresetsJobStyleHint: &'static NSPrintPanelJobStyleHint);

typed_enum!(
    pub type NSPrintPanelAccessorySummaryKey = NSString;
);

extern_static!(NSPrintPanelAccessorySummaryItemNameKey: &'static NSPrintPanelAccessorySummaryKey);

extern_static!(NSPrintPanelAccessorySummaryItemDescriptionKey: &'static NSPrintPanelAccessorySummaryKey);

extern_protocol!(
    pub unsafe trait NSPrintPanelAccessorizing: IsMainThreadOnly {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other localizedSummaryItems)]
        unsafe fn localizedSummaryItems(
            &self,
        ) -> Id<NSArray<NSDictionary<NSPrintPanelAccessorySummaryKey, NSString>>>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other keyPathsForValuesAffectingPreview)]
        unsafe fn keyPathsForValuesAffectingPreview(&self) -> Id<NSSet<NSString>>;
    }

    unsafe impl ProtocolType for dyn NSPrintPanelAccessorizing {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPrintPanel")]
    pub struct NSPrintPanel;

    #[cfg(feature = "AppKit_NSPrintPanel")]
    unsafe impl ClassType for NSPrintPanel {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSPrintPanel")]
unsafe impl NSObjectProtocol for NSPrintPanel {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPrintPanel")]
    unsafe impl NSPrintPanel {
        #[method_id(@__retain_semantics Other printPanel)]
        pub unsafe fn printPanel(mtm: MainThreadMarker) -> Id<NSPrintPanel>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(addAccessoryController:)]
        pub unsafe fn addAccessoryController(&self, accessory_controller: &NSViewController);

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(removeAccessoryController:)]
        pub unsafe fn removeAccessoryController(&self, accessory_controller: &NSViewController);

        #[cfg(all(feature = "AppKit_NSViewController", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other accessoryControllers)]
        pub unsafe fn accessoryControllers(&self) -> Id<NSArray<NSViewController>>;

        #[method(options)]
        pub unsafe fn options(&self) -> NSPrintPanelOptions;

        #[method(setOptions:)]
        pub unsafe fn setOptions(&self, options: NSPrintPanelOptions);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDefaultButtonTitle:)]
        pub unsafe fn setDefaultButtonTitle(&self, default_button_title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultButtonTitle)]
        pub unsafe fn defaultButtonTitle(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other helpAnchor)]
        pub unsafe fn helpAnchor(&self) -> Option<Id<NSHelpAnchorName>>;

        #[method(setHelpAnchor:)]
        pub unsafe fn setHelpAnchor(&self, help_anchor: Option<&NSHelpAnchorName>);

        #[method_id(@__retain_semantics Other jobStyleHint)]
        pub unsafe fn jobStyleHint(&self) -> Option<Id<NSPrintPanelJobStyleHint>>;

        #[method(setJobStyleHint:)]
        pub unsafe fn setJobStyleHint(&self, job_style_hint: Option<&NSPrintPanelJobStyleHint>);

        #[cfg(all(feature = "AppKit_NSPrintInfo", feature = "AppKit_NSWindow"))]
        #[method(beginSheetUsingPrintInfo:onWindow:completionHandler:)]
        pub unsafe fn beginSheetUsingPrintInfo_onWindow_completionHandler(
            &self,
            print_info: &NSPrintInfo,
            parent_window: &NSWindow,
            handler: Option<&Block<dyn Fn(NSPrintPanelResult)>>,
        );

        #[cfg(all(feature = "AppKit_NSPrintInfo", feature = "AppKit_NSWindow"))]
        #[deprecated]
        #[method(beginSheetWithPrintInfo:modalForWindow:delegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetWithPrintInfo_modalForWindow_delegate_didEndSelector_contextInfo(
            &self,
            print_info: &NSPrintInfo,
            doc_window: &NSWindow,
            delegate: Option<&AnyObject>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method(runModalWithPrintInfo:)]
        pub unsafe fn runModalWithPrintInfo(&self, print_info: &NSPrintInfo) -> NSInteger;

        #[method(runModal)]
        pub unsafe fn runModal(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method_id(@__retain_semantics Other printInfo)]
        pub unsafe fn printInfo(&self) -> Id<NSPrintInfo>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSPrintPanel")]
    unsafe impl NSPrintPanel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSPrintPanel")]
    unsafe impl NSPrintPanel {
        #[cfg(feature = "AppKit_NSView")]
        #[deprecated = "Use -addAccessoryController instead"]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[cfg(feature = "AppKit_NSView")]
        #[deprecated = "Use -accessoryControllers instead. For compatibility this returns the view of the first accessory controller, or nil"]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView>>;

        #[deprecated]
        #[method(updateFromPrintInfo)]
        pub unsafe fn updateFromPrintInfo(&self);

        #[deprecated]
        #[method(finalWritePrintInfo)]
        pub unsafe fn finalWritePrintInfo(&self);
    }
);
