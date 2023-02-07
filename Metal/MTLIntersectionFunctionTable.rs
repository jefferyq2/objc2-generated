//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLIntersectionFunctionSignature {
        MTLIntersectionFunctionSignatureNone = 0,
        MTLIntersectionFunctionSignatureInstancing = 1 << 0,
        MTLIntersectionFunctionSignatureTriangleData = 1 << 1,
        MTLIntersectionFunctionSignatureWorldSpaceData = 1 << 2,
        MTLIntersectionFunctionSignatureInstanceMotion = 1 << 3,
        MTLIntersectionFunctionSignaturePrimitiveMotion = 1 << 4,
        MTLIntersectionFunctionSignatureExtendedLimits = 1 << 5,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLIntersectionFunctionTableDescriptor")]
    pub struct MTLIntersectionFunctionTableDescriptor;

    #[cfg(feature = "Metal_MTLIntersectionFunctionTableDescriptor")]
    unsafe impl ClassType for MTLIntersectionFunctionTableDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLIntersectionFunctionTableDescriptor")]
unsafe impl NSObjectProtocol for MTLIntersectionFunctionTableDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLIntersectionFunctionTableDescriptor")]
    unsafe impl MTLIntersectionFunctionTableDescriptor {
        #[method_id(@__retain_semantics Other intersectionFunctionTableDescriptor)]
        pub unsafe fn intersectionFunctionTableDescriptor(
        ) -> Id<MTLIntersectionFunctionTableDescriptor>;

        #[method(functionCount)]
        pub unsafe fn functionCount(&self) -> NSUInteger;

        #[method(setFunctionCount:)]
        pub fn setFunctionCount(&self, function_count: NSUInteger);
    }
);

extern_protocol!(
    pub unsafe trait MTLIntersectionFunctionTable: MTLResource {
        #[method(setBuffer:offset:atIndex:)]
        unsafe fn setBuffer_offset_atIndex(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[method(setBuffers:offsets:withRange:)]
        unsafe fn setBuffers_offsets_withRange(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<NSUInteger>,
            range: NSRange,
        );

        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[method(setFunction:atIndex:)]
        fn setFunction_atIndex(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunctionHandle>>,
            index: NSUInteger,
        );

        #[method(setFunctions:withRange:)]
        unsafe fn setFunctions_withRange(
            &self,
            functions: NonNull<*const ProtocolObject<dyn MTLFunctionHandle>>,
            range: NSRange,
        );

        #[method(setOpaqueTriangleIntersectionFunctionWithSignature:atIndex:)]
        unsafe fn setOpaqueTriangleIntersectionFunctionWithSignature_atIndex(
            &self,
            signature: MTLIntersectionFunctionSignature,
            index: NSUInteger,
        );

        #[method(setOpaqueTriangleIntersectionFunctionWithSignature:withRange:)]
        unsafe fn setOpaqueTriangleIntersectionFunctionWithSignature_withRange(
            &self,
            signature: MTLIntersectionFunctionSignature,
            range: NSRange,
        );

        #[method(setVisibleFunctionTable:atBufferIndex:)]
        unsafe fn setVisibleFunctionTable_atBufferIndex(
            &self,
            function_table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_index: NSUInteger,
        );

        #[method(setVisibleFunctionTables:withBufferRange:)]
        unsafe fn setVisibleFunctionTables_withBufferRange(
            &self,
            function_tables: NonNull<*const ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_range: NSRange,
        );
    }

    unsafe impl ProtocolType for dyn MTLIntersectionFunctionTable {}
);
