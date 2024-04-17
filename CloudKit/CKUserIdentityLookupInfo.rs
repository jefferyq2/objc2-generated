//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKUserIdentityLookupInfo;

    unsafe impl ClassType for CKUserIdentityLookupInfo {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CKUserIdentityLookupInfo {}

unsafe impl NSCopying for CKUserIdentityLookupInfo {}

unsafe impl NSObjectProtocol for CKUserIdentityLookupInfo {}

unsafe impl NSSecureCoding for CKUserIdentityLookupInfo {}

extern_methods!(
    unsafe impl CKUserIdentityLookupInfo {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithEmailAddress:)]
        pub unsafe fn initWithEmailAddress(
            this: Allocated<Self>,
            email_address: &NSString,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithPhoneNumber:)]
        pub unsafe fn initWithPhoneNumber(
            this: Allocated<Self>,
            phone_number: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Init initWithUserRecordID:)]
        pub unsafe fn initWithUserRecordID(
            this: Allocated<Self>,
            user_record_id: &CKRecordID,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other lookupInfosWithEmails:)]
        pub unsafe fn lookupInfosWithEmails(
            emails: &NSArray<NSString>,
        ) -> Id<NSArray<CKUserIdentityLookupInfo>>;

        #[method_id(@__retain_semantics Other lookupInfosWithPhoneNumbers:)]
        pub unsafe fn lookupInfosWithPhoneNumbers(
            phone_numbers: &NSArray<NSString>,
        ) -> Id<NSArray<CKUserIdentityLookupInfo>>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other lookupInfosWithRecordIDs:)]
        pub unsafe fn lookupInfosWithRecordIDs(
            record_i_ds: &NSArray<CKRecordID>,
        ) -> Id<NSArray<CKUserIdentityLookupInfo>>;

        #[method_id(@__retain_semantics Other emailAddress)]
        pub unsafe fn emailAddress(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other phoneNumber)]
        pub unsafe fn phoneNumber(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "CKRecordID")]
        #[method_id(@__retain_semantics Other userRecordID)]
        pub unsafe fn userRecordID(&self) -> Option<Id<CKRecordID>>;
    }
);
