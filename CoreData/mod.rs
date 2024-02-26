// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `CoreData` framework
#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[cfg_attr(feature = "apple", link(name = "CoreData", kind = "framework"))]
extern "C" {}

#[path = "CoreDataDefines.rs"]
mod __CoreDataDefines;
#[path = "CoreDataErrors.rs"]
mod __CoreDataErrors;
#[path = "NSAtomicStore.rs"]
mod __NSAtomicStore;
#[path = "NSAtomicStoreCacheNode.rs"]
mod __NSAtomicStoreCacheNode;
#[path = "NSAttributeDescription.rs"]
mod __NSAttributeDescription;
#[path = "NSBatchDeleteRequest.rs"]
mod __NSBatchDeleteRequest;
#[path = "NSBatchInsertRequest.rs"]
mod __NSBatchInsertRequest;
#[path = "NSBatchUpdateRequest.rs"]
mod __NSBatchUpdateRequest;
#[path = "NSCompositeAttributeDescription.rs"]
mod __NSCompositeAttributeDescription;
#[path = "NSCoreDataCoreSpotlightDelegate.rs"]
mod __NSCoreDataCoreSpotlightDelegate;
#[path = "NSCustomMigrationStage.rs"]
mod __NSCustomMigrationStage;
#[path = "NSDerivedAttributeDescription.rs"]
mod __NSDerivedAttributeDescription;
#[path = "NSEntityDescription.rs"]
mod __NSEntityDescription;
#[path = "NSEntityMapping.rs"]
mod __NSEntityMapping;
#[path = "NSEntityMigrationPolicy.rs"]
mod __NSEntityMigrationPolicy;
#[path = "NSExpressionDescription.rs"]
mod __NSExpressionDescription;
#[path = "NSFetchIndexDescription.rs"]
mod __NSFetchIndexDescription;
#[path = "NSFetchIndexElementDescription.rs"]
mod __NSFetchIndexElementDescription;
#[path = "NSFetchRequest.rs"]
mod __NSFetchRequest;
#[path = "NSFetchRequestExpression.rs"]
mod __NSFetchRequestExpression;
#[path = "NSFetchedPropertyDescription.rs"]
mod __NSFetchedPropertyDescription;
#[path = "NSFetchedResultsController.rs"]
mod __NSFetchedResultsController;
#[path = "NSIncrementalStore.rs"]
mod __NSIncrementalStore;
#[path = "NSIncrementalStoreNode.rs"]
mod __NSIncrementalStoreNode;
#[path = "NSLightweightMigrationStage.rs"]
mod __NSLightweightMigrationStage;
#[path = "NSManagedObject.rs"]
mod __NSManagedObject;
#[path = "NSManagedObjectContext.rs"]
mod __NSManagedObjectContext;
#[path = "NSManagedObjectID.rs"]
mod __NSManagedObjectID;
#[path = "NSManagedObjectModel.rs"]
mod __NSManagedObjectModel;
#[path = "NSManagedObjectModelReference.rs"]
mod __NSManagedObjectModelReference;
#[path = "NSMappingModel.rs"]
mod __NSMappingModel;
#[path = "NSMergePolicy.rs"]
mod __NSMergePolicy;
#[path = "NSMigrationManager.rs"]
mod __NSMigrationManager;
#[path = "NSMigrationStage.rs"]
mod __NSMigrationStage;
#[path = "NSPersistentCloudKitContainer.rs"]
mod __NSPersistentCloudKitContainer;
#[path = "NSPersistentCloudKitContainerEvent.rs"]
mod __NSPersistentCloudKitContainerEvent;
#[path = "NSPersistentCloudKitContainerEventRequest.rs"]
mod __NSPersistentCloudKitContainerEventRequest;
#[path = "NSPersistentCloudKitContainerOptions.rs"]
mod __NSPersistentCloudKitContainerOptions;
#[path = "NSPersistentContainer.rs"]
mod __NSPersistentContainer;
#[path = "NSPersistentHistoryChange.rs"]
mod __NSPersistentHistoryChange;
#[path = "NSPersistentHistoryChangeRequest.rs"]
mod __NSPersistentHistoryChangeRequest;
#[path = "NSPersistentHistoryToken.rs"]
mod __NSPersistentHistoryToken;
#[path = "NSPersistentHistoryTransaction.rs"]
mod __NSPersistentHistoryTransaction;
#[path = "NSPersistentStore.rs"]
mod __NSPersistentStore;
#[path = "NSPersistentStoreCoordinator.rs"]
mod __NSPersistentStoreCoordinator;
#[path = "NSPersistentStoreDescription.rs"]
mod __NSPersistentStoreDescription;
#[path = "NSPersistentStoreRequest.rs"]
mod __NSPersistentStoreRequest;
#[path = "NSPersistentStoreResult.rs"]
mod __NSPersistentStoreResult;
#[path = "NSPropertyDescription.rs"]
mod __NSPropertyDescription;
#[path = "NSPropertyMapping.rs"]
mod __NSPropertyMapping;
#[path = "NSQueryGenerationToken.rs"]
mod __NSQueryGenerationToken;
#[path = "NSRelationshipDescription.rs"]
mod __NSRelationshipDescription;
#[path = "NSSaveChangesRequest.rs"]
mod __NSSaveChangesRequest;
#[path = "NSStagedMigrationManager.rs"]
mod __NSStagedMigrationManager;

