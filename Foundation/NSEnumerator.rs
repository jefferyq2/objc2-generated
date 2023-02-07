//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_struct!(
    #[encoding_name("?")]
    pub struct NSFastEnumerationState {
        pub state: c_ulong,
        pub itemsPtr: *mut *mut Object,
        pub mutationsPtr: *mut c_ulong,
        pub extra: [c_ulong; 5],
    }
);

extern_protocol!(
    pub unsafe trait NSFastEnumeration {
        #[method(countByEnumeratingWithState:objects:count:)]
        unsafe fn countByEnumeratingWithState_objects_count(
            &self,
            state: NonNull<NSFastEnumerationState>,
            buffer: NonNull<*mut Object>,
            len: NSUInteger,
        ) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn NSFastEnumeration {}
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSEnumerator")]
    pub struct NSEnumerator<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "Foundation_NSEnumerator")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for NSEnumerator<ObjectType, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSEnumerator")]
unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> NSFastEnumeration
    for NSEnumerator<ObjectType, ObjectTypeOwnership>
{
}

#[cfg(feature = "Foundation_NSEnumerator")]
unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> NSObjectProtocol
    for NSEnumerator<ObjectType, ObjectTypeOwnership>
{
}

extern_methods!(
    #[cfg(feature = "Foundation_NSEnumerator")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSEnumerator<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other nextObject)]
        pub unsafe fn nextObject(&self) -> Option<Id<ObjectType, ObjectTypeOwnership>>;
    }
);

extern_methods!(
    /// NSExtendedEnumerator
    #[cfg(feature = "Foundation_NSEnumerator")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSEnumerator<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allObjects)]
        pub unsafe fn allObjects(&self) -> Id<NSArray<ObjectType>>;
    }
);
