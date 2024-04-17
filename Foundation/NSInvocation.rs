//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSInvocation;

    unsafe impl ClassType for NSInvocation {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSInvocation {}

extern_methods!(
    unsafe impl NSInvocation {
        #[cfg(feature = "NSMethodSignature")]
        #[method_id(@__retain_semantics Other invocationWithMethodSignature:)]
        pub unsafe fn invocationWithMethodSignature(sig: &NSMethodSignature) -> Id<NSInvocation>;

        #[cfg(feature = "NSMethodSignature")]
        #[method_id(@__retain_semantics Other methodSignature)]
        pub unsafe fn methodSignature(&self) -> Id<NSMethodSignature>;

        #[method(retainArguments)]
        pub unsafe fn retainArguments(&self);

        #[method(argumentsRetained)]
        pub unsafe fn argumentsRetained(&self) -> bool;

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<AnyObject>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[method(selector)]
        pub unsafe fn selector(&self) -> Sel;

        #[method(setSelector:)]
        pub unsafe fn setSelector(&self, selector: Sel);

        #[method(getReturnValue:)]
        pub unsafe fn getReturnValue(&self, ret_loc: NonNull<c_void>);

        #[method(setReturnValue:)]
        pub unsafe fn setReturnValue(&self, ret_loc: NonNull<c_void>);

        #[method(getArgument:atIndex:)]
        pub unsafe fn getArgument_atIndex(
            &self,
            argument_location: NonNull<c_void>,
            idx: NSInteger,
        );

        #[method(setArgument:atIndex:)]
        pub unsafe fn setArgument_atIndex(
            &self,
            argument_location: NonNull<c_void>,
            idx: NSInteger,
        );

        #[method(invoke)]
        pub unsafe fn invoke(&self);

        #[method(invokeWithTarget:)]
        pub unsafe fn invokeWithTarget(&self, target: &AnyObject);

        #[method(invokeUsingIMP:)]
        pub unsafe fn invokeUsingIMP(&self, imp: IMP);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSInvocation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