pub use self::__CoreDataDefines::NSCoreDataVersionNumber;
#[cfg(feature = "Foundation_NSString")]
pub use self::__CoreDataErrors::NSAffectedObjectsErrorKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__CoreDataErrors::NSAffectedStoresErrorKey;
pub use self::__CoreDataErrors::NSCoreDataError;
#[cfg(feature = "Foundation_NSString")]
pub use self::__CoreDataErrors::NSDetailedErrorsKey;
pub use self::__CoreDataErrors::NSEntityMigrationPolicyError;
pub use self::__CoreDataErrors::NSExternalRecordImportError;
pub use self::__CoreDataErrors::NSInferredMappingModelError;
pub use self::__CoreDataErrors::NSManagedObjectConstraintMergeError;
pub use self::__CoreDataErrors::NSManagedObjectConstraintValidationError;
pub use self::__CoreDataErrors::NSManagedObjectContextLockingError;
pub use self::__CoreDataErrors::NSManagedObjectExternalRelationshipError;
pub use self::__CoreDataErrors::NSManagedObjectMergeError;
pub use self::__CoreDataErrors::NSManagedObjectModelReferenceNotFoundError;
pub use self::__CoreDataErrors::NSManagedObjectReferentialIntegrityError;
pub use self::__CoreDataErrors::NSManagedObjectValidationError;
pub use self::__CoreDataErrors::NSMigrationCancelledError;
pub use self::__CoreDataErrors::NSMigrationConstraintViolationError;
pub use self::__CoreDataErrors::NSMigrationError;
pub use self::__CoreDataErrors::NSMigrationManagerDestinationStoreError;
pub use self::__CoreDataErrors::NSMigrationManagerSourceStoreError;
pub use self::__CoreDataErrors::NSMigrationMissingMappingModelError;
pub use self::__CoreDataErrors::NSMigrationMissingSourceModelError;
pub use self::__CoreDataErrors::NSPersistentHistoryTokenExpiredError;
pub use self::__CoreDataErrors::NSPersistentStoreCoordinatorLockingError;
pub use self::__CoreDataErrors::NSPersistentStoreIncompatibleSchemaError;
pub use self::__CoreDataErrors::NSPersistentStoreIncompatibleVersionHashError;
pub use self::__CoreDataErrors::NSPersistentStoreIncompleteSaveError;
pub use self::__CoreDataErrors::NSPersistentStoreInvalidTypeError;
pub use self::__CoreDataErrors::NSPersistentStoreOpenError;
pub use self::__CoreDataErrors::NSPersistentStoreOperationError;
pub use self::__CoreDataErrors::NSPersistentStoreSaveConflictsError;
#[cfg(feature = "Foundation_NSString")]
pub use self::__CoreDataErrors::NSPersistentStoreSaveConflictsErrorKey;
pub use self::__CoreDataErrors::NSPersistentStoreSaveError;
pub use self::__CoreDataErrors::NSPersistentStoreTimeoutError;
pub use self::__CoreDataErrors::NSPersistentStoreTypeMismatchError;
pub use self::__CoreDataErrors::NSPersistentStoreUnsupportedRequestTypeError;
pub use self::__CoreDataErrors::NSSQLiteError;
#[cfg(feature = "Foundation_NSString")]
pub use self::__CoreDataErrors::NSSQLiteErrorDomain;
pub use self::__CoreDataErrors::NSStagedMigrationBackwardMigrationError;
pub use self::__CoreDataErrors::NSStagedMigrationFrameworkVersionMismatchError;
pub use self::__CoreDataErrors::NSValidationDateTooLateError;
pub use self::__CoreDataErrors::NSValidationDateTooSoonError;
pub use self::__CoreDataErrors::NSValidationInvalidDateError;
pub use self::__CoreDataErrors::NSValidationInvalidURIError;
#[cfg(feature = "Foundation_NSString")]
pub use self::__CoreDataErrors::NSValidationKeyErrorKey;
pub use self::__CoreDataErrors::NSValidationMissingMandatoryPropertyError;
pub use self::__CoreDataErrors::NSValidationMultipleErrorsError;
pub use self::__CoreDataErrors::NSValidationNumberTooLargeError;
pub use self::__CoreDataErrors::NSValidationNumberTooSmallError;
#[cfg(feature = "Foundation_NSString")]
pub use self::__CoreDataErrors::NSValidationObjectErrorKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__CoreDataErrors::NSValidationPredicateErrorKey;
pub use self::__CoreDataErrors::NSValidationRelationshipDeniedDeleteError;
pub use self::__CoreDataErrors::NSValidationRelationshipExceedsMaximumCountError;
pub use self::__CoreDataErrors::NSValidationRelationshipLacksMinimumCountError;
pub use self::__CoreDataErrors::NSValidationStringPatternMatchingError;
pub use self::__CoreDataErrors::NSValidationStringTooLongError;
pub use self::__CoreDataErrors::NSValidationStringTooShortError;
#[cfg(feature = "Foundation_NSString")]
pub use self::__CoreDataErrors::NSValidationValueErrorKey;
#[cfg(feature = "CoreData_NSAtomicStore")]
pub use self::__NSAtomicStore::NSAtomicStore;
#[cfg(feature = "CoreData_NSAtomicStoreCacheNode")]
pub use self::__NSAtomicStoreCacheNode::NSAtomicStoreCacheNode;
#[cfg(feature = "CoreData_NSAttributeDescription")]
pub use self::__NSAttributeDescription::NSAttributeDescription;
pub use self::__NSAttributeDescription::NSAttributeType;
#[cfg(feature = "CoreData_NSBatchDeleteRequest")]
pub use self::__NSBatchDeleteRequest::NSBatchDeleteRequest;
#[cfg(feature = "CoreData_NSBatchInsertRequest")]
pub use self::__NSBatchInsertRequest::NSBatchInsertRequest;
#[cfg(feature = "CoreData_NSBatchUpdateRequest")]
pub use self::__NSBatchUpdateRequest::NSBatchUpdateRequest;
#[cfg(feature = "CoreData_NSCompositeAttributeDescription")]
pub use self::__NSCompositeAttributeDescription::NSCompositeAttributeDescription;
#[cfg(feature = "CoreData_NSCoreDataCoreSpotlightDelegate")]
pub use self::__NSCoreDataCoreSpotlightDelegate::NSCoreDataCoreSpotlightDelegate;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSCoreDataCoreSpotlightDelegate::NSCoreDataCoreSpotlightDelegateIndexDidUpdateNotification;
#[cfg(feature = "CoreData_NSCustomMigrationStage")]
pub use self::__NSCustomMigrationStage::NSCustomMigrationStage;
#[cfg(feature = "CoreData_NSDerivedAttributeDescription")]
pub use self::__NSDerivedAttributeDescription::NSDerivedAttributeDescription;
#[cfg(feature = "CoreData_NSEntityDescription")]
pub use self::__NSEntityDescription::NSEntityDescription;
#[cfg(feature = "CoreData_NSEntityMapping")]
pub use self::__NSEntityMapping::NSEntityMapping;
pub use self::__NSEntityMapping::NSEntityMappingType;
#[cfg(feature = "CoreData_NSEntityMigrationPolicy")]
pub use self::__NSEntityMigrationPolicy::NSEntityMigrationPolicy;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSEntityMigrationPolicy::NSMigrationDestinationObjectKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSEntityMigrationPolicy::NSMigrationEntityMappingKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSEntityMigrationPolicy::NSMigrationEntityPolicyKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSEntityMigrationPolicy::NSMigrationManagerKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSEntityMigrationPolicy::NSMigrationPropertyMappingKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSEntityMigrationPolicy::NSMigrationSourceObjectKey;
#[cfg(feature = "CoreData_NSExpressionDescription")]
pub use self::__NSExpressionDescription::NSExpressionDescription;
#[cfg(feature = "CoreData_NSFetchIndexDescription")]
pub use self::__NSFetchIndexDescription::NSFetchIndexDescription;
#[cfg(feature = "CoreData_NSFetchIndexElementDescription")]
pub use self::__NSFetchIndexElementDescription::NSFetchIndexElementDescription;
pub use self::__NSFetchIndexElementDescription::NSFetchIndexElementType;
#[cfg(feature = "CoreData_NSAsynchronousFetchRequest")]
pub use self::__NSFetchRequest::NSAsynchronousFetchRequest;
#[cfg(feature = "CoreData_NSFetchRequest")]
pub use self::__NSFetchRequest::NSFetchRequest;
pub use self::__NSFetchRequest::NSFetchRequestResult;
pub use self::__NSFetchRequest::NSFetchRequestResultType;
#[cfg(feature = "CoreData_NSAsynchronousFetchResult")]
pub use self::__NSFetchRequest::NSPersistentStoreAsynchronousFetchResultCompletionBlock;
#[cfg(feature = "CoreData_NSFetchRequestExpression")]
pub use self::__NSFetchRequestExpression::NSFetchRequestExpression;
pub use self::__NSFetchRequestExpression::NSFetchRequestExpressionType;
#[cfg(feature = "CoreData_NSFetchedPropertyDescription")]
pub use self::__NSFetchedPropertyDescription::NSFetchedPropertyDescription;
pub use self::__NSFetchedResultsController::NSFetchedResultsChangeType;
#[cfg(feature = "CoreData_NSFetchedResultsController")]
pub use self::__NSFetchedResultsController::NSFetchedResultsController;
pub use self::__NSFetchedResultsController::NSFetchedResultsControllerDelegate;
pub use self::__NSFetchedResultsController::NSFetchedResultsSectionInfo;
#[cfg(feature = "CoreData_NSIncrementalStore")]
pub use self::__NSIncrementalStore::NSIncrementalStore;
#[cfg(feature = "CoreData_NSIncrementalStoreNode")]
pub use self::__NSIncrementalStoreNode::NSIncrementalStoreNode;
#[cfg(feature = "CoreData_NSLightweightMigrationStage")]
pub use self::__NSLightweightMigrationStage::NSLightweightMigrationStage;
#[cfg(feature = "CoreData_NSManagedObject")]
pub use self::__NSManagedObject::NSManagedObject;
pub use self::__NSManagedObject::NSSnapshotEventType;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSDeletedObjectIDsKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSDeletedObjectsKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSInsertedObjectIDsKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSInsertedObjectsKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSInvalidatedAllObjectsKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSInvalidatedObjectIDsKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSInvalidatedObjectsKey;
#[cfg(feature = "CoreData_NSManagedObjectContext")]
pub use self::__NSManagedObjectContext::NSManagedObjectContext;
pub use self::__NSManagedObjectContext::NSManagedObjectContextConcurrencyType;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSManagedObjectContextDidMergeChangesObjectIDsNotification;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSManagedObjectContextDidSaveNotification;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSManagedObjectContextDidSaveObjectIDsNotification;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSManagedObjectContextObjectsDidChangeNotification;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSManagedObjectContextQueryGenerationKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSManagedObjectContextWillSaveNotification;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSRefreshedObjectIDsKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSRefreshedObjectsKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSUpdatedObjectIDsKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSManagedObjectContext::NSUpdatedObjectsKey;
#[cfg(feature = "CoreData_NSManagedObjectID")]
pub use self::__NSManagedObjectID::NSManagedObjectID;
#[cfg(feature = "CoreData_NSManagedObjectModel")]
pub use self::__NSManagedObjectModel::NSManagedObjectModel;
#[cfg(feature = "CoreData_NSManagedObjectModelReference")]
pub use self::__NSManagedObjectModelReference::NSManagedObjectModelReference;
#[cfg(feature = "CoreData_NSMappingModel")]
pub use self::__NSMappingModel::NSMappingModel;
#[cfg(feature = "CoreData_NSConstraintConflict")]
pub use self::__NSMergePolicy::NSConstraintConflict;
#[cfg(feature = "CoreData_NSMergeConflict")]
pub use self::__NSMergePolicy::NSMergeConflict;
#[cfg(feature = "CoreData_NSMergePolicy")]
pub use self::__NSMergePolicy::NSMergePolicy;
pub use self::__NSMergePolicy::NSMergePolicyType;
#[cfg(feature = "CoreData_NSMigrationManager")]
pub use self::__NSMigrationManager::NSMigrationManager;
#[cfg(feature = "CoreData_NSMigrationStage")]
pub use self::__NSMigrationStage::NSMigrationStage;
#[cfg(feature = "CoreData_NSPersistentCloudKitContainer")]
pub use self::__NSPersistentCloudKitContainer::NSPersistentCloudKitContainer;
pub use self::__NSPersistentCloudKitContainer::NSPersistentCloudKitContainerSchemaInitializationOptions;
#[cfg(feature = "CoreData_NSPersistentCloudKitContainerEvent")]
pub use self::__NSPersistentCloudKitContainerEvent::NSPersistentCloudKitContainerEvent;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentCloudKitContainerEvent::NSPersistentCloudKitContainerEventChangedNotification;
pub use self::__NSPersistentCloudKitContainerEvent::NSPersistentCloudKitContainerEventType;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentCloudKitContainerEvent::NSPersistentCloudKitContainerEventUserInfoKey;
#[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventRequest")]
pub use self::__NSPersistentCloudKitContainerEventRequest::NSPersistentCloudKitContainerEventRequest;
#[cfg(feature = "CoreData_NSPersistentCloudKitContainerOptions")]
pub use self::__NSPersistentCloudKitContainerOptions::NSPersistentCloudKitContainerOptions;
#[cfg(feature = "CoreData_NSPersistentContainer")]
pub use self::__NSPersistentContainer::NSPersistentContainer;
#[cfg(feature = "CoreData_NSPersistentHistoryChange")]
pub use self::__NSPersistentHistoryChange::NSPersistentHistoryChange;
pub use self::__NSPersistentHistoryChange::NSPersistentHistoryChangeType;
#[cfg(feature = "CoreData_NSPersistentHistoryChangeRequest")]
pub use self::__NSPersistentHistoryChangeRequest::NSPersistentHistoryChangeRequest;
#[cfg(feature = "CoreData_NSPersistentHistoryToken")]
pub use self::__NSPersistentHistoryToken::NSPersistentHistoryToken;
#[cfg(feature = "CoreData_NSPersistentHistoryTransaction")]
pub use self::__NSPersistentHistoryTransaction::NSPersistentHistoryTransaction;
#[cfg(feature = "CoreData_NSPersistentStore")]
pub use self::__NSPersistentStore::NSPersistentStore;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSAddedPersistentStoresKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSBinaryExternalRecordType;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSBinaryStoreInsecureDecodingCompatibilityOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSBinaryStoreSecureDecodingClasses;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSBinaryStoreType;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSCoreDataCoreSpotlightExporter;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSEntityNameInPathKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSExternalRecordExtensionOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSExternalRecordsDirectoryOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSExternalRecordsFileFormatOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSIgnorePersistentStoreVersioningOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSInMemoryStoreType;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSInferMappingModelAutomaticallyOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSMigratePersistentStoresAutomaticallyOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSModelPathKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSObjectURIKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentHistoryTokenKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentHistoryTrackingKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreConnectionPoolMaxSizeKey;
#[cfg(feature = "CoreData_NSPersistentStoreCoordinator")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreCoordinator;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreCoordinatorStoresDidChangeNotification;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreCoordinatorStoresWillChangeNotification;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreCoordinatorWillRemoveStoreNotification;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreDeferredLightweightMigrationOptionKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreDidImportUbiquitousContentChangesNotification;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreFileProtectionKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreForceDestroyOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreOSCompatibility;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreRebuildFromUbiquitousContentOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreRemoteChangeNotification;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreRemoteChangeNotificationPostOptionKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreRemoveUbiquitousMetadataOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreStagedMigrationManagerOptionKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreTimeoutOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreURLKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousContainerIdentifierKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousContentNameKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousContentURLKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousPeerTokenOption;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousTransitionType;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousTransitionTypeKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSReadOnlyPersistentStoreOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSRemovedPersistentStoresKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSSQLiteAnalyzeOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSSQLiteManualVacuumOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSSQLitePragmasOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSSQLiteStoreType;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSStoreModelVersionHashesKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSStoreModelVersionIdentifiersKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSStorePathKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSStoreTypeKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSStoreUUIDInPathKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSStoreUUIDKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSUUIDChangedPersistentStoresKey;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSValidateXMLStoreOption;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSXMLExternalRecordType;
#[cfg(feature = "Foundation_NSString")]
pub use self::__NSPersistentStoreCoordinator::NSXMLStoreType;
#[cfg(feature = "CoreData_NSPersistentStoreDescription")]
pub use self::__NSPersistentStoreDescription::NSPersistentStoreDescription;
#[cfg(feature = "CoreData_NSPersistentStoreRequest")]
pub use self::__NSPersistentStoreRequest::NSPersistentStoreRequest;
pub use self::__NSPersistentStoreRequest::NSPersistentStoreRequestType;
#[cfg(feature = "CoreData_NSAsynchronousFetchResult")]
pub use self::__NSPersistentStoreResult::NSAsynchronousFetchResult;
pub use self::__NSPersistentStoreResult::NSBatchDeleteRequestResultType;
#[cfg(feature = "CoreData_NSBatchDeleteResult")]
pub use self::__NSPersistentStoreResult::NSBatchDeleteResult;
pub use self::__NSPersistentStoreResult::NSBatchInsertRequestResultType;
#[cfg(feature = "CoreData_NSBatchInsertResult")]
pub use self::__NSPersistentStoreResult::NSBatchInsertResult;
pub use self::__NSPersistentStoreResult::NSBatchUpdateRequestResultType;
#[cfg(feature = "CoreData_NSBatchUpdateResult")]
pub use self::__NSPersistentStoreResult::NSBatchUpdateResult;
#[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventResult")]
pub use self::__NSPersistentStoreResult::NSPersistentCloudKitContainerEventResult;
pub use self::__NSPersistentStoreResult::NSPersistentCloudKitContainerEventResultType;
#[cfg(feature = "CoreData_NSPersistentHistoryResult")]
pub use self::__NSPersistentStoreResult::NSPersistentHistoryResult;
pub use self::__NSPersistentStoreResult::NSPersistentHistoryResultType;
#[cfg(feature = "CoreData_NSPersistentStoreAsynchronousResult")]
pub use self::__NSPersistentStoreResult::NSPersistentStoreAsynchronousResult;
#[cfg(feature = "CoreData_NSPersistentStoreResult")]
pub use self::__NSPersistentStoreResult::NSPersistentStoreResult;
#[cfg(feature = "CoreData_NSPropertyDescription")]
pub use self::__NSPropertyDescription::NSPropertyDescription;
#[cfg(feature = "CoreData_NSPropertyMapping")]
pub use self::__NSPropertyMapping::NSPropertyMapping;
#[cfg(feature = "CoreData_NSQueryGenerationToken")]
pub use self::__NSQueryGenerationToken::NSQueryGenerationToken;
pub use self::__NSRelationshipDescription::NSDeleteRule;
#[cfg(feature = "CoreData_NSRelationshipDescription")]
pub use self::__NSRelationshipDescription::NSRelationshipDescription;
#[cfg(feature = "CoreData_NSSaveChangesRequest")]
pub use self::__NSSaveChangesRequest::NSSaveChangesRequest;
#[cfg(feature = "CoreData_NSStagedMigrationManager")]
pub use self::__NSStagedMigrationManager::NSStagedMigrationManager;
