// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `ClassKit` framework

#[cfg_attr(feature = "apple", link(name = "ClassKit", kind = "framework"))]
extern "C" {}

#[path = "CLSActivity.rs"]
mod __CLSActivity;
#[path = "CLSActivityItem.rs"]
mod __CLSActivityItem;
#[path = "CLSBinaryItem.rs"]
mod __CLSBinaryItem;
#[path = "CLSContext.rs"]
mod __CLSContext;
#[path = "CLSContextProvider.rs"]
mod __CLSContextProvider;
#[path = "CLSDataStore.rs"]
mod __CLSDataStore;
#[path = "CLSDefines.rs"]
mod __CLSDefines;
#[path = "CLSObject.rs"]
mod __CLSObject;
#[path = "CLSProgressReportingCapability.rs"]
mod __CLSProgressReportingCapability;
#[path = "CLSQuantityItem.rs"]
mod __CLSQuantityItem;
#[path = "CLSScoreItem.rs"]
mod __CLSScoreItem;
#[path = "NSUserActivity_CLSDeepLinks.rs"]
mod __NSUserActivity_CLSDeepLinks;

#[cfg(feature = "ClassKit_CLSActivity")]
pub use self::__CLSActivity::CLSActivity;
#[cfg(feature = "ClassKit_CLSActivityItem")]
pub use self::__CLSActivityItem::CLSActivityItem;
#[cfg(feature = "ClassKit_CLSBinaryItem")]
pub use self::__CLSBinaryItem::CLSBinaryItem;
pub use self::__CLSBinaryItem::{
    CLSBinaryValueType, CLSBinaryValueTypeCorrectIncorrect, CLSBinaryValueTypePassFail,
    CLSBinaryValueTypeTrueFalse, CLSBinaryValueTypeYesNo,
};
#[cfg(feature = "ClassKit_CLSContext")]
pub use self::__CLSContext::CLSContext;
pub use self::__CLSContext::CLSContextTopic;
pub use self::__CLSContext::CLSContextTopicArtsAndMusic;
pub use self::__CLSContext::CLSContextTopicComputerScienceAndEngineering;
pub use self::__CLSContext::CLSContextTopicHealthAndFitness;
pub use self::__CLSContext::CLSContextTopicLiteracyAndWriting;
pub use self::__CLSContext::CLSContextTopicMath;
pub use self::__CLSContext::CLSContextTopicScience;
pub use self::__CLSContext::CLSContextTopicSocialScience;
pub use self::__CLSContext::CLSContextTopicWorldLanguage;
pub use self::__CLSContext::{
    CLSContextType, CLSContextTypeApp, CLSContextTypeAudio, CLSContextTypeBook,
    CLSContextTypeChallenge, CLSContextTypeChapter, CLSContextTypeCourse, CLSContextTypeCustom,
    CLSContextTypeDocument, CLSContextTypeExercise, CLSContextTypeGame, CLSContextTypeLesson,
    CLSContextTypeLevel, CLSContextTypeNone, CLSContextTypePage, CLSContextTypeQuiz,
    CLSContextTypeSection, CLSContextTypeTask, CLSContextTypeVideo,
};
pub use self::__CLSContextProvider::CLSContextProvider;
#[cfg(feature = "ClassKit_CLSDataStore")]
pub use self::__CLSDataStore::CLSDataStore;
pub use self::__CLSDataStore::CLSDataStoreDelegate;
pub use self::__CLSDefines::CLSErrorCodeDomain;
pub use self::__CLSDefines::CLSErrorObjectKey;
pub use self::__CLSDefines::CLSErrorSuccessfulObjectsKey;
pub use self::__CLSDefines::CLSErrorUnderlyingErrorsKey;
pub use self::__CLSDefines::CLSErrorUserInfoKey;
pub use self::__CLSDefines::CLSPredicateKeyPath;
pub use self::__CLSDefines::CLSPredicateKeyPathDateCreated;
pub use self::__CLSDefines::CLSPredicateKeyPathIdentifier;
pub use self::__CLSDefines::CLSPredicateKeyPathParent;
pub use self::__CLSDefines::CLSPredicateKeyPathTitle;
pub use self::__CLSDefines::CLSPredicateKeyPathTopic;
pub use self::__CLSDefines::CLSPredicateKeyPathUniversalLinkURL;
pub use self::__CLSDefines::{
    CLSErrorCode, CLSErrorCodeAuthorizationDenied, CLSErrorCodeClassKitUnavailable,
    CLSErrorCodeDatabaseInaccessible, CLSErrorCodeInvalidAccountCredentials,
    CLSErrorCodeInvalidArgument, CLSErrorCodeInvalidCreate, CLSErrorCodeInvalidModification,
    CLSErrorCodeInvalidUpdate, CLSErrorCodeLimits, CLSErrorCodeNone, CLSErrorCodePartialFailure,
};
#[cfg(feature = "ClassKit_CLSObject")]
pub use self::__CLSObject::CLSObject;
#[cfg(feature = "ClassKit_CLSProgressReportingCapability")]
pub use self::__CLSProgressReportingCapability::CLSProgressReportingCapability;
pub use self::__CLSProgressReportingCapability::{
    CLSProgressReportingCapabilityKind, CLSProgressReportingCapabilityKindBinary,
    CLSProgressReportingCapabilityKindDuration, CLSProgressReportingCapabilityKindPercent,
    CLSProgressReportingCapabilityKindQuantity, CLSProgressReportingCapabilityKindScore,
};
#[cfg(feature = "ClassKit_CLSQuantityItem")]
pub use self::__CLSQuantityItem::CLSQuantityItem;
#[cfg(feature = "ClassKit_CLSScoreItem")]
pub use self::__CLSScoreItem::CLSScoreItem;
