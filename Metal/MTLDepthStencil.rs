//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLCompareFunction(pub NSUInteger);
impl MTLCompareFunction {
    #[doc(alias = "MTLCompareFunctionNever")]
    pub const Never: Self = Self(0);
    #[doc(alias = "MTLCompareFunctionLess")]
    pub const Less: Self = Self(1);
    #[doc(alias = "MTLCompareFunctionEqual")]
    pub const Equal: Self = Self(2);
    #[doc(alias = "MTLCompareFunctionLessEqual")]
    pub const LessEqual: Self = Self(3);
    #[doc(alias = "MTLCompareFunctionGreater")]
    pub const Greater: Self = Self(4);
    #[doc(alias = "MTLCompareFunctionNotEqual")]
    pub const NotEqual: Self = Self(5);
    #[doc(alias = "MTLCompareFunctionGreaterEqual")]
    pub const GreaterEqual: Self = Self(6);
    #[doc(alias = "MTLCompareFunctionAlways")]
    pub const Always: Self = Self(7);
}

unsafe impl Encode for MTLCompareFunction {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLCompareFunction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLStencilOperation(pub NSUInteger);
impl MTLStencilOperation {
    #[doc(alias = "MTLStencilOperationKeep")]
    pub const Keep: Self = Self(0);
    #[doc(alias = "MTLStencilOperationZero")]
    pub const Zero: Self = Self(1);
    #[doc(alias = "MTLStencilOperationReplace")]
    pub const Replace: Self = Self(2);
    #[doc(alias = "MTLStencilOperationIncrementClamp")]
    pub const IncrementClamp: Self = Self(3);
    #[doc(alias = "MTLStencilOperationDecrementClamp")]
    pub const DecrementClamp: Self = Self(4);
    #[doc(alias = "MTLStencilOperationInvert")]
    pub const Invert: Self = Self(5);
    #[doc(alias = "MTLStencilOperationIncrementWrap")]
    pub const IncrementWrap: Self = Self(6);
    #[doc(alias = "MTLStencilOperationDecrementWrap")]
    pub const DecrementWrap: Self = Self(7);
}

unsafe impl Encode for MTLStencilOperation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLStencilOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStencilDescriptor;

    unsafe impl ClassType for MTLStencilDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MTLStencilDescriptor {}

unsafe impl NSObjectProtocol for MTLStencilDescriptor {}

extern_methods!(
    unsafe impl MTLStencilDescriptor {
        #[method(stencilCompareFunction)]
        pub fn stencilCompareFunction(&self) -> MTLCompareFunction;

        #[method(setStencilCompareFunction:)]
        pub fn setStencilCompareFunction(&self, stencil_compare_function: MTLCompareFunction);

        #[method(stencilFailureOperation)]
        pub fn stencilFailureOperation(&self) -> MTLStencilOperation;

        #[method(setStencilFailureOperation:)]
        pub fn setStencilFailureOperation(&self, stencil_failure_operation: MTLStencilOperation);

        #[method(depthFailureOperation)]
        pub fn depthFailureOperation(&self) -> MTLStencilOperation;

        #[method(setDepthFailureOperation:)]
        pub fn setDepthFailureOperation(&self, depth_failure_operation: MTLStencilOperation);

        #[method(depthStencilPassOperation)]
        pub fn depthStencilPassOperation(&self) -> MTLStencilOperation;

        #[method(setDepthStencilPassOperation:)]
        pub fn setDepthStencilPassOperation(
            &self,
            depth_stencil_pass_operation: MTLStencilOperation,
        );

        #[method(readMask)]
        pub fn readMask(&self) -> u32;

        #[method(setReadMask:)]
        pub fn setReadMask(&self, read_mask: u32);

        #[method(writeMask)]
        pub fn writeMask(&self) -> u32;

        #[method(setWriteMask:)]
        pub fn setWriteMask(&self, write_mask: u32);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLStencilDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLDepthStencilDescriptor;

    unsafe impl ClassType for MTLDepthStencilDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MTLDepthStencilDescriptor {}

unsafe impl NSObjectProtocol for MTLDepthStencilDescriptor {}

extern_methods!(
    unsafe impl MTLDepthStencilDescriptor {
        #[method(depthCompareFunction)]
        pub fn depthCompareFunction(&self) -> MTLCompareFunction;

        #[method(setDepthCompareFunction:)]
        pub fn setDepthCompareFunction(&self, depth_compare_function: MTLCompareFunction);

        #[method(isDepthWriteEnabled)]
        pub fn isDepthWriteEnabled(&self) -> bool;

        #[method(setDepthWriteEnabled:)]
        pub fn setDepthWriteEnabled(&self, depth_write_enabled: bool);

        #[method_id(@__retain_semantics Other frontFaceStencil)]
        pub fn frontFaceStencil(&self) -> Id<MTLStencilDescriptor>;

        #[method(setFrontFaceStencil:)]
        pub fn setFrontFaceStencil(&self, front_face_stencil: Option<&MTLStencilDescriptor>);

        #[method_id(@__retain_semantics Other backFaceStencil)]
        pub fn backFaceStencil(&self) -> Id<MTLStencilDescriptor>;

        #[method(setBackFaceStencil:)]
        pub fn setBackFaceStencil(&self, back_face_stencil: Option<&MTLStencilDescriptor>);

        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString>>;

        #[method(setLabel:)]
        pub fn setLabel(&self, label: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLDepthStencilDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLDepthStencilState: NSObjectProtocol {
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;
    }

    unsafe impl ProtocolType for dyn MTLDepthStencilState {}
);
