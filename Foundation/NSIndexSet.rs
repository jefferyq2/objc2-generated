//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSIndexSet")]
    pub struct NSIndexSet;

    #[cfg(feature = "Foundation_NSIndexSet")]
    unsafe impl ClassType for NSIndexSet {
        type Super = NSObject;
        type Mutability = ImmutableWithMutableSubclass<NSMutableIndexSet>;
    }
);

#[cfg(feature = "Foundation_NSIndexSet")]
unsafe impl NSCoding for NSIndexSet {}

#[cfg(feature = "Foundation_NSIndexSet")]
unsafe impl NSCopying for NSIndexSet {}

#[cfg(feature = "Foundation_NSIndexSet")]
unsafe impl NSMutableCopying for NSIndexSet {}

#[cfg(feature = "Foundation_NSIndexSet")]
unsafe impl NSObjectProtocol for NSIndexSet {}

#[cfg(feature = "Foundation_NSIndexSet")]
unsafe impl NSSecureCoding for NSIndexSet {}

extern_methods!(
    #[cfg(feature = "Foundation_NSIndexSet")]
    unsafe impl NSIndexSet {
        #[method_id(@__retain_semantics Other indexSet)]
        pub unsafe fn indexSet() -> Id<Self>;

        #[method_id(@__retain_semantics Other indexSetWithIndex:)]
        pub unsafe fn indexSetWithIndex(value: NSUInteger) -> Id<Self>;

        #[method_id(@__retain_semantics Other indexSetWithIndexesInRange:)]
        pub unsafe fn indexSetWithIndexesInRange(range: NSRange) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithIndexesInRange:)]
        pub unsafe fn initWithIndexesInRange(this: Allocated<Self>, range: NSRange) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithIndexSet:)]
        pub unsafe fn initWithIndexSet(this: Allocated<Self>, index_set: &NSIndexSet) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithIndex:)]
        pub unsafe fn initWithIndex(this: Allocated<Self>, value: NSUInteger) -> Id<Self>;

        #[method(isEqualToIndexSet:)]
        pub unsafe fn isEqualToIndexSet(&self, index_set: &NSIndexSet) -> bool;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method(firstIndex)]
        pub unsafe fn firstIndex(&self) -> NSUInteger;

        #[method(lastIndex)]
        pub unsafe fn lastIndex(&self) -> NSUInteger;

        #[method(indexGreaterThanIndex:)]
        pub unsafe fn indexGreaterThanIndex(&self, value: NSUInteger) -> NSUInteger;

        #[method(indexLessThanIndex:)]
        pub unsafe fn indexLessThanIndex(&self, value: NSUInteger) -> NSUInteger;

        #[method(indexGreaterThanOrEqualToIndex:)]
        pub unsafe fn indexGreaterThanOrEqualToIndex(&self, value: NSUInteger) -> NSUInteger;

        #[method(indexLessThanOrEqualToIndex:)]
        pub unsafe fn indexLessThanOrEqualToIndex(&self, value: NSUInteger) -> NSUInteger;

        #[method(getIndexes:maxCount:inIndexRange:)]
        pub unsafe fn getIndexes_maxCount_inIndexRange(
            &self,
            index_buffer: NonNull<NSUInteger>,
            buffer_size: NSUInteger,
            range: NSRangePointer,
        ) -> NSUInteger;

        #[method(countOfIndexesInRange:)]
        pub unsafe fn countOfIndexesInRange(&self, range: NSRange) -> NSUInteger;

        #[method(containsIndex:)]
        pub unsafe fn containsIndex(&self, value: NSUInteger) -> bool;

        #[method(containsIndexesInRange:)]
        pub unsafe fn containsIndexesInRange(&self, range: NSRange) -> bool;

        #[method(containsIndexes:)]
        pub unsafe fn containsIndexes(&self, index_set: &NSIndexSet) -> bool;

        #[method(intersectsIndexesInRange:)]
        pub unsafe fn intersectsIndexesInRange(&self, range: NSRange) -> bool;

        #[method(enumerateIndexesUsingBlock:)]
        pub unsafe fn enumerateIndexesUsingBlock(
            &self,
            block: &Block<dyn Fn(NSUInteger, NonNull<Bool>) + '_>,
        );

        #[method(enumerateIndexesWithOptions:usingBlock:)]
        pub unsafe fn enumerateIndexesWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &Block<dyn Fn(NSUInteger, NonNull<Bool>) + '_>,
        );

        #[method(enumerateIndexesInRange:options:usingBlock:)]
        pub unsafe fn enumerateIndexesInRange_options_usingBlock(
            &self,
            range: NSRange,
            opts: NSEnumerationOptions,
            block: &Block<dyn Fn(NSUInteger, NonNull<Bool>) + '_>,
        );

        #[method(indexPassingTest:)]
        pub unsafe fn indexPassingTest(
            &self,
            predicate: &Block<dyn Fn(NSUInteger, NonNull<Bool>) -> Bool + '_>,
        ) -> NSUInteger;

        #[method(indexWithOptions:passingTest:)]
        pub unsafe fn indexWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &Block<dyn Fn(NSUInteger, NonNull<Bool>) -> Bool + '_>,
        ) -> NSUInteger;

        #[method(indexInRange:options:passingTest:)]
        pub unsafe fn indexInRange_options_passingTest(
            &self,
            range: NSRange,
            opts: NSEnumerationOptions,
            predicate: &Block<dyn Fn(NSUInteger, NonNull<Bool>) -> Bool + '_>,
        ) -> NSUInteger;

        #[method_id(@__retain_semantics Other indexesPassingTest:)]
        pub unsafe fn indexesPassingTest(
            &self,
            predicate: &Block<dyn Fn(NSUInteger, NonNull<Bool>) -> Bool + '_>,
        ) -> Id<NSIndexSet>;

        #[method_id(@__retain_semantics Other indexesWithOptions:passingTest:)]
        pub unsafe fn indexesWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &Block<dyn Fn(NSUInteger, NonNull<Bool>) -> Bool + '_>,
        ) -> Id<NSIndexSet>;

        #[method_id(@__retain_semantics Other indexesInRange:options:passingTest:)]
        pub unsafe fn indexesInRange_options_passingTest(
            &self,
            range: NSRange,
            opts: NSEnumerationOptions,
            predicate: &Block<dyn Fn(NSUInteger, NonNull<Bool>) -> Bool + '_>,
        ) -> Id<NSIndexSet>;

        #[method(enumerateRangesUsingBlock:)]
        pub unsafe fn enumerateRangesUsingBlock(
            &self,
            block: &Block<dyn Fn(NSRange, NonNull<Bool>) + '_>,
        );

        #[method(enumerateRangesWithOptions:usingBlock:)]
        pub unsafe fn enumerateRangesWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &Block<dyn Fn(NSRange, NonNull<Bool>) + '_>,
        );

        #[method(enumerateRangesInRange:options:usingBlock:)]
        pub unsafe fn enumerateRangesInRange_options_usingBlock(
            &self,
            range: NSRange,
            opts: NSEnumerationOptions,
            block: &Block<dyn Fn(NSRange, NonNull<Bool>) + '_>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSIndexSet")]
    unsafe impl NSIndexSet {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSIndexSet")]
    pub struct NSMutableIndexSet;

    #[cfg(feature = "Foundation_NSIndexSet")]
    unsafe impl ClassType for NSMutableIndexSet {
        #[inherits(NSObject)]
        type Super = NSIndexSet;
        type Mutability = MutableWithImmutableSuperclass<NSIndexSet>;
    }
);

#[cfg(feature = "Foundation_NSMutableIndexSet")]
unsafe impl NSCoding for NSMutableIndexSet {}

#[cfg(feature = "Foundation_NSMutableIndexSet")]
unsafe impl NSCopying for NSMutableIndexSet {}

#[cfg(feature = "Foundation_NSMutableIndexSet")]
unsafe impl NSMutableCopying for NSMutableIndexSet {}

#[cfg(feature = "Foundation_NSMutableIndexSet")]
unsafe impl NSObjectProtocol for NSMutableIndexSet {}

#[cfg(feature = "Foundation_NSMutableIndexSet")]
unsafe impl NSSecureCoding for NSMutableIndexSet {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMutableIndexSet")]
    unsafe impl NSMutableIndexSet {
        #[method(addIndexes:)]
        pub unsafe fn addIndexes(&mut self, index_set: &NSIndexSet);

        #[method(removeIndexes:)]
        pub unsafe fn removeIndexes(&mut self, index_set: &NSIndexSet);

        #[method(removeAllIndexes)]
        pub unsafe fn removeAllIndexes(&mut self);

        #[method(addIndex:)]
        pub unsafe fn addIndex(&mut self, value: NSUInteger);

        #[method(removeIndex:)]
        pub unsafe fn removeIndex(&mut self, value: NSUInteger);

        #[method(addIndexesInRange:)]
        pub unsafe fn addIndexesInRange(&mut self, range: NSRange);

        #[method(removeIndexesInRange:)]
        pub unsafe fn removeIndexesInRange(&mut self, range: NSRange);

        #[method(shiftIndexesStartingAtIndex:by:)]
        pub unsafe fn shiftIndexesStartingAtIndex_by(
            &mut self,
            index: NSUInteger,
            delta: NSInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSIndexSet`
    #[cfg(feature = "Foundation_NSMutableIndexSet")]
    unsafe impl NSMutableIndexSet {
        #[method_id(@__retain_semantics Other indexSet)]
        pub unsafe fn indexSet() -> Id<Self>;

        #[method_id(@__retain_semantics Other indexSetWithIndex:)]
        pub unsafe fn indexSetWithIndex(value: NSUInteger) -> Id<Self>;

        #[method_id(@__retain_semantics Other indexSetWithIndexesInRange:)]
        pub unsafe fn indexSetWithIndexesInRange(range: NSRange) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithIndexesInRange:)]
        pub unsafe fn initWithIndexesInRange(this: Allocated<Self>, range: NSRange) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithIndexSet:)]
        pub unsafe fn initWithIndexSet(this: Allocated<Self>, index_set: &NSIndexSet) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithIndex:)]
        pub unsafe fn initWithIndex(this: Allocated<Self>, value: NSUInteger) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMutableIndexSet")]
    unsafe impl NSMutableIndexSet {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
