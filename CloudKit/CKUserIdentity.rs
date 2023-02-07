//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKUserIdentity")]
    pub struct CKUserIdentity;

    #[cfg(feature = "CloudKit_CKUserIdentity")]
    unsafe impl ClassType for CKUserIdentity {
        type Super = NSObject;
    }
);

#[cfg(feature = "CloudKit_CKUserIdentity")]
unsafe impl NSCoding for CKUserIdentity {}

#[cfg(feature = "CloudKit_CKUserIdentity")]
unsafe impl NSObjectProtocol for CKUserIdentity {}

#[cfg(feature = "CloudKit_CKUserIdentity")]
unsafe impl NSSecureCoding for CKUserIdentity {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKUserIdentity")]
    unsafe impl CKUserIdentity {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "CloudKit_CKUserIdentityLookupInfo")]
        #[method_id(@__retain_semantics Other lookupInfo)]
        pub unsafe fn lookupInfo(&self) -> Option<Id<CKUserIdentityLookupInfo>>;

        #[cfg(feature = "Foundation_NSPersonNameComponents")]
        #[method_id(@__retain_semantics Other nameComponents)]
        pub unsafe fn nameComponents(&self) -> Option<Id<NSPersonNameComponents>>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Other userRecordID)]
        pub unsafe fn userRecordID(&self) -> Option<Id<CKRecordID>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other contactIdentifiers)]
        pub unsafe fn contactIdentifiers(&self) -> Id<NSArray<NSString>>;

        #[method(hasiCloudAccount)]
        pub unsafe fn hasiCloudAccount(&self) -> bool;
    }
);
