//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CNContactFormatterStyle(pub NSInteger);
impl CNContactFormatterStyle {
    #[doc(alias = "CNContactFormatterStyleFullName")]
    pub const FullName: Self = Self(0);
    #[doc(alias = "CNContactFormatterStylePhoneticFullName")]
    pub const PhoneticFullName: Self = Self(1);
}

unsafe impl Encode for CNContactFormatterStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CNContactFormatterStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CNContactDisplayNameOrder(pub NSInteger);
impl CNContactDisplayNameOrder {
    #[doc(alias = "CNContactDisplayNameOrderUserDefault")]
    pub const UserDefault: Self = Self(0);
    #[doc(alias = "CNContactDisplayNameOrderGivenNameFirst")]
    pub const GivenNameFirst: Self = Self(1);
    #[doc(alias = "CNContactDisplayNameOrderFamilyNameFirst")]
    pub const FamilyNameFirst: Self = Self(2);
}

unsafe impl Encode for CNContactDisplayNameOrder {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CNContactDisplayNameOrder {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNContactFormatter;

    unsafe impl ClassType for CNContactFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CNContactFormatter {}

unsafe impl NSCopying for CNContactFormatter {}

unsafe impl NSObjectProtocol for CNContactFormatter {}

unsafe impl NSSecureCoding for CNContactFormatter {}

extern_methods!(
    unsafe impl CNContactFormatter {
        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other descriptorForRequiredKeysForStyle:)]
        pub unsafe fn descriptorForRequiredKeysForStyle(
            style: CNContactFormatterStyle,
        ) -> Id<ProtocolObject<dyn CNKeyDescriptor>>;

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other descriptorForRequiredKeysForNameOrder)]
        pub unsafe fn descriptorForRequiredKeysForNameOrder(
        ) -> Id<ProtocolObject<dyn CNKeyDescriptor>>;

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other descriptorForRequiredKeysForDelimiter)]
        pub unsafe fn descriptorForRequiredKeysForDelimiter(
        ) -> Id<ProtocolObject<dyn CNKeyDescriptor>>;

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other stringFromContact:style:)]
        pub unsafe fn stringFromContact_style(
            contact: &CNContact,
            style: CNContactFormatterStyle,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other attributedStringFromContact:style:defaultAttributes:)]
        pub unsafe fn attributedStringFromContact_style_defaultAttributes(
            contact: &CNContact,
            style: CNContactFormatterStyle,
            attributes: Option<&NSDictionary>,
        ) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "CNContact")]
        #[method(nameOrderForContact:)]
        pub unsafe fn nameOrderForContact(contact: &CNContact) -> CNContactDisplayNameOrder;

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other delimiterForContact:)]
        pub unsafe fn delimiterForContact(contact: &CNContact) -> Id<NSString>;

        #[method(style)]
        pub unsafe fn style(&self) -> CNContactFormatterStyle;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: CNContactFormatterStyle);

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other stringFromContact:)]
        pub unsafe fn stringFromContact(&self, contact: &CNContact) -> Option<Id<NSString>>;

        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other attributedStringFromContact:defaultAttributes:)]
        pub unsafe fn attributedStringFromContact_defaultAttributes(
            &self,
            contact: &CNContact,
            attributes: Option<&NSDictionary>,
        ) -> Option<Id<NSAttributedString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNContactFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    pub static CNContactPropertyAttribute: &'static NSString;
}
