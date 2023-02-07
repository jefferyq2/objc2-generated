//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UniformTypeIdentifiers_UTType")]
    pub struct UTType;

    #[cfg(feature = "UniformTypeIdentifiers_UTType")]
    unsafe impl ClassType for UTType {
        type Super = NSObject;
    }
);

#[cfg(feature = "UniformTypeIdentifiers_UTType")]
unsafe impl NSCoding for UTType {}

#[cfg(feature = "UniformTypeIdentifiers_UTType")]
unsafe impl NSObjectProtocol for UTType {}

#[cfg(feature = "UniformTypeIdentifiers_UTType")]
unsafe impl NSSecureCoding for UTType {}

extern_methods!(
    #[cfg(feature = "UniformTypeIdentifiers_UTType")]
    unsafe impl UTType {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other typeWithIdentifier:)]
        pub unsafe fn typeWithIdentifier(identifier: &NSString) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other typeWithFilenameExtension:)]
        pub unsafe fn typeWithFilenameExtension(filename_extension: &NSString) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other typeWithFilenameExtension:conformingToType:)]
        pub unsafe fn typeWithFilenameExtension_conformingToType(
            filename_extension: &NSString,
            supertype: &UTType,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other typeWithMIMEType:)]
        pub unsafe fn typeWithMIMEType(mime_type: &NSString) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other typeWithMIMEType:conformingToType:)]
        pub unsafe fn typeWithMIMEType_conformingToType(
            mime_type: &NSString,
            supertype: &UTType,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other preferredFilenameExtension)]
        pub unsafe fn preferredFilenameExtension(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other preferredMIMEType)]
        pub unsafe fn preferredMIMEType(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other version)]
        pub unsafe fn version(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other referenceURL)]
        pub unsafe fn referenceURL(&self) -> Option<Id<NSURL>>;

        #[method(isDynamic)]
        pub unsafe fn isDynamic(&self) -> bool;

        #[method(isDeclared)]
        pub unsafe fn isDeclared(&self) -> bool;

        #[method(isPublicType)]
        pub unsafe fn isPublicType(&self) -> bool;
    }
);

extern_methods!(
    /// Conformance
    #[cfg(feature = "UniformTypeIdentifiers_UTType")]
    unsafe impl UTType {
        #[method(conformsToType:)]
        pub unsafe fn conformsToType(&self, r#type: &UTType) -> bool;

        #[method(isSupertypeOfType:)]
        pub unsafe fn isSupertypeOfType(&self, r#type: &UTType) -> bool;

        #[method(isSubtypeOfType:)]
        pub unsafe fn isSubtypeOfType(&self, r#type: &UTType) -> bool;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other supertypes)]
        pub unsafe fn supertypes(&self) -> Id<NSSet<UTType>>;
    }
);

extern_methods!(
    /// UTTagSpecification
    #[cfg(feature = "UniformTypeIdentifiers_UTType")]
    unsafe impl UTType {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other typeWithTag:tagClass:conformingToType:)]
        pub unsafe fn typeWithTag_tagClass_conformingToType(
            tag: &NSString,
            tag_class: &NSString,
            supertype: Option<&UTType>,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other typesWithTag:tagClass:conformingToType:)]
        pub unsafe fn typesWithTag_tagClass_conformingToType(
            tag: &NSString,
            tag_class: &NSString,
            supertype: Option<&UTType>,
        ) -> Id<NSArray<UTType>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other tags)]
        pub unsafe fn tags(&self) -> Id<NSDictionary<NSString, NSArray<NSString>>>;
    }
);

extern_methods!(
    /// LocalConstants
    #[cfg(feature = "UniformTypeIdentifiers_UTType")]
    unsafe impl UTType {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other exportedTypeWithIdentifier:)]
        pub unsafe fn exportedTypeWithIdentifier(identifier: &NSString) -> Id<UTType>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other exportedTypeWithIdentifier:conformingToType:)]
        pub unsafe fn exportedTypeWithIdentifier_conformingToType(
            identifier: &NSString,
            parent_type: &UTType,
        ) -> Id<UTType>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other importedTypeWithIdentifier:)]
        pub unsafe fn importedTypeWithIdentifier(identifier: &NSString) -> Id<UTType>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other importedTypeWithIdentifier:conformingToType:)]
        pub unsafe fn importedTypeWithIdentifier_conformingToType(
            identifier: &NSString,
            parent_type: &UTType,
        ) -> Id<UTType>;
    }
);
