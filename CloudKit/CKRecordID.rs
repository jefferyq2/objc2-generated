//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKRecordID")]
    pub struct CKRecordID;

    #[cfg(feature = "CloudKit_CKRecordID")]
    unsafe impl ClassType for CKRecordID {
        type Super = NSObject;
    }
);

#[cfg(feature = "CloudKit_CKRecordID")]
unsafe impl NSCoding for CKRecordID {}

#[cfg(feature = "CloudKit_CKRecordID")]
unsafe impl NSObjectProtocol for CKRecordID {}

#[cfg(feature = "CloudKit_CKRecordID")]
unsafe impl NSSecureCoding for CKRecordID {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKRecordID")]
    unsafe impl CKRecordID {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithRecordName:)]
        pub unsafe fn initWithRecordName(
            this: Option<Allocated<Self>>,
            record_name: &NSString,
        ) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithRecordName:zoneID:)]
        pub unsafe fn initWithRecordName_zoneID(
            this: Option<Allocated<Self>>,
            record_name: &NSString,
            zone_id: &CKRecordZoneID,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other recordName)]
        pub unsafe fn recordName(&self) -> Id<NSString>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Id<CKRecordZoneID>;
    }
);
