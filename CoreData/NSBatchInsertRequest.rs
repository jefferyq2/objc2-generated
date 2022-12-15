//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSBatchInsertRequest;

    unsafe impl ClassType for NSBatchInsertRequest {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreRequest;
    }
);

extern_methods!(
    unsafe impl NSBatchInsertRequest {
        #[method_id(@__retain_semantics Other entityName)]
        pub unsafe fn entityName(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Option<Id<NSEntityDescription, Shared>>;

        #[method_id(@__retain_semantics Other objectsToInsert)]
        pub unsafe fn objectsToInsert(
            &self,
        ) -> Option<Id<NSArray<NSDictionary<NSString, Object>>, Shared>>;

        #[method(setObjectsToInsert:)]
        pub unsafe fn setObjectsToInsert(
            &self,
            objectsToInsert: Option<&NSArray<NSDictionary<NSString, Object>>>,
        );

        #[method(dictionaryHandler)]
        pub unsafe fn dictionaryHandler(
            &self,
        ) -> *mut Block<(NonNull<NSMutableDictionary<NSString, Object>>,), Bool>;

        #[method(setDictionaryHandler:)]
        pub unsafe fn setDictionaryHandler(
            &self,
            dictionaryHandler: Option<
                &Block<(NonNull<NSMutableDictionary<NSString, Object>>,), Bool>,
            >,
        );

        #[method(managedObjectHandler)]
        pub unsafe fn managedObjectHandler(&self) -> *mut Block<(NonNull<NSManagedObject>,), Bool>;

        #[method(setManagedObjectHandler:)]
        pub unsafe fn setManagedObjectHandler(
            &self,
            managedObjectHandler: Option<&Block<(NonNull<NSManagedObject>,), Bool>>,
        );

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchInsertRequestResultType;

        #[method(setResultType:)]
        pub unsafe fn setResultType(&self, resultType: NSBatchInsertRequestResultType);

        #[method_id(@__retain_semantics Other batchInsertRequestWithEntityName:objects:)]
        pub unsafe fn batchInsertRequestWithEntityName_objects(
            entityName: &NSString,
            dictionaries: &NSArray<NSDictionary<NSString, Object>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other batchInsertRequestWithEntityName:dictionaryHandler:)]
        pub unsafe fn batchInsertRequestWithEntityName_dictionaryHandler(
            entityName: &NSString,
            handler: &Block<(NonNull<NSMutableDictionary<NSString, Object>>,), Bool>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other batchInsertRequestWithEntityName:managedObjectHandler:)]
        pub unsafe fn batchInsertRequestWithEntityName_managedObjectHandler(
            entityName: &NSString,
            handler: &Block<(NonNull<NSManagedObject>,), Bool>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithEntityName:objects:)]
        pub unsafe fn initWithEntityName_objects(
            this: Option<Allocated<Self>>,
            entityName: &NSString,
            dictionaries: &NSArray<NSDictionary<NSString, Object>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithEntity:objects:)]
        pub unsafe fn initWithEntity_objects(
            this: Option<Allocated<Self>>,
            entity: &NSEntityDescription,
            dictionaries: &NSArray<NSDictionary<NSString, Object>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithEntity:dictionaryHandler:)]
        pub unsafe fn initWithEntity_dictionaryHandler(
            this: Option<Allocated<Self>>,
            entity: &NSEntityDescription,
            handler: &Block<(NonNull<NSMutableDictionary<NSString, Object>>,), Bool>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithEntity:managedObjectHandler:)]
        pub unsafe fn initWithEntity_managedObjectHandler(
            this: Option<Allocated<Self>>,
            entity: &NSEntityDescription,
            handler: &Block<(NonNull<NSManagedObject>,), Bool>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithEntityName:dictionaryHandler:)]
        pub unsafe fn initWithEntityName_dictionaryHandler(
            this: Option<Allocated<Self>>,
            entityName: &NSString,
            handler: &Block<(NonNull<NSMutableDictionary<NSString, Object>>,), Bool>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithEntityName:managedObjectHandler:)]
        pub unsafe fn initWithEntityName_managedObjectHandler(
            this: Option<Allocated<Self>>,
            entityName: &NSString,
            handler: &Block<(NonNull<NSManagedObject>,), Bool>,
        ) -> Id<Self, Shared>;
    }
);
