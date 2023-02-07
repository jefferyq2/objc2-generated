//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

typed_enum!(
    pub type MTLCommonCounter = NSString;
);

extern_static!(MTLCommonCounterTimestamp: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterTessellationInputPatches: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterVertexInvocations: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterPostTessellationVertexInvocations: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterClipperInvocations: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterClipperPrimitivesOut: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterFragmentInvocations: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterFragmentsPassed: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterComputeKernelInvocations: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterTotalCycles: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterVertexCycles: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterTessellationCycles: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterPostTessellationVertexCycles: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterFragmentCycles: &'static MTLCommonCounter);

extern_static!(MTLCommonCounterRenderTargetWriteCycles: &'static MTLCommonCounter);

typed_enum!(
    pub type MTLCommonCounterSet = NSString;
);

extern_static!(MTLCommonCounterSetTimestamp: &'static MTLCommonCounterSet);

extern_static!(MTLCommonCounterSetStageUtilization: &'static MTLCommonCounterSet);

extern_static!(MTLCommonCounterSetStatistic: &'static MTLCommonCounterSet);

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLCounterResultTimestamp {
        pub timestamp: u64,
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLCounterResultStageUtilization {
        pub totalCycles: u64,
        pub vertexCycles: u64,
        pub tessellationCycles: u64,
        pub postTessellationVertexCycles: u64,
        pub fragmentCycles: u64,
        pub renderTargetCycles: u64,
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLCounterResultStatistic {
        pub tessellationInputPatches: u64,
        pub vertexInvocations: u64,
        pub postTessellationVertexInvocations: u64,
        pub clipperInvocations: u64,
        pub clipperPrimitivesOut: u64,
        pub fragmentInvocations: u64,
        pub fragmentsPassed: u64,
        pub computeKernelInvocations: u64,
    }
);

extern_protocol!(
    pub unsafe trait MTLCounter: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(&self) -> Id<NSString>;
    }

    unsafe impl ProtocolType for dyn MTLCounter {}
);

extern_protocol!(
    pub unsafe trait MTLCounterSet: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other counters)]
        unsafe fn counters(&self) -> Id<NSArray<ProtocolObject<dyn MTLCounter>>>;
    }

    unsafe impl ProtocolType for dyn MTLCounterSet {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLCounterSampleBufferDescriptor")]
    pub struct MTLCounterSampleBufferDescriptor;

    #[cfg(feature = "Metal_MTLCounterSampleBufferDescriptor")]
    unsafe impl ClassType for MTLCounterSampleBufferDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLCounterSampleBufferDescriptor")]
unsafe impl NSObjectProtocol for MTLCounterSampleBufferDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLCounterSampleBufferDescriptor")]
    unsafe impl MTLCounterSampleBufferDescriptor {
        #[method_id(@__retain_semantics Other counterSet)]
        pub unsafe fn counterSet(&self) -> Option<Id<ProtocolObject<dyn MTLCounterSet>>>;

        #[method(setCounterSet:)]
        pub unsafe fn setCounterSet(&self, counter_set: Option<&ProtocolObject<dyn MTLCounterSet>>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);

        #[method(storageMode)]
        pub unsafe fn storageMode(&self) -> MTLStorageMode;

        #[method(setStorageMode:)]
        pub unsafe fn setStorageMode(&self, storage_mode: MTLStorageMode);

        #[method(sampleCount)]
        pub unsafe fn sampleCount(&self) -> NSUInteger;

        #[method(setSampleCount:)]
        pub unsafe fn setSampleCount(&self, sample_count: NSUInteger);
    }
);

extern_protocol!(
    pub unsafe trait MTLCounterSampleBuffer: NSObjectProtocol {
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Id<NSString>;

        #[method(sampleCount)]
        unsafe fn sampleCount(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other resolveCounterRange:)]
        unsafe fn resolveCounterRange(&self, range: NSRange) -> Option<Id<NSData>>;
    }

    unsafe impl ProtocolType for dyn MTLCounterSampleBuffer {}
);

extern_static!(MTLCounterErrorDomain: &'static NSErrorDomain);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLCounterSampleBufferError {
        MTLCounterSampleBufferErrorOutOfMemory = 0,
        MTLCounterSampleBufferErrorInvalid = 1,
        MTLCounterSampleBufferErrorInternal = 2,
    }
);
