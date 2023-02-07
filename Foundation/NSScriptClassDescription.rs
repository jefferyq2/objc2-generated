//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptClassDescription")]
    pub struct NSScriptClassDescription;

    #[cfg(feature = "Foundation_NSScriptClassDescription")]
    unsafe impl ClassType for NSScriptClassDescription {
        #[inherits(NSObject)]
        type Super = NSClassDescription;
    }
);

#[cfg(feature = "Foundation_NSScriptClassDescription")]
unsafe impl NSObjectProtocol for NSScriptClassDescription {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptClassDescription")]
    unsafe impl NSScriptClassDescription {
        #[method_id(@__retain_semantics Other classDescriptionForClass:)]
        pub unsafe fn classDescriptionForClass(
            a_class: &Class,
        ) -> Option<Id<NSScriptClassDescription>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithSuiteName:className:dictionary:)]
        pub unsafe fn initWithSuiteName_className_dictionary(
            this: Option<Allocated<Self>>,
            suite_name: &NSString,
            class_name: &NSString,
            class_declaration: Option<&NSDictionary>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other suiteName)]
        pub unsafe fn suiteName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other className)]
        pub unsafe fn className(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other implementationClassName)]
        pub unsafe fn implementationClassName(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other superclassDescription)]
        pub unsafe fn superclassDescription(&self) -> Option<Id<NSScriptClassDescription>>;

        #[method(appleEventCode)]
        pub unsafe fn appleEventCode(&self) -> FourCharCode;

        #[method(matchesAppleEventCode:)]
        pub unsafe fn matchesAppleEventCode(&self, apple_event_code: FourCharCode) -> bool;

        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method(supportsCommand:)]
        pub unsafe fn supportsCommand(
            &self,
            command_description: &NSScriptCommandDescription,
        ) -> bool;

        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method(selectorForCommand:)]
        pub unsafe fn selectorForCommand(
            &self,
            command_description: &NSScriptCommandDescription,
        ) -> Option<Sel>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other typeForKey:)]
        pub unsafe fn typeForKey(&self, key: &NSString) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other classDescriptionForKey:)]
        pub unsafe fn classDescriptionForKey(
            &self,
            key: &NSString,
        ) -> Option<Id<NSScriptClassDescription>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(appleEventCodeForKey:)]
        pub unsafe fn appleEventCodeForKey(&self, key: &NSString) -> FourCharCode;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other keyWithAppleEventCode:)]
        pub unsafe fn keyWithAppleEventCode(
            &self,
            apple_event_code: FourCharCode,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultSubcontainerAttributeKey)]
        pub unsafe fn defaultSubcontainerAttributeKey(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(isLocationRequiredToCreateForKey:)]
        pub unsafe fn isLocationRequiredToCreateForKey(
            &self,
            to_many_relationship_key: &NSString,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(hasPropertyForKey:)]
        pub unsafe fn hasPropertyForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(hasOrderedToManyRelationshipForKey:)]
        pub unsafe fn hasOrderedToManyRelationshipForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(hasReadablePropertyForKey:)]
        pub unsafe fn hasReadablePropertyForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(hasWritablePropertyForKey:)]
        pub unsafe fn hasWritablePropertyForKey(&self, key: &NSString) -> bool;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSScriptClassDescription")]
    unsafe impl NSScriptClassDescription {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(isReadOnlyKey:)]
        pub unsafe fn isReadOnlyKey(&self, key: &NSString) -> bool;
    }
);
