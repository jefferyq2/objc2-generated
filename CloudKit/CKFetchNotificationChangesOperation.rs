//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchNotificationChangesOperation")]
    #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
    pub struct CKFetchNotificationChangesOperation;

    #[cfg(feature = "CloudKit_CKFetchNotificationChangesOperation")]
    unsafe impl ClassType for CKFetchNotificationChangesOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKFetchNotificationChangesOperation")]
unsafe impl NSObjectProtocol for CKFetchNotificationChangesOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchNotificationChangesOperation")]
    unsafe impl CKFetchNotificationChangesOperation {
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method_id(@__retain_semantics Init initWithPreviousServerChangeToken:)]
        pub unsafe fn initWithPreviousServerChangeToken(
            this: Allocated<Self>,
            previous_server_change_token: Option<&CKServerChangeToken>,
        ) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method_id(@__retain_semantics Other previousServerChangeToken)]
        pub unsafe fn previousServerChangeToken(&self) -> Option<Id<CKServerChangeToken>>;

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(setPreviousServerChangeToken:)]
        pub unsafe fn setPreviousServerChangeToken(
            &self,
            previous_server_change_token: Option<&CKServerChangeToken>,
        );

        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(resultsLimit)]
        pub unsafe fn resultsLimit(&self) -> NSUInteger;

        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(setResultsLimit:)]
        pub unsafe fn setResultsLimit(&self, results_limit: NSUInteger);

        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(moreComing)]
        pub unsafe fn moreComing(&self) -> bool;

        #[cfg(feature = "CloudKit_CKNotification")]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(notificationChangedBlock)]
        pub unsafe fn notificationChangedBlock(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<CKNotification>)>;

        #[cfg(feature = "CloudKit_CKNotification")]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(setNotificationChangedBlock:)]
        pub unsafe fn setNotificationChangedBlock(
            &self,
            notification_changed_block: Option<&Block<dyn Fn(NonNull<CKNotification>)>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKServerChangeToken",
            feature = "Foundation_NSError"
        ))]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(fetchNotificationChangesCompletionBlock)]
        pub unsafe fn fetchNotificationChangesCompletionBlock(
            &self,
        ) -> *mut Block<dyn Fn(*mut CKServerChangeToken, *mut NSError)>;

        #[cfg(all(
            feature = "CloudKit_CKServerChangeToken",
            feature = "Foundation_NSError"
        ))]
        #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
        #[method(setFetchNotificationChangesCompletionBlock:)]
        pub unsafe fn setFetchNotificationChangesCompletionBlock(
            &self,
            fetch_notification_changes_completion_block: Option<
                &Block<dyn Fn(*mut CKServerChangeToken, *mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CloudKit_CKFetchNotificationChangesOperation")]
    unsafe impl CKFetchNotificationChangesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
