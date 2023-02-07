//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_methods!(
    /// NSScripting
    #[cfg(feature = "AppKit_NSDocument")]
    unsafe impl NSDocument {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other lastComponentOfFileName)]
        pub unsafe fn lastComponentOfFileName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLastComponentOfFileName:)]
        pub unsafe fn setLastComponentOfFileName(&self, last_component_of_file_name: &NSString);

        #[cfg(feature = "Foundation_NSScriptCommand")]
        #[method_id(@__retain_semantics Other handleSaveScriptCommand:)]
        pub unsafe fn handleSaveScriptCommand(
            &self,
            command: &NSScriptCommand,
        ) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSCloseCommand")]
        #[method_id(@__retain_semantics Other handleCloseScriptCommand:)]
        pub unsafe fn handleCloseScriptCommand(
            &self,
            command: &NSCloseCommand,
        ) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSScriptCommand")]
        #[method_id(@__retain_semantics Other handlePrintScriptCommand:)]
        pub unsafe fn handlePrintScriptCommand(
            &self,
            command: &NSScriptCommand,
        ) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method_id(@__retain_semantics Other objectSpecifier)]
        pub unsafe fn objectSpecifier(&self) -> Id<NSScriptObjectSpecifier>;
    }
);
