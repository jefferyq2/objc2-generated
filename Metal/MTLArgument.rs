//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLDataType {
        MTLDataTypeNone = 0,
        MTLDataTypeStruct = 1,
        MTLDataTypeArray = 2,
        MTLDataTypeFloat = 3,
        MTLDataTypeFloat2 = 4,
        MTLDataTypeFloat3 = 5,
        MTLDataTypeFloat4 = 6,
        MTLDataTypeFloat2x2 = 7,
        MTLDataTypeFloat2x3 = 8,
        MTLDataTypeFloat2x4 = 9,
        MTLDataTypeFloat3x2 = 10,
        MTLDataTypeFloat3x3 = 11,
        MTLDataTypeFloat3x4 = 12,
        MTLDataTypeFloat4x2 = 13,
        MTLDataTypeFloat4x3 = 14,
        MTLDataTypeFloat4x4 = 15,
        MTLDataTypeHalf = 16,
        MTLDataTypeHalf2 = 17,
        MTLDataTypeHalf3 = 18,
        MTLDataTypeHalf4 = 19,
        MTLDataTypeHalf2x2 = 20,
        MTLDataTypeHalf2x3 = 21,
        MTLDataTypeHalf2x4 = 22,
        MTLDataTypeHalf3x2 = 23,
        MTLDataTypeHalf3x3 = 24,
        MTLDataTypeHalf3x4 = 25,
        MTLDataTypeHalf4x2 = 26,
        MTLDataTypeHalf4x3 = 27,
        MTLDataTypeHalf4x4 = 28,
        MTLDataTypeInt = 29,
        MTLDataTypeInt2 = 30,
        MTLDataTypeInt3 = 31,
        MTLDataTypeInt4 = 32,
        MTLDataTypeUInt = 33,
        MTLDataTypeUInt2 = 34,
        MTLDataTypeUInt3 = 35,
        MTLDataTypeUInt4 = 36,
        MTLDataTypeShort = 37,
        MTLDataTypeShort2 = 38,
        MTLDataTypeShort3 = 39,
        MTLDataTypeShort4 = 40,
        MTLDataTypeUShort = 41,
        MTLDataTypeUShort2 = 42,
        MTLDataTypeUShort3 = 43,
        MTLDataTypeUShort4 = 44,
        MTLDataTypeChar = 45,
        MTLDataTypeChar2 = 46,
        MTLDataTypeChar3 = 47,
        MTLDataTypeChar4 = 48,
        MTLDataTypeUChar = 49,
        MTLDataTypeUChar2 = 50,
        MTLDataTypeUChar3 = 51,
        MTLDataTypeUChar4 = 52,
        MTLDataTypeBool = 53,
        MTLDataTypeBool2 = 54,
        MTLDataTypeBool3 = 55,
        MTLDataTypeBool4 = 56,
        MTLDataTypeTexture = 58,
        MTLDataTypeSampler = 59,
        MTLDataTypePointer = 60,
        MTLDataTypeR8Unorm = 62,
        MTLDataTypeR8Snorm = 63,
        MTLDataTypeR16Unorm = 64,
        MTLDataTypeR16Snorm = 65,
        MTLDataTypeRG8Unorm = 66,
        MTLDataTypeRG8Snorm = 67,
        MTLDataTypeRG16Unorm = 68,
        MTLDataTypeRG16Snorm = 69,
        MTLDataTypeRGBA8Unorm = 70,
        MTLDataTypeRGBA8Unorm_sRGB = 71,
        MTLDataTypeRGBA8Snorm = 72,
        MTLDataTypeRGBA16Unorm = 73,
        MTLDataTypeRGBA16Snorm = 74,
        MTLDataTypeRGB10A2Unorm = 75,
        MTLDataTypeRG11B10Float = 76,
        MTLDataTypeRGB9E5Float = 77,
        MTLDataTypeRenderPipeline = 78,
        MTLDataTypeComputePipeline = 79,
        MTLDataTypeIndirectCommandBuffer = 80,
        MTLDataTypeLong = 81,
        MTLDataTypeLong2 = 82,
        MTLDataTypeLong3 = 83,
        MTLDataTypeLong4 = 84,
        MTLDataTypeULong = 85,
        MTLDataTypeULong2 = 86,
        MTLDataTypeULong3 = 87,
        MTLDataTypeULong4 = 88,
        MTLDataTypeVisibleFunctionTable = 115,
        MTLDataTypeIntersectionFunctionTable = 116,
        MTLDataTypePrimitiveAccelerationStructure = 117,
        MTLDataTypeInstanceAccelerationStructure = 118,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLBindingType {
        MTLBindingTypeBuffer = 0,
        MTLBindingTypeThreadgroupMemory = 1,
        MTLBindingTypeTexture = 2,
        MTLBindingTypeSampler = 3,
        MTLBindingTypeImageblockData = 16,
        MTLBindingTypeImageblock = 17,
        MTLBindingTypeVisibleFunctionTable = 24,
        MTLBindingTypePrimitiveAccelerationStructure = 25,
        MTLBindingTypeInstanceAccelerationStructure = 26,
        MTLBindingTypeIntersectionFunctionTable = 27,
        MTLBindingTypeObjectPayload = 34,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    #[deprecated]
    pub enum MTLArgumentType {
        #[deprecated]
        MTLArgumentTypeBuffer = 0,
        #[deprecated]
        MTLArgumentTypeThreadgroupMemory = 1,
        #[deprecated]
        MTLArgumentTypeTexture = 2,
        #[deprecated]
        MTLArgumentTypeSampler = 3,
        MTLArgumentTypeImageblockData = 16,
        MTLArgumentTypeImageblock = 17,
        MTLArgumentTypeVisibleFunctionTable = 24,
        MTLArgumentTypePrimitiveAccelerationStructure = 25,
        MTLArgumentTypeInstanceAccelerationStructure = 26,
        MTLArgumentTypeIntersectionFunctionTable = 27,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLArgumentAccess {
        MTLArgumentAccessReadOnly = 0,
        MTLArgumentAccessReadWrite = 1,
        MTLArgumentAccessWriteOnly = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLType")]
    pub struct MTLType;

    #[cfg(feature = "Metal_MTLType")]
    unsafe impl ClassType for MTLType {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLType")]
unsafe impl NSObjectProtocol for MTLType {}

extern_methods!(
    #[cfg(feature = "Metal_MTLType")]
    unsafe impl MTLType {
        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MTLDataType;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLStructMember")]
    pub struct MTLStructMember;

    #[cfg(feature = "Metal_MTLStructMember")]
    unsafe impl ClassType for MTLStructMember {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLStructMember")]
unsafe impl NSObjectProtocol for MTLStructMember {}

extern_methods!(
    #[cfg(feature = "Metal_MTLStructMember")]
    unsafe impl MTLStructMember {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Id<NSString>;

        #[method(offset)]
        pub fn offset(&self) -> NSUInteger;

        #[method(dataType)]
        pub fn dataType(&self) -> MTLDataType;

        #[cfg(feature = "Metal_MTLStructType")]
        #[method_id(@__retain_semantics Other structType)]
        pub fn structType(&self) -> Option<Id<MTLStructType>>;

        #[cfg(feature = "Metal_MTLArrayType")]
        #[method_id(@__retain_semantics Other arrayType)]
        pub fn arrayType(&self) -> Option<Id<MTLArrayType>>;

        #[cfg(feature = "Metal_MTLTextureReferenceType")]
        #[method_id(@__retain_semantics Other textureReferenceType)]
        pub unsafe fn textureReferenceType(&self) -> Option<Id<MTLTextureReferenceType>>;

        #[cfg(feature = "Metal_MTLPointerType")]
        #[method_id(@__retain_semantics Other pointerType)]
        pub unsafe fn pointerType(&self) -> Option<Id<MTLPointerType>>;

        #[method(argumentIndex)]
        pub unsafe fn argumentIndex(&self) -> NSUInteger;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLStructType")]
    pub struct MTLStructType;

    #[cfg(feature = "Metal_MTLStructType")]
    unsafe impl ClassType for MTLStructType {
        #[inherits(NSObject)]
        type Super = MTLType;
    }
);

#[cfg(feature = "Metal_MTLStructType")]
unsafe impl NSObjectProtocol for MTLStructType {}

extern_methods!(
    #[cfg(feature = "Metal_MTLStructType")]
    unsafe impl MTLStructType {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLStructMember"))]
        #[method_id(@__retain_semantics Other members)]
        pub fn members(&self) -> Id<NSArray<MTLStructMember>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Metal_MTLStructMember"))]
        #[method_id(@__retain_semantics Other memberByName:)]
        pub fn memberByName(&self, name: &NSString) -> Option<Id<MTLStructMember>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLArrayType")]
    pub struct MTLArrayType;

    #[cfg(feature = "Metal_MTLArrayType")]
    unsafe impl ClassType for MTLArrayType {
        #[inherits(NSObject)]
        type Super = MTLType;
    }
);

#[cfg(feature = "Metal_MTLArrayType")]
unsafe impl NSObjectProtocol for MTLArrayType {}

extern_methods!(
    #[cfg(feature = "Metal_MTLArrayType")]
    unsafe impl MTLArrayType {
        #[method(elementType)]
        pub fn elementType(&self) -> MTLDataType;

        #[method(arrayLength)]
        pub fn arrayLength(&self) -> NSUInteger;

        #[method(stride)]
        pub fn stride(&self) -> NSUInteger;

        #[method(argumentIndexStride)]
        pub unsafe fn argumentIndexStride(&self) -> NSUInteger;

        #[cfg(feature = "Metal_MTLStructType")]
        #[method_id(@__retain_semantics Other elementStructType)]
        pub fn elementStructType(&self) -> Option<Id<MTLStructType>>;

        #[method_id(@__retain_semantics Other elementArrayType)]
        pub fn elementArrayType(&self) -> Option<Id<MTLArrayType>>;

        #[cfg(feature = "Metal_MTLTextureReferenceType")]
        #[method_id(@__retain_semantics Other elementTextureReferenceType)]
        pub unsafe fn elementTextureReferenceType(&self) -> Option<Id<MTLTextureReferenceType>>;

        #[cfg(feature = "Metal_MTLPointerType")]
        #[method_id(@__retain_semantics Other elementPointerType)]
        pub unsafe fn elementPointerType(&self) -> Option<Id<MTLPointerType>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLPointerType")]
    pub struct MTLPointerType;

    #[cfg(feature = "Metal_MTLPointerType")]
    unsafe impl ClassType for MTLPointerType {
        #[inherits(NSObject)]
        type Super = MTLType;
    }
);

#[cfg(feature = "Metal_MTLPointerType")]
unsafe impl NSObjectProtocol for MTLPointerType {}

extern_methods!(
    #[cfg(feature = "Metal_MTLPointerType")]
    unsafe impl MTLPointerType {
        #[method(elementType)]
        pub unsafe fn elementType(&self) -> MTLDataType;

        #[method(access)]
        pub unsafe fn access(&self) -> MTLArgumentAccess;

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSUInteger;

        #[method(dataSize)]
        pub unsafe fn dataSize(&self) -> NSUInteger;

        #[method(elementIsArgumentBuffer)]
        pub unsafe fn elementIsArgumentBuffer(&self) -> bool;

        #[cfg(feature = "Metal_MTLStructType")]
        #[method_id(@__retain_semantics Other elementStructType)]
        pub unsafe fn elementStructType(&self) -> Option<Id<MTLStructType>>;

        #[cfg(feature = "Metal_MTLArrayType")]
        #[method_id(@__retain_semantics Other elementArrayType)]
        pub unsafe fn elementArrayType(&self) -> Option<Id<MTLArrayType>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLTextureReferenceType")]
    pub struct MTLTextureReferenceType;

    #[cfg(feature = "Metal_MTLTextureReferenceType")]
    unsafe impl ClassType for MTLTextureReferenceType {
        #[inherits(NSObject)]
        type Super = MTLType;
    }
);

#[cfg(feature = "Metal_MTLTextureReferenceType")]
unsafe impl NSObjectProtocol for MTLTextureReferenceType {}

extern_methods!(
    #[cfg(feature = "Metal_MTLTextureReferenceType")]
    unsafe impl MTLTextureReferenceType {
        #[method(textureDataType)]
        pub unsafe fn textureDataType(&self) -> MTLDataType;

        #[method(textureType)]
        pub unsafe fn textureType(&self) -> MTLTextureType;

        #[method(access)]
        pub unsafe fn access(&self) -> MTLArgumentAccess;

        #[method(isDepthTexture)]
        pub unsafe fn isDepthTexture(&self) -> bool;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLArgument")]
    #[deprecated]
    pub struct MTLArgument;

    #[cfg(feature = "Metal_MTLArgument")]
    unsafe impl ClassType for MTLArgument {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLArgument")]
unsafe impl NSObjectProtocol for MTLArgument {}

extern_methods!(
    #[cfg(feature = "Metal_MTLArgument")]
    unsafe impl MTLArgument {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Id<NSString>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> MTLArgumentType;

        #[method(access)]
        pub fn access(&self) -> MTLArgumentAccess;

        #[method(index)]
        pub fn index(&self) -> NSUInteger;

        #[method(isActive)]
        pub fn isActive(&self) -> bool;

        #[method(bufferAlignment)]
        pub fn bufferAlignment(&self) -> NSUInteger;

        #[method(bufferDataSize)]
        pub fn bufferDataSize(&self) -> NSUInteger;

        #[method(bufferDataType)]
        pub fn bufferDataType(&self) -> MTLDataType;

        #[cfg(feature = "Metal_MTLStructType")]
        #[method_id(@__retain_semantics Other bufferStructType)]
        pub fn bufferStructType(&self) -> Option<Id<MTLStructType>>;

        #[cfg(feature = "Metal_MTLPointerType")]
        #[method_id(@__retain_semantics Other bufferPointerType)]
        pub unsafe fn bufferPointerType(&self) -> Option<Id<MTLPointerType>>;

        #[method(threadgroupMemoryAlignment)]
        pub fn threadgroupMemoryAlignment(&self) -> NSUInteger;

        #[method(threadgroupMemoryDataSize)]
        pub fn threadgroupMemoryDataSize(&self) -> NSUInteger;

        #[method(textureType)]
        pub fn textureType(&self) -> MTLTextureType;

        #[method(textureDataType)]
        pub fn textureDataType(&self) -> MTLDataType;

        #[method(isDepthTexture)]
        pub unsafe fn isDepthTexture(&self) -> bool;

        #[method(arrayLength)]
        pub unsafe fn arrayLength(&self) -> NSUInteger;
    }
);

extern_protocol!(
    pub unsafe trait MTLBinding: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(&self) -> Id<NSString>;

        #[method(type)]
        unsafe fn r#type(&self) -> MTLBindingType;

        #[method(access)]
        unsafe fn access(&self) -> MTLArgumentAccess;

        #[method(index)]
        unsafe fn index(&self) -> NSUInteger;

        #[method(isUsed)]
        unsafe fn isUsed(&self) -> bool;

        #[method(isArgument)]
        unsafe fn isArgument(&self) -> bool;
    }

    unsafe impl ProtocolType for dyn MTLBinding {}
);

extern_protocol!(
    pub unsafe trait MTLBufferBinding: MTLBinding {
        #[method(bufferAlignment)]
        unsafe fn bufferAlignment(&self) -> NSUInteger;

        #[method(bufferDataSize)]
        unsafe fn bufferDataSize(&self) -> NSUInteger;

        #[method(bufferDataType)]
        unsafe fn bufferDataType(&self) -> MTLDataType;

        #[cfg(feature = "Metal_MTLStructType")]
        #[method_id(@__retain_semantics Other bufferStructType)]
        unsafe fn bufferStructType(&self) -> Option<Id<MTLStructType>>;

        #[cfg(feature = "Metal_MTLPointerType")]
        #[method_id(@__retain_semantics Other bufferPointerType)]
        unsafe fn bufferPointerType(&self) -> Option<Id<MTLPointerType>>;
    }

    unsafe impl ProtocolType for dyn MTLBufferBinding {}
);

extern_protocol!(
    pub unsafe trait MTLThreadgroupBinding: MTLBinding {
        #[method(threadgroupMemoryAlignment)]
        unsafe fn threadgroupMemoryAlignment(&self) -> NSUInteger;

        #[method(threadgroupMemoryDataSize)]
        unsafe fn threadgroupMemoryDataSize(&self) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn MTLThreadgroupBinding {}
);

extern_protocol!(
    pub unsafe trait MTLTextureBinding: MTLBinding {
        #[method(textureType)]
        unsafe fn textureType(&self) -> MTLTextureType;

        #[method(textureDataType)]
        unsafe fn textureDataType(&self) -> MTLDataType;

        #[method(isDepthTexture)]
        unsafe fn isDepthTexture(&self) -> bool;

        #[method(arrayLength)]
        unsafe fn arrayLength(&self) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn MTLTextureBinding {}
);

extern_protocol!(
    pub unsafe trait MTLObjectPayloadBinding: MTLBinding {
        #[method(objectPayloadAlignment)]
        unsafe fn objectPayloadAlignment(&self) -> NSUInteger;

        #[method(objectPayloadDataSize)]
        unsafe fn objectPayloadDataSize(&self) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn MTLObjectPayloadBinding {}
);
