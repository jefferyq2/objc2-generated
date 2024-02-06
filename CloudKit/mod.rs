// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `CloudKit` framework

#[link(name = "CloudKit", kind = "framework")]
extern "C" {}

#[path = "CKAcceptSharesOperation.rs"]
mod __CKAcceptSharesOperation;
#[path = "CKAllowedSharingOptions.rs"]
mod __CKAllowedSharingOptions;
#[path = "CKAsset.rs"]
mod __CKAsset;
#[path = "CKContainer.rs"]
mod __CKContainer;
#[path = "CKDatabase.rs"]
mod __CKDatabase;
#[path = "CKDatabaseOperation.rs"]
mod __CKDatabaseOperation;
#[path = "CKDefines.rs"]
mod __CKDefines;
#[path = "CKDiscoverAllUserIdentitiesOperation.rs"]
mod __CKDiscoverAllUserIdentitiesOperation;
#[path = "CKDiscoverUserIdentitiesOperation.rs"]
mod __CKDiscoverUserIdentitiesOperation;
#[path = "CKError.rs"]
mod __CKError;
#[path = "CKFetchDatabaseChangesOperation.rs"]
mod __CKFetchDatabaseChangesOperation;
#[path = "CKFetchNotificationChangesOperation.rs"]
mod __CKFetchNotificationChangesOperation;
#[path = "CKFetchRecordChangesOperation.rs"]
mod __CKFetchRecordChangesOperation;
#[path = "CKFetchRecordZoneChangesOperation.rs"]
mod __CKFetchRecordZoneChangesOperation;
#[path = "CKFetchRecordZonesOperation.rs"]
mod __CKFetchRecordZonesOperation;
#[path = "CKFetchRecordsOperation.rs"]
mod __CKFetchRecordsOperation;
#[path = "CKFetchShareMetadataOperation.rs"]
mod __CKFetchShareMetadataOperation;
#[path = "CKFetchShareParticipantsOperation.rs"]
mod __CKFetchShareParticipantsOperation;
#[path = "CKFetchSubscriptionsOperation.rs"]
mod __CKFetchSubscriptionsOperation;
#[path = "CKFetchWebAuthTokenOperation.rs"]
mod __CKFetchWebAuthTokenOperation;
#[path = "CKLocationSortDescriptor.rs"]
mod __CKLocationSortDescriptor;
#[path = "CKMarkNotificationsReadOperation.rs"]
mod __CKMarkNotificationsReadOperation;
#[path = "CKModifyBadgeOperation.rs"]
mod __CKModifyBadgeOperation;
#[path = "CKModifyRecordZonesOperation.rs"]
mod __CKModifyRecordZonesOperation;
#[path = "CKModifyRecordsOperation.rs"]
mod __CKModifyRecordsOperation;
#[path = "CKModifySubscriptionsOperation.rs"]
mod __CKModifySubscriptionsOperation;
#[path = "CKNotification.rs"]
mod __CKNotification;
#[path = "CKOperation.rs"]
mod __CKOperation;
#[path = "CKOperationGroup.rs"]
mod __CKOperationGroup;
#[path = "CKQuery.rs"]
mod __CKQuery;
#[path = "CKQueryOperation.rs"]
mod __CKQueryOperation;
#[path = "CKRecord.rs"]
mod __CKRecord;
#[path = "CKRecordID.rs"]
mod __CKRecordID;
#[path = "CKRecordZone.rs"]
mod __CKRecordZone;
#[path = "CKRecordZoneID.rs"]
mod __CKRecordZoneID;
#[path = "CKReference.rs"]
mod __CKReference;
#[path = "CKServerChangeToken.rs"]
mod __CKServerChangeToken;
#[path = "CKShare.rs"]
mod __CKShare;
#[path = "CKShareMetadata.rs"]
mod __CKShareMetadata;
#[path = "CKShareParticipant.rs"]
mod __CKShareParticipant;
#[path = "CKSubscription.rs"]
mod __CKSubscription;
#[path = "CKSyncEngine.rs"]
mod __CKSyncEngine;
#[path = "CKSyncEngineConfiguration.rs"]
mod __CKSyncEngineConfiguration;
#[path = "CKSyncEngineEvent.rs"]
mod __CKSyncEngineEvent;
#[path = "CKSyncEngineRecordZoneChangeBatch.rs"]
mod __CKSyncEngineRecordZoneChangeBatch;
#[path = "CKSyncEngineState.rs"]
mod __CKSyncEngineState;
#[path = "CKSystemSharingUIObserver.rs"]
mod __CKSystemSharingUIObserver;
#[path = "CKUserIdentity.rs"]
mod __CKUserIdentity;
#[path = "CKUserIdentityLookupInfo.rs"]
mod __CKUserIdentityLookupInfo;
#[path = "NSItemProvider_CKSharingSupport.rs"]
mod __NSItemProvider_CKSharingSupport;

