//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAtomicStore;

    unsafe impl ClassType for NSAtomicStore {
        #[inherits(NSObject)]
        type Super = NSPersistentStore;
    }
);

extern_methods!(
    unsafe impl NSAtomicStore {
        #[method_id(@__retain_semantics Init initWithPersistentStoreCoordinator:configurationName:URL:options:)]
        pub unsafe fn initWithPersistentStoreCoordinator_configurationName_URL_options(
            this: Option<Allocated<Self>>,
            coordinator: Option<&NSPersistentStoreCoordinator>,
            configurationName: Option<&NSString>,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Id<Self, Shared>;

        #[method(load:)]
        pub unsafe fn load(&self) -> Result<(), Id<NSError, Shared>>;

        #[method(save:)]
        pub unsafe fn save(&self) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics New newCacheNodeForManagedObject:)]
        pub unsafe fn newCacheNodeForManagedObject(
            &self,
            managedObject: &NSManagedObject,
        ) -> Id<NSAtomicStoreCacheNode, Shared>;

        #[method(updateCacheNode:fromManagedObject:)]
        pub unsafe fn updateCacheNode_fromManagedObject(
            &self,
            node: &NSAtomicStoreCacheNode,
            managedObject: &NSManagedObject,
        );

        #[method_id(@__retain_semantics Other cacheNodes)]
        pub unsafe fn cacheNodes(&self) -> Id<NSSet<NSAtomicStoreCacheNode>, Shared>;

        #[method(addCacheNodes:)]
        pub unsafe fn addCacheNodes(&self, cacheNodes: &NSSet<NSAtomicStoreCacheNode>);

        #[method(willRemoveCacheNodes:)]
        pub unsafe fn willRemoveCacheNodes(&self, cacheNodes: &NSSet<NSAtomicStoreCacheNode>);

        #[method_id(@__retain_semantics Other cacheNodeForObjectID:)]
        pub unsafe fn cacheNodeForObjectID(
            &self,
            objectID: &NSManagedObjectID,
        ) -> Option<Id<NSAtomicStoreCacheNode, Shared>>;

        #[method_id(@__retain_semantics Other objectIDForEntity:referenceObject:)]
        pub unsafe fn objectIDForEntity_referenceObject(
            &self,
            entity: &NSEntityDescription,
            data: &Object,
        ) -> Id<NSManagedObjectID, Shared>;

        #[method_id(@__retain_semantics New newReferenceObjectForManagedObject:)]
        pub unsafe fn newReferenceObjectForManagedObject(
            &self,
            managedObject: &NSManagedObject,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other referenceObjectForObjectID:)]
        pub unsafe fn referenceObjectForObjectID(
            &self,
            objectID: &NSManagedObjectID,
        ) -> Id<Object, Shared>;
    }
);
