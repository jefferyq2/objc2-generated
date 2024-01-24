//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentContainer")]
    pub struct NSPersistentContainer;

    #[cfg(feature = "CoreData_NSPersistentContainer")]
    unsafe impl ClassType for NSPersistentContainer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSPersistentContainer")]
unsafe impl Send for NSPersistentContainer {}

#[cfg(feature = "CoreData_NSPersistentContainer")]
unsafe impl Sync for NSPersistentContainer {}

#[cfg(feature = "CoreData_NSPersistentContainer")]
unsafe impl NSObjectProtocol for NSPersistentContainer {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentContainer")]
    unsafe impl NSPersistentContainer {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other persistentContainerWithName:)]
        pub unsafe fn persistentContainerWithName(name: &NSString) -> Id<Self>;

        #[cfg(all(
            feature = "CoreData_NSManagedObjectModel",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other persistentContainerWithName:managedObjectModel:)]
        pub unsafe fn persistentContainerWithName_managedObjectModel(
            name: &NSString,
            model: &NSManagedObjectModel,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other defaultDirectoryURL)]
        pub unsafe fn defaultDirectoryURL() -> Id<NSURL>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics Other viewContext)]
        pub unsafe fn viewContext(&self) -> Id<NSManagedObjectContext>;

        #[cfg(feature = "CoreData_NSManagedObjectModel")]
        #[method_id(@__retain_semantics Other managedObjectModel)]
        pub unsafe fn managedObjectModel(&self) -> Id<NSManagedObjectModel>;

        #[cfg(feature = "CoreData_NSPersistentStoreCoordinator")]
        #[method_id(@__retain_semantics Other persistentStoreCoordinator)]
        pub unsafe fn persistentStoreCoordinator(&self) -> Id<NSPersistentStoreCoordinator>;

        #[cfg(all(
            feature = "CoreData_NSPersistentStoreDescription",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other persistentStoreDescriptions)]
        pub unsafe fn persistentStoreDescriptions(
            &self,
        ) -> Id<NSArray<NSPersistentStoreDescription>>;

        #[cfg(all(
            feature = "CoreData_NSPersistentStoreDescription",
            feature = "Foundation_NSArray"
        ))]
        #[method(setPersistentStoreDescriptions:)]
        pub unsafe fn setPersistentStoreDescriptions(
            &self,
            persistent_store_descriptions: &NSArray<NSPersistentStoreDescription>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithName:)]
        pub unsafe fn initWithName(this: Allocated<Self>, name: &NSString) -> Id<Self>;

        #[cfg(all(
            feature = "CoreData_NSManagedObjectModel",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithName:managedObjectModel:)]
        pub unsafe fn initWithName_managedObjectModel(
            this: Allocated<Self>,
            name: &NSString,
            model: &NSManagedObjectModel,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CoreData_NSPersistentStoreDescription",
            feature = "Foundation_NSError"
        ))]
        #[method(loadPersistentStoresWithCompletionHandler:)]
        pub unsafe fn loadPersistentStoresWithCompletionHandler(
            &self,
            block: &Block<dyn Fn(NonNull<NSPersistentStoreDescription>, *mut NSError)>,
        );

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method_id(@__retain_semantics New newBackgroundContext)]
        pub unsafe fn newBackgroundContext(&self) -> Id<NSManagedObjectContext>;

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[method(performBackgroundTask:)]
        pub unsafe fn performBackgroundTask(
            &self,
            block: &Block<dyn Fn(NonNull<NSManagedObjectContext>)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSPersistentContainer")]
    unsafe impl NSPersistentContainer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