#[cfg(feature = "CloudKit_CKAcceptSharesOperation")]
pub use self::__CKAcceptSharesOperation::CKAcceptSharesOperation;
#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
pub use self::__CKAllowedSharingOptions::CKAllowedSharingOptions;
pub use self::__CKAllowedSharingOptions::CKSharingParticipantAccessOption;
pub use self::__CKAllowedSharingOptions::CKSharingParticipantPermissionOption;
pub use self::__CKAllowedSharingOptions::{
    CKSharingParticipantAccessOptionAny, CKSharingParticipantAccessOptionAnyoneWithLink,
    CKSharingParticipantAccessOptionSpecifiedRecipientsOnly,
};
pub use self::__CKAllowedSharingOptions::{
    CKSharingParticipantPermissionOptionAny, CKSharingParticipantPermissionOptionReadOnly,
    CKSharingParticipantPermissionOptionReadWrite,
};
#[cfg(feature = "CloudKit_CKAsset")]
pub use self::__CKAsset::CKAsset;
pub use self::__CKContainer::CKAccountChangedNotification;
pub use self::__CKContainer::CKAccountStatus;
pub use self::__CKContainer::CKApplicationPermissionBlock;
pub use self::__CKContainer::CKApplicationPermissionStatus;
pub use self::__CKContainer::CKApplicationPermissionUserDiscoverability;
pub use self::__CKContainer::CKApplicationPermissions;
#[cfg(feature = "CloudKit_CKContainer")]
pub use self::__CKContainer::CKContainer;
pub use self::__CKContainer::CKCurrentUserDefaultName;
pub use self::__CKContainer::CKOwnerDefaultName;
pub use self::__CKContainer::{
    CKAccountStatusAvailable, CKAccountStatusCouldNotDetermine, CKAccountStatusNoAccount,
    CKAccountStatusRestricted, CKAccountStatusTemporarilyUnavailable,
};
pub use self::__CKContainer::{
    CKApplicationPermissionStatusCouldNotComplete, CKApplicationPermissionStatusDenied,
    CKApplicationPermissionStatusGranted, CKApplicationPermissionStatusInitialState,
};
#[cfg(feature = "CloudKit_CKDatabase")]
pub use self::__CKDatabase::CKDatabase;
pub use self::__CKDatabase::CKDatabaseScope;
pub use self::__CKDatabase::{
    CKDatabaseScopePrivate, CKDatabaseScopePublic, CKDatabaseScopeShared,
};
#[cfg(feature = "CloudKit_CKDatabaseOperation")]
pub use self::__CKDatabaseOperation::CKDatabaseOperation;
#[cfg(feature = "CloudKit_CKDiscoverAllUserIdentitiesOperation")]
pub use self::__CKDiscoverAllUserIdentitiesOperation::CKDiscoverAllUserIdentitiesOperation;
#[cfg(feature = "CloudKit_CKDiscoverUserIdentitiesOperation")]
pub use self::__CKDiscoverUserIdentitiesOperation::CKDiscoverUserIdentitiesOperation;
pub use self::__CKError::CKErrorCode;
pub use self::__CKError::CKErrorDomain;
pub use self::__CKError::CKErrorRetryAfterKey;
pub use self::__CKError::CKErrorUserDidResetEncryptedDataKey;
pub use self::__CKError::CKPartialErrorsByItemIDKey;
pub use self::__CKError::CKRecordChangedErrorAncestorRecordKey;
pub use self::__CKError::CKRecordChangedErrorClientRecordKey;
pub use self::__CKError::CKRecordChangedErrorServerRecordKey;
pub use self::__CKError::{
    CKErrorAccountTemporarilyUnavailable, CKErrorAlreadyShared, CKErrorAssetFileModified,
    CKErrorAssetFileNotFound, CKErrorAssetNotAvailable, CKErrorBadContainer, CKErrorBadDatabase,
    CKErrorBatchRequestFailed, CKErrorChangeTokenExpired, CKErrorConstraintViolation,
    CKErrorIncompatibleVersion, CKErrorInternalError, CKErrorInvalidArguments,
    CKErrorLimitExceeded, CKErrorManagedAccountRestricted, CKErrorMissingEntitlement,
    CKErrorNetworkFailure, CKErrorNetworkUnavailable, CKErrorNotAuthenticated,
    CKErrorOperationCancelled, CKErrorPartialFailure, CKErrorParticipantMayNeedVerification,
    CKErrorPermissionFailure, CKErrorQuotaExceeded, CKErrorReferenceViolation,
    CKErrorRequestRateLimited, CKErrorResultsTruncated, CKErrorServerRecordChanged,
    CKErrorServerRejectedRequest, CKErrorServerResponseLost, CKErrorServiceUnavailable,
    CKErrorTooManyParticipants, CKErrorUnknownItem, CKErrorUserDeletedZone, CKErrorZoneBusy,
    CKErrorZoneNotFound,
};
#[cfg(feature = "CloudKit_CKFetchDatabaseChangesOperation")]
pub use self::__CKFetchDatabaseChangesOperation::CKFetchDatabaseChangesOperation;
#[cfg(feature = "CloudKit_CKFetchNotificationChangesOperation")]
pub use self::__CKFetchNotificationChangesOperation::CKFetchNotificationChangesOperation;
#[cfg(feature = "CloudKit_CKFetchRecordChangesOperation")]
pub use self::__CKFetchRecordChangesOperation::CKFetchRecordChangesOperation;
#[cfg(feature = "CloudKit_CKFetchRecordZoneChangesConfiguration")]
pub use self::__CKFetchRecordZoneChangesOperation::CKFetchRecordZoneChangesConfiguration;
#[cfg(feature = "CloudKit_CKFetchRecordZoneChangesOperation")]
pub use self::__CKFetchRecordZoneChangesOperation::CKFetchRecordZoneChangesOperation;
#[cfg(feature = "CloudKit_CKFetchRecordZoneChangesOptions")]
pub use self::__CKFetchRecordZoneChangesOperation::CKFetchRecordZoneChangesOptions;
#[cfg(feature = "CloudKit_CKFetchRecordZonesOperation")]
pub use self::__CKFetchRecordZonesOperation::CKFetchRecordZonesOperation;
#[cfg(feature = "CloudKit_CKFetchRecordsOperation")]
pub use self::__CKFetchRecordsOperation::CKFetchRecordsOperation;
#[cfg(feature = "CloudKit_CKFetchShareMetadataOperation")]
pub use self::__CKFetchShareMetadataOperation::CKFetchShareMetadataOperation;
#[cfg(feature = "CloudKit_CKFetchShareParticipantsOperation")]
pub use self::__CKFetchShareParticipantsOperation::CKFetchShareParticipantsOperation;
#[cfg(feature = "CloudKit_CKFetchSubscriptionsOperation")]
pub use self::__CKFetchSubscriptionsOperation::CKFetchSubscriptionsOperation;
#[cfg(feature = "CloudKit_CKFetchWebAuthTokenOperation")]
pub use self::__CKFetchWebAuthTokenOperation::CKFetchWebAuthTokenOperation;
#[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
pub use self::__CKLocationSortDescriptor::CKLocationSortDescriptor;
#[cfg(feature = "CloudKit_CKMarkNotificationsReadOperation")]
pub use self::__CKMarkNotificationsReadOperation::CKMarkNotificationsReadOperation;
#[cfg(feature = "CloudKit_CKModifyBadgeOperation")]
pub use self::__CKModifyBadgeOperation::CKModifyBadgeOperation;
#[cfg(feature = "CloudKit_CKModifyRecordZonesOperation")]
pub use self::__CKModifyRecordZonesOperation::CKModifyRecordZonesOperation;
#[cfg(feature = "CloudKit_CKModifyRecordsOperation")]
pub use self::__CKModifyRecordsOperation::CKModifyRecordsOperation;
pub use self::__CKModifyRecordsOperation::CKRecordSavePolicy;
pub use self::__CKModifyRecordsOperation::{
    CKRecordSaveAllKeys, CKRecordSaveChangedKeys, CKRecordSaveIfServerRecordUnchanged,
};
#[cfg(feature = "CloudKit_CKModifySubscriptionsOperation")]
pub use self::__CKModifySubscriptionsOperation::CKModifySubscriptionsOperation;
#[cfg(feature = "CloudKit_CKDatabaseNotification")]
pub use self::__CKNotification::CKDatabaseNotification;
#[cfg(feature = "CloudKit_CKNotification")]
pub use self::__CKNotification::CKNotification;
#[cfg(feature = "CloudKit_CKNotificationID")]
pub use self::__CKNotification::CKNotificationID;
pub use self::__CKNotification::CKNotificationType;
#[cfg(feature = "CloudKit_CKQueryNotification")]
pub use self::__CKNotification::CKQueryNotification;
pub use self::__CKNotification::CKQueryNotificationReason;
#[cfg(feature = "CloudKit_CKRecordZoneNotification")]
pub use self::__CKNotification::CKRecordZoneNotification;
pub use self::__CKNotification::{
    CKNotificationTypeDatabase, CKNotificationTypeQuery, CKNotificationTypeReadNotification,
    CKNotificationTypeRecordZone,
};
pub use self::__CKNotification::{
    CKQueryNotificationReasonRecordCreated, CKQueryNotificationReasonRecordDeleted,
    CKQueryNotificationReasonRecordUpdated,
};
#[cfg(feature = "CloudKit_CKOperation")]
pub use self::__CKOperation::CKOperation;
#[cfg(feature = "CloudKit_CKOperationConfiguration")]
pub use self::__CKOperation::CKOperationConfiguration;
pub use self::__CKOperation::CKOperationID;
#[cfg(feature = "CloudKit_CKOperationGroup")]
pub use self::__CKOperationGroup::CKOperationGroup;
pub use self::__CKOperationGroup::CKOperationGroupTransferSize;
pub use self::__CKOperationGroup::{
    CKOperationGroupTransferSizeGigabytes, CKOperationGroupTransferSizeHundredsOfGigabytes,
    CKOperationGroupTransferSizeHundredsOfMegabytes, CKOperationGroupTransferSizeKilobytes,
    CKOperationGroupTransferSizeMegabytes, CKOperationGroupTransferSizeTensOfGigabytes,
    CKOperationGroupTransferSizeTensOfMegabytes, CKOperationGroupTransferSizeUnknown,
};
#[cfg(feature = "CloudKit_CKQuery")]
pub use self::__CKQuery::CKQuery;
#[cfg(feature = "CloudKit_CKQueryCursor")]
pub use self::__CKQueryOperation::CKQueryCursor;
#[cfg(feature = "CloudKit_CKQueryOperation")]
pub use self::__CKQueryOperation::CKQueryOperation;
pub use self::__CKQueryOperation::CKQueryOperationMaximumResults;
#[cfg(feature = "CloudKit_CKRecord")]
pub use self::__CKRecord::CKRecord;
pub use self::__CKRecord::CKRecordCreationDateKey;
pub use self::__CKRecord::CKRecordCreatorUserRecordIDKey;
pub use self::__CKRecord::CKRecordFieldKey;
pub use self::__CKRecord::CKRecordKeyValueSetting;
pub use self::__CKRecord::CKRecordLastModifiedUserRecordIDKey;
pub use self::__CKRecord::CKRecordModificationDateKey;
pub use self::__CKRecord::CKRecordParentKey;
pub use self::__CKRecord::CKRecordRecordIDKey;
pub use self::__CKRecord::CKRecordShareKey;
pub use self::__CKRecord::CKRecordType;
pub use self::__CKRecord::CKRecordTypeUserRecord;
pub use self::__CKRecord::CKRecordValue;
#[cfg(feature = "CloudKit_CKRecordID")]
pub use self::__CKRecordID::CKRecordID;
#[cfg(feature = "CloudKit_CKRecordZone")]
pub use self::__CKRecordZone::CKRecordZone;
pub use self::__CKRecordZone::CKRecordZoneCapabilities;
pub use self::__CKRecordZone::CKRecordZoneDefaultName;
pub use self::__CKRecordZone::{
    CKRecordZoneCapabilityAtomic, CKRecordZoneCapabilityFetchChanges,
    CKRecordZoneCapabilitySharing, CKRecordZoneCapabilityZoneWideSharing,
};
#[cfg(feature = "CloudKit_CKRecordZoneID")]
pub use self::__CKRecordZoneID::CKRecordZoneID;
#[cfg(feature = "CloudKit_CKReference")]
pub use self::__CKReference::CKReference;
pub use self::__CKReference::CKReferenceAction;
pub use self::__CKReference::{CKReferenceActionDeleteSelf, CKReferenceActionNone};
#[cfg(feature = "CloudKit_CKServerChangeToken")]
pub use self::__CKServerChangeToken::CKServerChangeToken;
pub use self::__CKShare::CKRecordNameZoneWideShare;
pub use self::__CKShare::CKRecordTypeShare;
#[cfg(feature = "CloudKit_CKShare")]
pub use self::__CKShare::CKShare;
pub use self::__CKShare::CKShareThumbnailImageDataKey;
pub use self::__CKShare::CKShareTitleKey;
pub use self::__CKShare::CKShareTypeKey;
#[cfg(feature = "CloudKit_CKShareMetadata")]
pub use self::__CKShareMetadata::CKShareMetadata;
#[cfg(feature = "CloudKit_CKShareParticipant")]
pub use self::__CKShareParticipant::CKShareParticipant;
pub use self::__CKShareParticipant::CKShareParticipantAcceptanceStatus;
pub use self::__CKShareParticipant::CKShareParticipantPermission;
pub use self::__CKShareParticipant::CKShareParticipantRole;
pub use self::__CKShareParticipant::CKShareParticipantType;
pub use self::__CKShareParticipant::{
    CKShareParticipantAcceptanceStatusAccepted, CKShareParticipantAcceptanceStatusPending,
    CKShareParticipantAcceptanceStatusRemoved, CKShareParticipantAcceptanceStatusUnknown,
};
pub use self::__CKShareParticipant::{
    CKShareParticipantPermissionNone, CKShareParticipantPermissionReadOnly,
    CKShareParticipantPermissionReadWrite, CKShareParticipantPermissionUnknown,
};
pub use self::__CKShareParticipant::{
    CKShareParticipantRoleOwner, CKShareParticipantRolePrivateUser,
    CKShareParticipantRolePublicUser, CKShareParticipantRoleUnknown,
};
pub use self::__CKShareParticipant::{
    CKShareParticipantTypeOwner, CKShareParticipantTypePrivateUser,
    CKShareParticipantTypePublicUser, CKShareParticipantTypeUnknown,
};
#[cfg(feature = "CloudKit_CKDatabaseSubscription")]
pub use self::__CKSubscription::CKDatabaseSubscription;
#[cfg(feature = "CloudKit_CKNotificationInfo")]
pub use self::__CKSubscription::CKNotificationInfo;
#[cfg(feature = "CloudKit_CKQuerySubscription")]
pub use self::__CKSubscription::CKQuerySubscription;
pub use self::__CKSubscription::CKQuerySubscriptionOptions;
#[cfg(feature = "CloudKit_CKRecordZoneSubscription")]
pub use self::__CKSubscription::CKRecordZoneSubscription;
#[cfg(feature = "CloudKit_CKSubscription")]
pub use self::__CKSubscription::CKSubscription;
pub use self::__CKSubscription::CKSubscriptionID;
pub use self::__CKSubscription::CKSubscriptionType;
pub use self::__CKSubscription::{
    CKQuerySubscriptionOptionsFiresOnRecordCreation,
    CKQuerySubscriptionOptionsFiresOnRecordDeletion, CKQuerySubscriptionOptionsFiresOnRecordUpdate,
    CKQuerySubscriptionOptionsFiresOnce,
};
pub use self::__CKSubscription::{
    CKSubscriptionTypeDatabase, CKSubscriptionTypeQuery, CKSubscriptionTypeRecordZone,
};
#[cfg(feature = "CloudKit_CKSyncEngine")]
pub use self::__CKSyncEngine::CKSyncEngine;
pub use self::__CKSyncEngine::CKSyncEngineDelegate;
#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesContext")]
pub use self::__CKSyncEngine::CKSyncEngineFetchChangesContext;
#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesOptions")]
pub use self::__CKSyncEngine::CKSyncEngineFetchChangesOptions;
#[cfg(feature = "CloudKit_CKSyncEngineFetchChangesScope")]
pub use self::__CKSyncEngine::CKSyncEngineFetchChangesScope;
#[cfg(feature = "CloudKit_CKSyncEngineSendChangesContext")]
pub use self::__CKSyncEngine::CKSyncEngineSendChangesContext;
#[cfg(feature = "CloudKit_CKSyncEngineSendChangesOptions")]
pub use self::__CKSyncEngine::CKSyncEngineSendChangesOptions;
#[cfg(feature = "CloudKit_CKSyncEngineSendChangesScope")]
pub use self::__CKSyncEngine::CKSyncEngineSendChangesScope;
pub use self::__CKSyncEngine::CKSyncEngineSyncReason;
pub use self::__CKSyncEngine::{CKSyncEngineSyncReasonManual, CKSyncEngineSyncReasonScheduled};
#[cfg(feature = "CloudKit_CKSyncEngineConfiguration")]
pub use self::__CKSyncEngineConfiguration::CKSyncEngineConfiguration;
#[cfg(feature = "CloudKit_CKSyncEngineAccountChangeEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineAccountChangeEvent;
pub use self::__CKSyncEngineEvent::CKSyncEngineAccountChangeType;
#[cfg(feature = "CloudKit_CKSyncEngineDidFetchChangesEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineDidFetchChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineDidFetchRecordZoneChangesEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineDidFetchRecordZoneChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineDidSendChangesEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineDidSendChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineEvent;
pub use self::__CKSyncEngineEvent::CKSyncEngineEventType;
#[cfg(feature = "CloudKit_CKSyncEngineFailedRecordSave")]
pub use self::__CKSyncEngineEvent::CKSyncEngineFailedRecordSave;
#[cfg(feature = "CloudKit_CKSyncEngineFailedZoneSave")]
pub use self::__CKSyncEngineEvent::CKSyncEngineFailedZoneSave;
#[cfg(feature = "CloudKit_CKSyncEngineFetchedDatabaseChangesEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineFetchedDatabaseChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineFetchedRecordDeletion")]
pub use self::__CKSyncEngineEvent::CKSyncEngineFetchedRecordDeletion;
#[cfg(feature = "CloudKit_CKSyncEngineFetchedRecordZoneChangesEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineFetchedRecordZoneChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineFetchedZoneDeletion")]
pub use self::__CKSyncEngineEvent::CKSyncEngineFetchedZoneDeletion;
#[cfg(feature = "CloudKit_CKSyncEngineSentDatabaseChangesEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineSentDatabaseChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineSentRecordZoneChangesEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineSentRecordZoneChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineStateUpdateEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineStateUpdateEvent;
#[cfg(feature = "CloudKit_CKSyncEngineWillFetchChangesEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineWillFetchChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineWillFetchRecordZoneChangesEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineWillFetchRecordZoneChangesEvent;
#[cfg(feature = "CloudKit_CKSyncEngineWillSendChangesEvent")]
pub use self::__CKSyncEngineEvent::CKSyncEngineWillSendChangesEvent;
pub use self::__CKSyncEngineEvent::CKSyncEngineZoneDeletionReason;
pub use self::__CKSyncEngineEvent::{
    CKSyncEngineAccountChangeTypeSignIn, CKSyncEngineAccountChangeTypeSignOut,
    CKSyncEngineAccountChangeTypeSwitchAccounts,
};
pub use self::__CKSyncEngineEvent::{
    CKSyncEngineEventTypeAccountChange, CKSyncEngineEventTypeDidFetchChanges,
    CKSyncEngineEventTypeDidFetchRecordZoneChanges, CKSyncEngineEventTypeDidSendChanges,
    CKSyncEngineEventTypeFetchedDatabaseChanges, CKSyncEngineEventTypeFetchedRecordZoneChanges,
    CKSyncEngineEventTypeSentDatabaseChanges, CKSyncEngineEventTypeSentRecordZoneChanges,
    CKSyncEngineEventTypeStateUpdate, CKSyncEngineEventTypeWillFetchChanges,
    CKSyncEngineEventTypeWillFetchRecordZoneChanges, CKSyncEngineEventTypeWillSendChanges,
};
pub use self::__CKSyncEngineEvent::{
    CKSyncEngineZoneDeletionReasonDeleted, CKSyncEngineZoneDeletionReasonEncryptedDataReset,
    CKSyncEngineZoneDeletionReasonPurged,
};
#[cfg(feature = "CloudKit_CKSyncEngineRecordZoneChangeBatch")]
pub use self::__CKSyncEngineRecordZoneChangeBatch::CKSyncEngineRecordZoneChangeBatch;
#[cfg(feature = "CloudKit_CKSyncEnginePendingDatabaseChange")]
pub use self::__CKSyncEngineState::CKSyncEnginePendingDatabaseChange;
pub use self::__CKSyncEngineState::CKSyncEnginePendingDatabaseChangeType;
#[cfg(feature = "CloudKit_CKSyncEnginePendingRecordZoneChange")]
pub use self::__CKSyncEngineState::CKSyncEnginePendingRecordZoneChange;
pub use self::__CKSyncEngineState::CKSyncEnginePendingRecordZoneChangeType;
#[cfg(feature = "CloudKit_CKSyncEnginePendingZoneDelete")]
pub use self::__CKSyncEngineState::CKSyncEnginePendingZoneDelete;
#[cfg(feature = "CloudKit_CKSyncEnginePendingZoneSave")]
pub use self::__CKSyncEngineState::CKSyncEnginePendingZoneSave;
#[cfg(feature = "CloudKit_CKSyncEngineState")]
pub use self::__CKSyncEngineState::CKSyncEngineState;
#[cfg(feature = "CloudKit_CKSyncEngineStateSerialization")]
pub use self::__CKSyncEngineState::CKSyncEngineStateSerialization;
pub use self::__CKSyncEngineState::{
    CKSyncEnginePendingDatabaseChangeTypeDeleteZone, CKSyncEnginePendingDatabaseChangeTypeSaveZone,
};
pub use self::__CKSyncEngineState::{
    CKSyncEnginePendingRecordZoneChangeTypeDeleteRecord,
    CKSyncEnginePendingRecordZoneChangeTypeSaveRecord,
};
#[cfg(feature = "CloudKit_CKSystemSharingUIObserver")]
pub use self::__CKSystemSharingUIObserver::CKSystemSharingUIObserver;
#[cfg(feature = "CloudKit_CKUserIdentity")]
pub use self::__CKUserIdentity::CKUserIdentity;
#[cfg(feature = "CloudKit_CKUserIdentityLookupInfo")]
pub use self::__CKUserIdentityLookupInfo::CKUserIdentityLookupInfo;
pub use self::__NSItemProvider_CKSharingSupport::CKSharePreparationCompletionHandler;
pub use self::__NSItemProvider_CKSharingSupport::CKSharePreparationHandler;
#[cfg(feature = "Foundation_NSItemProvider")]
pub use self::__NSItemProvider_CKSharingSupport::NSItemProviderCKSharingSupport;
