//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSDetailedErrorsKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSValidationObjectErrorKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSValidationKeyErrorKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSValidationPredicateErrorKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSValidationValueErrorKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSAffectedStoresErrorKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSAffectedObjectsErrorKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSPersistentStoreSaveConflictsErrorKey: &'static NSString);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSSQLiteErrorDomain: &'static NSString);

pub const NSManagedObjectValidationError: NSInteger = 1550;
pub const NSManagedObjectConstraintValidationError: NSInteger = 1551;
pub const NSValidationMultipleErrorsError: NSInteger = 1560;
pub const NSValidationMissingMandatoryPropertyError: NSInteger = 1570;
pub const NSValidationRelationshipLacksMinimumCountError: NSInteger = 1580;
pub const NSValidationRelationshipExceedsMaximumCountError: NSInteger = 1590;
pub const NSValidationRelationshipDeniedDeleteError: NSInteger = 1600;
pub const NSValidationNumberTooLargeError: NSInteger = 1610;
pub const NSValidationNumberTooSmallError: NSInteger = 1620;
pub const NSValidationDateTooLateError: NSInteger = 1630;
pub const NSValidationDateTooSoonError: NSInteger = 1640;
pub const NSValidationInvalidDateError: NSInteger = 1650;
pub const NSValidationStringTooLongError: NSInteger = 1660;
pub const NSValidationStringTooShortError: NSInteger = 1670;
pub const NSValidationStringPatternMatchingError: NSInteger = 1680;
pub const NSValidationInvalidURIError: NSInteger = 1690;
pub const NSManagedObjectContextLockingError: NSInteger = 132000;
pub const NSPersistentStoreCoordinatorLockingError: NSInteger = 132010;
pub const NSManagedObjectReferentialIntegrityError: NSInteger = 133000;
pub const NSManagedObjectExternalRelationshipError: NSInteger = 133010;
pub const NSManagedObjectMergeError: NSInteger = 133020;
pub const NSManagedObjectConstraintMergeError: NSInteger = 133021;
pub const NSPersistentStoreInvalidTypeError: NSInteger = 134000;
pub const NSPersistentStoreTypeMismatchError: NSInteger = 134010;
pub const NSPersistentStoreIncompatibleSchemaError: NSInteger = 134020;
pub const NSPersistentStoreSaveError: NSInteger = 134030;
pub const NSPersistentStoreIncompleteSaveError: NSInteger = 134040;
pub const NSPersistentStoreSaveConflictsError: NSInteger = 134050;
pub const NSCoreDataError: NSInteger = 134060;
pub const NSPersistentStoreOperationError: NSInteger = 134070;
pub const NSPersistentStoreOpenError: NSInteger = 134080;
pub const NSPersistentStoreTimeoutError: NSInteger = 134090;
pub const NSPersistentStoreUnsupportedRequestTypeError: NSInteger = 134091;
pub const NSPersistentStoreIncompatibleVersionHashError: NSInteger = 134100;
pub const NSMigrationError: NSInteger = 134110;
pub const NSMigrationConstraintViolationError: NSInteger = 134111;
pub const NSMigrationCancelledError: NSInteger = 134120;
pub const NSMigrationMissingSourceModelError: NSInteger = 134130;
pub const NSMigrationMissingMappingModelError: NSInteger = 134140;
pub const NSMigrationManagerSourceStoreError: NSInteger = 134150;
pub const NSMigrationManagerDestinationStoreError: NSInteger = 134160;
pub const NSEntityMigrationPolicyError: NSInteger = 134170;
pub const NSSQLiteError: NSInteger = 134180;
pub const NSInferredMappingModelError: NSInteger = 134190;
pub const NSExternalRecordImportError: NSInteger = 134200;
pub const NSPersistentHistoryTokenExpiredError: NSInteger = 134301;
pub const NSManagedObjectModelReferenceNotFoundError: NSInteger = 134504;
pub const NSStagedMigrationFrameworkVersionMismatchError: NSInteger = 134505;
pub const NSStagedMigrationBackwardMigrationError: NSInteger = 134506;
