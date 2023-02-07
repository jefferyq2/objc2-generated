//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchRecordsOperation")]
    pub struct CKFetchRecordsOperation;

    #[cfg(feature = "CloudKit_CKFetchRecordsOperation")]
    unsafe impl ClassType for CKFetchRecordsOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
    }
);

#[cfg(feature = "CloudKit_CKFetchRecordsOperation")]
unsafe impl NSObjectProtocol for CKFetchRecordsOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchRecordsOperation")]
    unsafe impl CKFetchRecordsOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Init initWithRecordIDs:)]
        pub unsafe fn initWithRecordIDs(
            this: Option<Allocated<Self>>,
            record_i_ds: &NSArray<CKRecordID>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other fetchCurrentUserRecordOperation)]
        pub unsafe fn fetchCurrentUserRecordOperation() -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other recordIDs)]
        pub unsafe fn recordIDs(&self) -> Option<Id<NSArray<CKRecordID>>>;

        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSArray"))]
        #[method(setRecordIDs:)]
        pub unsafe fn setRecordIDs(&self, record_i_ds: Option<&NSArray<CKRecordID>>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other desiredKeys)]
        pub unsafe fn desiredKeys(&self) -> Option<Id<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setDesiredKeys:)]
        pub unsafe fn setDesiredKeys(&self, desired_keys: Option<&NSArray<CKRecordFieldKey>>);

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method(perRecordProgressBlock)]
        pub unsafe fn perRecordProgressBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordID>, c_double), ()>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method(setPerRecordProgressBlock:)]
        pub unsafe fn setPerRecordProgressBlock(
            &self,
            per_record_progress_block: Option<&Block<(NonNull<CKRecordID>, c_double), ()>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKRecordID",
            feature = "Foundation_NSError"
        ))]
        #[method(perRecordCompletionBlock)]
        pub unsafe fn perRecordCompletionBlock(
            &self,
        ) -> *mut Block<(*mut CKRecord, *mut CKRecordID, *mut NSError), ()>;

        #[cfg(all(
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKRecordID",
            feature = "Foundation_NSError"
        ))]
        #[method(setPerRecordCompletionBlock:)]
        pub unsafe fn setPerRecordCompletionBlock(
            &self,
            per_record_completion_block: Option<
                &Block<(*mut CKRecord, *mut CKRecordID, *mut NSError), ()>,
            >,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKRecordID",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchRecordsCompletionBlock)]
        pub unsafe fn fetchRecordsCompletionBlock(
            &self,
        ) -> *mut Block<(*mut NSDictionary<CKRecordID, CKRecord>, *mut NSError), ()>;

        #[cfg(all(
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKRecordID",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method(setFetchRecordsCompletionBlock:)]
        pub unsafe fn setFetchRecordsCompletionBlock(
            &self,
            fetch_records_completion_block: Option<
                &Block<(*mut NSDictionary<CKRecordID, CKRecord>, *mut NSError), ()>,
            >,
        );
    }
);
