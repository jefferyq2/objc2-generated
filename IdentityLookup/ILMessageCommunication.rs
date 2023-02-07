//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILMessageCommunication")]
    pub struct ILMessageCommunication;

    #[cfg(feature = "IdentityLookup_ILMessageCommunication")]
    unsafe impl ClassType for ILMessageCommunication {
        #[inherits(NSObject)]
        type Super = ILCommunication;
    }
);

#[cfg(feature = "IdentityLookup_ILMessageCommunication")]
unsafe impl NSCoding for ILMessageCommunication {}

#[cfg(feature = "IdentityLookup_ILMessageCommunication")]
unsafe impl NSObjectProtocol for ILMessageCommunication {}

#[cfg(feature = "IdentityLookup_ILMessageCommunication")]
unsafe impl NSSecureCoding for ILMessageCommunication {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILMessageCommunication")]
    unsafe impl ILMessageCommunication {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other messageBody)]
        pub unsafe fn messageBody(&self) -> Option<Id<NSString>>;

        #[method(isEqualToMessageCommunication:)]
        pub unsafe fn isEqualToMessageCommunication(
            &self,
            communication: &ILMessageCommunication,
        ) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
