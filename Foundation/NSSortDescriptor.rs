//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSSortDescriptor")]
    pub struct NSSortDescriptor;

    #[cfg(feature = "Foundation_NSSortDescriptor")]
    unsafe impl ClassType for NSSortDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSSortDescriptor")]
unsafe impl NSCoding for NSSortDescriptor {}

#[cfg(feature = "Foundation_NSSortDescriptor")]
unsafe impl NSObjectProtocol for NSSortDescriptor {}

#[cfg(feature = "Foundation_NSSortDescriptor")]
unsafe impl NSSecureCoding for NSSortDescriptor {}

extern_methods!(
    #[cfg(feature = "Foundation_NSSortDescriptor")]
    unsafe impl NSSortDescriptor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sortDescriptorWithKey:ascending:)]
        pub unsafe fn sortDescriptorWithKey_ascending(
            key: Option<&NSString>,
            ascending: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sortDescriptorWithKey:ascending:selector:)]
        pub unsafe fn sortDescriptorWithKey_ascending_selector(
            key: Option<&NSString>,
            ascending: bool,
            selector: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithKey:ascending:)]
        pub unsafe fn initWithKey_ascending(
            this: Option<Allocated<Self>>,
            key: Option<&NSString>,
            ascending: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithKey:ascending:selector:)]
        pub unsafe fn initWithKey_ascending_selector(
            this: Option<Allocated<Self>>,
            key: Option<&NSString>,
            ascending: bool,
            selector: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Option<Id<NSString>>;

        #[method(ascending)]
        pub unsafe fn ascending(&self) -> bool;

        #[method(selector)]
        pub unsafe fn selector(&self) -> Option<Sel>;

        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sortDescriptorWithKey:ascending:comparator:)]
        pub unsafe fn sortDescriptorWithKey_ascending_comparator(
            key: Option<&NSString>,
            ascending: bool,
            cmptr: NSComparator,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithKey:ascending:comparator:)]
        pub unsafe fn initWithKey_ascending_comparator(
            this: Option<Allocated<Self>>,
            key: Option<&NSString>,
            ascending: bool,
            cmptr: NSComparator,
        ) -> Id<Self>;

        #[method(comparator)]
        pub unsafe fn comparator(&self) -> NSComparator;

        #[method(compareObject:toObject:)]
        pub unsafe fn compareObject_toObject(
            &self,
            object1: &Object,
            object2: &Object,
        ) -> NSComparisonResult;

        #[method_id(@__retain_semantics Other reversedSortDescriptor)]
        pub unsafe fn reversedSortDescriptor(&self) -> Id<Object>;
    }
);

extern_methods!(
    /// NSSortDescriptorSorting
    #[cfg(feature = "Foundation_NSSet")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSSet<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortedArrayUsingDescriptors:)]
        pub unsafe fn sortedArrayUsingDescriptors(
            &self,
            sort_descriptors: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<ObjectType>>;
    }
);

extern_methods!(
    /// NSSortDescriptorSorting
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSSortDescriptor")]
        #[method_id(@__retain_semantics Other sortedArrayUsingDescriptors:)]
        pub unsafe fn sortedArrayUsingDescriptors(
            &self,
            sort_descriptors: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<ObjectType>>;
    }
);

extern_methods!(
    /// NSSortDescriptorSorting
    #[cfg(feature = "Foundation_NSMutableArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableArray<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method(sortUsingDescriptors:)]
        pub unsafe fn sortUsingDescriptors(&self, sort_descriptors: &NSArray<NSSortDescriptor>);
    }
);

extern_methods!(
    /// NSKeyValueSorting
    #[cfg(feature = "Foundation_NSOrderedSet")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSOrderedSet<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortedArrayUsingDescriptors:)]
        pub unsafe fn sortedArrayUsingDescriptors(
            &self,
            sort_descriptors: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<ObjectType>>;
    }
);

extern_methods!(
    /// NSKeyValueSorting
    #[cfg(feature = "Foundation_NSMutableOrderedSet")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableOrderedSet<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method(sortUsingDescriptors:)]
        pub unsafe fn sortUsingDescriptors(&self, sort_descriptors: &NSArray<NSSortDescriptor>);
    }
);
