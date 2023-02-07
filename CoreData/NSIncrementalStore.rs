//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSIncrementalStore")]
    pub struct NSIncrementalStore;

    #[cfg(feature = "CoreData_NSIncrementalStore")]
    unsafe impl ClassType for NSIncrementalStore {
        #[inherits(NSObject)]
        type Super = NSPersistentStore;
    }
);

#[cfg(feature = "CoreData_NSIncrementalStore")]
unsafe impl NSObjectProtocol for NSIncrementalStore {}

extern_methods!(
    #[cfg(feature = "CoreData_NSIncrementalStore")]
    unsafe impl NSIncrementalStore {
        #[cfg(feature = "Foundation_NSError")]
        #[method(loadMetadata:_)]
        pub unsafe fn loadMetadata(&self) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "CoreData_NSManagedObjectContext",
            feature = "CoreData_NSPersistentStoreRequest",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other executeRequest:withContext:error:_)]
        pub unsafe fn executeRequest_withContext_error(
            &self,
            request: &NSPersistentStoreRequest,
            context: Option<&NSManagedObjectContext>,
        ) -> Result<Id<Object>, Id<NSError>>;

        #[cfg(all(
            feature = "CoreData_NSIncrementalStoreNode",
            feature = "CoreData_NSManagedObjectContext",
            feature = "CoreData_NSManagedObjectID",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics New newValuesForObjectWithID:withContext:error:_)]
        pub unsafe fn newValuesForObjectWithID_withContext_error(
            &self,
            object_id: &NSManagedObjectID,
            context: &NSManagedObjectContext,
        ) -> Result<Id<NSIncrementalStoreNode>, Id<NSError>>;

        #[cfg(all(
            feature = "CoreData_NSManagedObjectContext",
            feature = "CoreData_NSManagedObjectID",
            feature = "CoreData_NSRelationshipDescription",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics New newValueForRelationship:forObjectWithID:withContext:error:_)]
        pub unsafe fn newValueForRelationship_forObjectWithID_withContext_error(
            &self,
            relationship: &NSRelationshipDescription,
            object_id: &NSManagedObjectID,
            context: Option<&NSManagedObjectContext>,
        ) -> Result<Id<Object>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other identifierForNewStoreAtURL:)]
        pub unsafe fn identifierForNewStoreAtURL(store_url: &NSURL) -> Id<Object>;

        #[cfg(all(
            feature = "CoreData_NSManagedObject",
            feature = "CoreData_NSManagedObjectID",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other obtainPermanentIDsForObjects:error:_)]
        pub unsafe fn obtainPermanentIDsForObjects_error(
            &self,
            array: &NSArray<NSManagedObject>,
        ) -> Result<Id<NSArray<NSManagedObjectID>>, Id<NSError>>;

        #[cfg(all(feature = "CoreData_NSManagedObjectID", feature = "Foundation_NSArray"))]
        #[method(managedObjectContextDidRegisterObjectsWithIDs:)]
        pub unsafe fn managedObjectContextDidRegisterObjectsWithIDs(
            &self,
            object_i_ds: &NSArray<NSManagedObjectID>,
        );

        #[cfg(all(feature = "CoreData_NSManagedObjectID", feature = "Foundation_NSArray"))]
        #[method(managedObjectContextDidUnregisterObjectsWithIDs:)]
        pub unsafe fn managedObjectContextDidUnregisterObjectsWithIDs(
            &self,
            object_i_ds: &NSArray<NSManagedObjectID>,
        );

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "CoreData_NSManagedObjectID"
        ))]
        #[method_id(@__retain_semantics New newObjectIDForEntity:referenceObject:)]
        pub unsafe fn newObjectIDForEntity_referenceObject(
            &self,
            entity: &NSEntityDescription,
            data: &Object,
        ) -> Id<NSManagedObjectID>;

        #[cfg(feature = "CoreData_NSManagedObjectID")]
        #[method_id(@__retain_semantics Other referenceObjectForObjectID:)]
        pub unsafe fn referenceObjectForObjectID(
            &self,
            object_id: &NSManagedObjectID,
        ) -> Id<Object>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSPersistentStore`
    #[cfg(feature = "CoreData_NSIncrementalStore")]
    unsafe impl NSIncrementalStore {
        #[cfg(all(
            feature = "CoreData_NSPersistentStoreCoordinator",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithPersistentStoreCoordinator:configurationName:URL:options:)]
        pub unsafe fn initWithPersistentStoreCoordinator_configurationName_URL_options(
            this: Option<Allocated<Self>>,
            root: Option<&NSPersistentStoreCoordinator>,
            name: Option<&NSString>,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Id<Self>;
    }
);
