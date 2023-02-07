//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLVertexFormat {
        MTLVertexFormatInvalid = 0,
        MTLVertexFormatUChar2 = 1,
        MTLVertexFormatUChar3 = 2,
        MTLVertexFormatUChar4 = 3,
        MTLVertexFormatChar2 = 4,
        MTLVertexFormatChar3 = 5,
        MTLVertexFormatChar4 = 6,
        MTLVertexFormatUChar2Normalized = 7,
        MTLVertexFormatUChar3Normalized = 8,
        MTLVertexFormatUChar4Normalized = 9,
        MTLVertexFormatChar2Normalized = 10,
        MTLVertexFormatChar3Normalized = 11,
        MTLVertexFormatChar4Normalized = 12,
        MTLVertexFormatUShort2 = 13,
        MTLVertexFormatUShort3 = 14,
        MTLVertexFormatUShort4 = 15,
        MTLVertexFormatShort2 = 16,
        MTLVertexFormatShort3 = 17,
        MTLVertexFormatShort4 = 18,
        MTLVertexFormatUShort2Normalized = 19,
        MTLVertexFormatUShort3Normalized = 20,
        MTLVertexFormatUShort4Normalized = 21,
        MTLVertexFormatShort2Normalized = 22,
        MTLVertexFormatShort3Normalized = 23,
        MTLVertexFormatShort4Normalized = 24,
        MTLVertexFormatHalf2 = 25,
        MTLVertexFormatHalf3 = 26,
        MTLVertexFormatHalf4 = 27,
        MTLVertexFormatFloat = 28,
        MTLVertexFormatFloat2 = 29,
        MTLVertexFormatFloat3 = 30,
        MTLVertexFormatFloat4 = 31,
        MTLVertexFormatInt = 32,
        MTLVertexFormatInt2 = 33,
        MTLVertexFormatInt3 = 34,
        MTLVertexFormatInt4 = 35,
        MTLVertexFormatUInt = 36,
        MTLVertexFormatUInt2 = 37,
        MTLVertexFormatUInt3 = 38,
        MTLVertexFormatUInt4 = 39,
        MTLVertexFormatInt1010102Normalized = 40,
        MTLVertexFormatUInt1010102Normalized = 41,
        MTLVertexFormatUChar4Normalized_BGRA = 42,
        MTLVertexFormatUChar = 45,
        MTLVertexFormatChar = 46,
        MTLVertexFormatUCharNormalized = 47,
        MTLVertexFormatCharNormalized = 48,
        MTLVertexFormatUShort = 49,
        MTLVertexFormatShort = 50,
        MTLVertexFormatUShortNormalized = 51,
        MTLVertexFormatShortNormalized = 52,
        MTLVertexFormatHalf = 53,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLVertexStepFunction {
        MTLVertexStepFunctionConstant = 0,
        MTLVertexStepFunctionPerVertex = 1,
        MTLVertexStepFunctionPerInstance = 2,
        MTLVertexStepFunctionPerPatch = 3,
        MTLVertexStepFunctionPerPatchControlPoint = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLVertexBufferLayoutDescriptor")]
    pub struct MTLVertexBufferLayoutDescriptor;

    #[cfg(feature = "Metal_MTLVertexBufferLayoutDescriptor")]
    unsafe impl ClassType for MTLVertexBufferLayoutDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLVertexBufferLayoutDescriptor")]
unsafe impl NSObjectProtocol for MTLVertexBufferLayoutDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLVertexBufferLayoutDescriptor")]
    unsafe impl MTLVertexBufferLayoutDescriptor {
        #[method(stride)]
        pub fn stride(&self) -> NSUInteger;

        #[method(setStride:)]
        pub unsafe fn setStride(&self, stride: NSUInteger);

        #[method(stepFunction)]
        pub fn stepFunction(&self) -> MTLVertexStepFunction;

        #[method(setStepFunction:)]
        pub unsafe fn setStepFunction(&self, step_function: MTLVertexStepFunction);

        #[method(stepRate)]
        pub fn stepRate(&self) -> NSUInteger;

        #[method(setStepRate:)]
        pub unsafe fn setStepRate(&self, step_rate: NSUInteger);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLVertexBufferLayoutDescriptorArray")]
    pub struct MTLVertexBufferLayoutDescriptorArray;

    #[cfg(feature = "Metal_MTLVertexBufferLayoutDescriptorArray")]
    unsafe impl ClassType for MTLVertexBufferLayoutDescriptorArray {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLVertexBufferLayoutDescriptorArray")]
unsafe impl NSObjectProtocol for MTLVertexBufferLayoutDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLVertexBufferLayoutDescriptorArray")]
    unsafe impl MTLVertexBufferLayoutDescriptorArray {
        #[cfg(feature = "Metal_MTLVertexBufferLayoutDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            index: NSUInteger,
        ) -> Id<MTLVertexBufferLayoutDescriptor>;

        #[cfg(feature = "Metal_MTLVertexBufferLayoutDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            buffer_desc: Option<&MTLVertexBufferLayoutDescriptor>,
            index: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLVertexAttributeDescriptor")]
    pub struct MTLVertexAttributeDescriptor;

    #[cfg(feature = "Metal_MTLVertexAttributeDescriptor")]
    unsafe impl ClassType for MTLVertexAttributeDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLVertexAttributeDescriptor")]
unsafe impl NSObjectProtocol for MTLVertexAttributeDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLVertexAttributeDescriptor")]
    unsafe impl MTLVertexAttributeDescriptor {
        #[method(format)]
        pub fn format(&self) -> MTLVertexFormat;

        #[method(setFormat:)]
        pub fn setFormat(&self, format: MTLVertexFormat);

        #[method(offset)]
        pub fn offset(&self) -> NSUInteger;

        #[method(setOffset:)]
        pub unsafe fn setOffset(&self, offset: NSUInteger);

        #[method(bufferIndex)]
        pub fn bufferIndex(&self) -> NSUInteger;

        #[method(setBufferIndex:)]
        pub unsafe fn setBufferIndex(&self, buffer_index: NSUInteger);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLVertexAttributeDescriptorArray")]
    pub struct MTLVertexAttributeDescriptorArray;

    #[cfg(feature = "Metal_MTLVertexAttributeDescriptorArray")]
    unsafe impl ClassType for MTLVertexAttributeDescriptorArray {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLVertexAttributeDescriptorArray")]
unsafe impl NSObjectProtocol for MTLVertexAttributeDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLVertexAttributeDescriptorArray")]
    unsafe impl MTLVertexAttributeDescriptorArray {
        #[cfg(feature = "Metal_MTLVertexAttributeDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            index: NSUInteger,
        ) -> Id<MTLVertexAttributeDescriptor>;

        #[cfg(feature = "Metal_MTLVertexAttributeDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attribute_desc: Option<&MTLVertexAttributeDescriptor>,
            index: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLVertexDescriptor")]
    pub struct MTLVertexDescriptor;

    #[cfg(feature = "Metal_MTLVertexDescriptor")]
    unsafe impl ClassType for MTLVertexDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLVertexDescriptor")]
unsafe impl NSObjectProtocol for MTLVertexDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLVertexDescriptor")]
    unsafe impl MTLVertexDescriptor {
        #[method_id(@__retain_semantics Other vertexDescriptor)]
        pub fn vertexDescriptor() -> Id<MTLVertexDescriptor>;

        #[cfg(feature = "Metal_MTLVertexBufferLayoutDescriptorArray")]
        #[method_id(@__retain_semantics Other layouts)]
        pub fn layouts(&self) -> Id<MTLVertexBufferLayoutDescriptorArray>;

        #[cfg(feature = "Metal_MTLVertexAttributeDescriptorArray")]
        #[method_id(@__retain_semantics Other attributes)]
        pub fn attributes(&self) -> Id<MTLVertexAttributeDescriptorArray>;

        #[method(reset)]
        pub fn reset(&self);
    }
);
