//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSortDescriptor;

    unsafe impl ClassType for NSSortDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSSortDescriptor {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSSortDescriptor {}

unsafe impl NSObjectProtocol for NSSortDescriptor {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSSortDescriptor {}

extern_methods!(
    unsafe impl NSSortDescriptor {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other sortDescriptorWithKey:ascending:)]
        pub unsafe fn sortDescriptorWithKey_ascending(
            key: Option<&NSString>,
            ascending: bool,
        ) -> Id<Self>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other sortDescriptorWithKey:ascending:selector:)]
        pub unsafe fn sortDescriptorWithKey_ascending_selector(
            key: Option<&NSString>,
            ascending: bool,
            selector: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithKey:ascending:)]
        pub unsafe fn initWithKey_ascending(
            this: Allocated<Self>,
            key: Option<&NSString>,
            ascending: bool,
        ) -> Id<Self>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithKey:ascending:selector:)]
        pub unsafe fn initWithKey_ascending_selector(
            this: Allocated<Self>,
            key: Option<&NSString>,
            ascending: bool,
            selector: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Option<Id<NSString>>;

        #[method(ascending)]
        pub unsafe fn ascending(&self) -> bool;

        #[method(selector)]
        pub unsafe fn selector(&self) -> Option<Sel>;

        #[method(allowEvaluation)]
        pub unsafe fn allowEvaluation(&self);

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSString", feature = "block2"))]
        #[method_id(@__retain_semantics Other sortDescriptorWithKey:ascending:comparator:)]
        pub unsafe fn sortDescriptorWithKey_ascending_comparator(
            key: Option<&NSString>,
            ascending: bool,
            cmptr: NSComparator,
        ) -> Id<Self>;

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSString", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithKey:ascending:comparator:)]
        pub unsafe fn initWithKey_ascending_comparator(
            this: Allocated<Self>,
            key: Option<&NSString>,
            ascending: bool,
            cmptr: NSComparator,
        ) -> Id<Self>;

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(comparator)]
        pub unsafe fn comparator(&self) -> NSComparator;

        #[cfg(feature = "NSObjCRuntime")]
        #[method(compareObject:toObject:)]
        pub unsafe fn compareObject_toObject(
            &self,
            object1: &AnyObject,
            object2: &AnyObject,
        ) -> NSComparisonResult;

        #[method_id(@__retain_semantics Other reversedSortDescriptor)]
        pub unsafe fn reversedSortDescriptor(&self) -> Id<AnyObject>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSortDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSSortDescriptorSorting
    #[cfg(feature = "NSSet")]
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other sortedArrayUsingDescriptors:)]
        pub unsafe fn sortedArrayUsingDescriptors(
            &self,
            sort_descriptors: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<ObjectType>>;
    }
);

extern_methods!(
    /// NSSortDescriptorSorting
    #[cfg(feature = "NSArray")]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method_id(@__retain_semantics Other sortedArrayUsingDescriptors:)]
        pub unsafe fn sortedArrayUsingDescriptors(
            &self,
            sort_descriptors: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<ObjectType>>;
    }
);

extern_methods!(
    /// NSSortDescriptorSorting
    #[cfg(feature = "NSArray")]
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method(sortUsingDescriptors:)]
        pub unsafe fn sortUsingDescriptors(&mut self, sort_descriptors: &NSArray<NSSortDescriptor>);
    }
);

extern_methods!(
    /// NSKeyValueSorting
    #[cfg(feature = "NSOrderedSet")]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other sortedArrayUsingDescriptors:)]
        pub unsafe fn sortedArrayUsingDescriptors(
            &self,
            sort_descriptors: &NSArray<NSSortDescriptor>,
        ) -> Id<NSArray<ObjectType>>;
    }
);

extern_methods!(
    /// NSKeyValueSorting
    #[cfg(feature = "NSOrderedSet")]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[cfg(feature = "NSArray")]
        #[method(sortUsingDescriptors:)]
        pub unsafe fn sortUsingDescriptors(&mut self, sort_descriptors: &NSArray<NSSortDescriptor>);
    }
);
