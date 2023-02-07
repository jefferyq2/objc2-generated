//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptCommandDescription")]
    pub struct NSScriptCommandDescription;

    #[cfg(feature = "Foundation_NSScriptCommandDescription")]
    unsafe impl ClassType for NSScriptCommandDescription {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSScriptCommandDescription")]
unsafe impl NSCoding for NSScriptCommandDescription {}

#[cfg(feature = "Foundation_NSScriptCommandDescription")]
unsafe impl NSObjectProtocol for NSScriptCommandDescription {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptCommandDescription")]
    unsafe impl NSScriptCommandDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithSuiteName:commandName:dictionary:)]
        pub unsafe fn initWithSuiteName_commandName_dictionary(
            this: Option<Allocated<Self>>,
            suite_name: &NSString,
            command_name: &NSString,
            command_declaration: Option<&NSDictionary>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            in_coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other suiteName)]
        pub unsafe fn suiteName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other commandName)]
        pub unsafe fn commandName(&self) -> Id<NSString>;

        #[method(appleEventClassCode)]
        pub unsafe fn appleEventClassCode(&self) -> FourCharCode;

        #[method(appleEventCode)]
        pub unsafe fn appleEventCode(&self) -> FourCharCode;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other commandClassName)]
        pub unsafe fn commandClassName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other returnType)]
        pub unsafe fn returnType(&self) -> Option<Id<NSString>>;

        #[method(appleEventCodeForReturnType)]
        pub unsafe fn appleEventCodeForReturnType(&self) -> FourCharCode;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other argumentNames)]
        pub unsafe fn argumentNames(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other typeForArgumentWithName:)]
        pub unsafe fn typeForArgumentWithName(
            &self,
            argument_name: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(appleEventCodeForArgumentWithName:)]
        pub unsafe fn appleEventCodeForArgumentWithName(
            &self,
            argument_name: &NSString,
        ) -> FourCharCode;

        #[cfg(feature = "Foundation_NSString")]
        #[method(isOptionalArgumentWithName:)]
        pub unsafe fn isOptionalArgumentWithName(&self, argument_name: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSScriptCommand")]
        #[method_id(@__retain_semantics Other createCommandInstance)]
        pub unsafe fn createCommandInstance(&self) -> Id<NSScriptCommand>;

        #[cfg(feature = "Foundation_NSScriptCommand")]
        #[method_id(@__retain_semantics Other createCommandInstanceWithZone:)]
        pub unsafe fn createCommandInstanceWithZone(
            &self,
            zone: *mut NSZone,
        ) -> Id<NSScriptCommand>;
    }
);
