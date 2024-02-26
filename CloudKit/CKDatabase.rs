//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CKDatabaseScope {
        #[doc(alias = "CKDatabaseScopePublic")]
        Public = 1,
        #[doc(alias = "CKDatabaseScopePrivate")]
        Private = 2,
        #[doc(alias = "CKDatabaseScopeShared")]
        Shared = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKDatabase")]
    pub struct CKDatabase;

    #[cfg(feature = "CloudKit_CKDatabase")]
    unsafe impl ClassType for CKDatabase {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKDatabase")]
unsafe impl Send for CKDatabase {}

#[cfg(feature = "CloudKit_CKDatabase")]
unsafe impl Sync for CKDatabase {}

#[cfg(feature = "CloudKit_CKDatabase")]
unsafe impl NSObjectProtocol for CKDatabase {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKDatabase")]
    unsafe impl CKDatabase {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "CloudKit_CKDatabaseOperation")]
        #[method(addOperation:)]
        pub unsafe fn addOperation(&self, operation: &CKDatabaseOperation);

        #[method(databaseScope)]
        pub unsafe fn databaseScope(&self) -> CKDatabaseScope;
    }
);

extern_methods!(
    /// ConvenienceMethods
    #[cfg(feature = "CloudKit_CKDatabase")]
    unsafe impl CKDatabase {
        #[cfg(all(
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKRecordID",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchRecordWithID:completionHandler:)]
        pub unsafe fn fetchRecordWithID_completionHandler(
            &self,
            record_id: &CKRecordID,
            completion_handler: &Block<dyn Fn(*mut CKRecord, *mut NSError)>,
        );

        #[cfg(all(feature = "CloudKit_CKRecord", feature = "Foundation_NSError"))]
        #[method(saveRecord:completionHandler:)]
        pub unsafe fn saveRecord_completionHandler(
            &self,
            record: &CKRecord,
            completion_handler: &Block<dyn Fn(*mut CKRecord, *mut NSError)>,
        );

        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSError"))]
        #[method(deleteRecordWithID:completionHandler:)]
        pub unsafe fn deleteRecordWithID_completionHandler(
            &self,
            record_id: &CKRecordID,
            completion_handler: &Block<dyn Fn(*mut CKRecordID, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKQuery",
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(performQuery:inZoneWithID:completionHandler:)]
        pub unsafe fn performQuery_inZoneWithID_completionHandler(
            &self,
            query: &CKQuery,
            zone_id: Option<&CKRecordZoneID>,
            completion_handler: &Block<dyn Fn(*mut NSArray<CKRecord>, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchAllRecordZonesWithCompletionHandler:)]
        pub unsafe fn fetchAllRecordZonesWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSArray<CKRecordZone>, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordZone",
            feature = "CloudKit_CKRecordZoneID",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchRecordZoneWithID:completionHandler:)]
        pub unsafe fn fetchRecordZoneWithID_completionHandler(
            &self,
            zone_id: &CKRecordZoneID,
            completion_handler: &Block<dyn Fn(*mut CKRecordZone, *mut NSError)>,
        );

        #[cfg(all(feature = "CloudKit_CKRecordZone", feature = "Foundation_NSError"))]
        #[method(saveRecordZone:completionHandler:)]
        pub unsafe fn saveRecordZone_completionHandler(
            &self,
            zone: &CKRecordZone,
            completion_handler: &Block<dyn Fn(*mut CKRecordZone, *mut NSError)>,
        );

        #[cfg(all(feature = "CloudKit_CKRecordZoneID", feature = "Foundation_NSError"))]
        #[method(deleteRecordZoneWithID:completionHandler:)]
        pub unsafe fn deleteRecordZoneWithID_completionHandler(
            &self,
            zone_id: &CKRecordZoneID,
            completion_handler: &Block<dyn Fn(*mut CKRecordZoneID, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(fetchSubscriptionWithID:completionHandler:)]
        pub unsafe fn fetchSubscriptionWithID_completionHandler(
            &self,
            subscription_id: &CKSubscriptionID,
            completion_handler: &Block<dyn Fn(*mut CKSubscription, *mut NSError)>,
        );

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchAllSubscriptionsWithCompletionHandler:)]
        pub unsafe fn fetchAllSubscriptionsWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSArray<CKSubscription>, *mut NSError)>,
        );

        #[cfg(all(feature = "CloudKit_CKSubscription", feature = "Foundation_NSError"))]
        #[method(saveSubscription:completionHandler:)]
        pub unsafe fn saveSubscription_completionHandler(
            &self,
            subscription: &CKSubscription,
            completion_handler: &Block<dyn Fn(*mut CKSubscription, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(deleteSubscriptionWithID:completionHandler:)]
        pub unsafe fn deleteSubscriptionWithID_completionHandler(
            &self,
            subscription_id: &CKSubscriptionID,
            completion_handler: &Block<dyn Fn(*mut CKSubscriptionID, *mut NSError)>,
        );
    }
);
