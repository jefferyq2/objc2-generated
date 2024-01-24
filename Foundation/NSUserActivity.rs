//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSUserActivityPersistentIdentifier = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUserActivity")]
    pub struct NSUserActivity;

    #[cfg(feature = "Foundation_NSUserActivity")]
    unsafe impl ClassType for NSUserActivity {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSUserActivity")]
unsafe impl NSObjectProtocol for NSUserActivity {}

extern_methods!(
    #[cfg(feature = "Foundation_NSUserActivity")]
    unsafe impl NSUserActivity {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithActivityType:)]
        pub unsafe fn initWithActivityType(
            this: Allocated<Self>,
            activity_type: &NSString,
        ) -> Id<Self>;

        #[deprecated = "Use initWithActivityType: with a specific activity type string"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other activityType)]
        pub unsafe fn activityType(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(addUserInfoEntriesFromDictionary:)]
        pub unsafe fn addUserInfoEntriesFromDictionary(&self, other_dictionary: &NSDictionary);

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other requiredUserInfoKeys)]
        pub unsafe fn requiredUserInfoKeys(&self) -> Option<Id<NSSet<NSString>>>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method(setRequiredUserInfoKeys:)]
        pub unsafe fn setRequiredUserInfoKeys(
            &self,
            required_user_info_keys: Option<&NSSet<NSString>>,
        );

        #[method(needsSave)]
        pub unsafe fn needsSave(&self) -> bool;

        #[method(setNeedsSave:)]
        pub unsafe fn setNeedsSave(&self, needs_save: bool);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other webpageURL)]
        pub unsafe fn webpageURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setWebpageURL:)]
        pub unsafe fn setWebpageURL(&self, webpage_url: Option<&NSURL>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other referrerURL)]
        pub unsafe fn referrerURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setReferrerURL:)]
        pub unsafe fn setReferrerURL(&self, referrer_url: Option<&NSURL>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other expirationDate)]
        pub unsafe fn expirationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setExpirationDate:)]
        pub unsafe fn setExpirationDate(&self, expiration_date: Option<&NSDate>);

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other keywords)]
        pub unsafe fn keywords(&self) -> Id<NSSet<NSString>>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method(setKeywords:)]
        pub unsafe fn setKeywords(&self, keywords: &NSSet<NSString>);

        #[method(supportsContinuationStreams)]
        pub unsafe fn supportsContinuationStreams(&self) -> bool;

        #[method(setSupportsContinuationStreams:)]
        pub unsafe fn setSupportsContinuationStreams(&self, supports_continuation_streams: bool);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSUserActivityDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSUserActivityDelegate>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other targetContentIdentifier)]
        pub unsafe fn targetContentIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTargetContentIdentifier:)]
        pub unsafe fn setTargetContentIdentifier(
            &self,
            target_content_identifier: Option<&NSString>,
        );

        #[method(becomeCurrent)]
        pub unsafe fn becomeCurrent(&self);

        #[method(resignCurrent)]
        pub unsafe fn resignCurrent(&self);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSInputStream",
            feature = "Foundation_NSOutputStream"
        ))]
        #[method(getContinuationStreamsWithCompletionHandler:)]
        pub unsafe fn getContinuationStreamsWithCompletionHandler(
            &self,
            completion_handler: &Block<
                dyn Fn(*mut NSInputStream, *mut NSOutputStream, *mut NSError),
            >,
        );

        #[method(isEligibleForHandoff)]
        pub unsafe fn isEligibleForHandoff(&self) -> bool;

        #[method(setEligibleForHandoff:)]
        pub unsafe fn setEligibleForHandoff(&self, eligible_for_handoff: bool);

        #[method(isEligibleForSearch)]
        pub unsafe fn isEligibleForSearch(&self) -> bool;

        #[method(setEligibleForSearch:)]
        pub unsafe fn setEligibleForSearch(&self, eligible_for_search: bool);

        #[method(isEligibleForPublicIndexing)]
        pub unsafe fn isEligibleForPublicIndexing(&self) -> bool;

        #[method(setEligibleForPublicIndexing:)]
        pub unsafe fn setEligibleForPublicIndexing(&self, eligible_for_public_indexing: bool);

        #[method(isEligibleForPrediction)]
        pub unsafe fn isEligibleForPrediction(&self) -> bool;

        #[method(setEligibleForPrediction:)]
        pub unsafe fn setEligibleForPrediction(&self, eligible_for_prediction: bool);

        #[method_id(@__retain_semantics Other persistentIdentifier)]
        pub unsafe fn persistentIdentifier(&self)
            -> Option<Id<NSUserActivityPersistentIdentifier>>;

        #[method(setPersistentIdentifier:)]
        pub unsafe fn setPersistentIdentifier(
            &self,
            persistent_identifier: Option<&NSUserActivityPersistentIdentifier>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(deleteSavedUserActivitiesWithPersistentIdentifiers:completionHandler:)]
        pub unsafe fn deleteSavedUserActivitiesWithPersistentIdentifiers_completionHandler(
            persistent_identifiers: &NSArray<NSUserActivityPersistentIdentifier>,
            handler: &Block<dyn Fn()>,
        );

        #[method(deleteAllSavedUserActivitiesWithCompletionHandler:)]
        pub unsafe fn deleteAllSavedUserActivitiesWithCompletionHandler(handler: &Block<dyn Fn()>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSUserActivity")]
    unsafe impl NSUserActivity {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSUserActivityTypeBrowsingWeb: &'static NSString);

extern_protocol!(
    pub unsafe trait NSUserActivityDelegate: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSUserActivity")]
        #[optional]
        #[method(userActivityWillSave:)]
        unsafe fn userActivityWillSave(&self, user_activity: &NSUserActivity);

        #[cfg(feature = "Foundation_NSUserActivity")]
        #[optional]
        #[method(userActivityWasContinued:)]
        unsafe fn userActivityWasContinued(&self, user_activity: &NSUserActivity);

        #[cfg(all(
            feature = "Foundation_NSInputStream",
            feature = "Foundation_NSOutputStream",
            feature = "Foundation_NSUserActivity"
        ))]
        #[optional]
        #[method(userActivity:didReceiveInputStream:outputStream:)]
        unsafe fn userActivity_didReceiveInputStream_outputStream(
            &self,
            user_activity: &NSUserActivity,
            input_stream: &NSInputStream,
            output_stream: &NSOutputStream,
        );
    }

    unsafe impl ProtocolType for dyn NSUserActivityDelegate {}
);
