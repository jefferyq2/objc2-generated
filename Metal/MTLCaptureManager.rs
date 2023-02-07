//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_static!(MTLCaptureErrorDomain: &'static NSErrorDomain);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLCaptureError {
        MTLCaptureErrorNotSupported = 1,
        MTLCaptureErrorAlreadyCapturing = 2,
        MTLCaptureErrorInvalidDescriptor = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLCaptureDestination {
        MTLCaptureDestinationDeveloperTools = 1,
        MTLCaptureDestinationGPUTraceDocument = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLCaptureDescriptor")]
    pub struct MTLCaptureDescriptor;

    #[cfg(feature = "Metal_MTLCaptureDescriptor")]
    unsafe impl ClassType for MTLCaptureDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLCaptureDescriptor")]
unsafe impl NSObjectProtocol for MTLCaptureDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLCaptureDescriptor")]
    unsafe impl MTLCaptureDescriptor {
        #[method_id(@__retain_semantics Other captureObject)]
        pub unsafe fn captureObject(&self) -> Option<Id<Object>>;

        #[method(setCaptureObject:)]
        pub unsafe fn setCaptureObject(&self, capture_object: Option<&Object>);

        #[method(destination)]
        pub fn destination(&self) -> MTLCaptureDestination;

        #[method(setDestination:)]
        pub fn setDestination(&self, destination: MTLCaptureDestination);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other outputURL)]
        pub fn outputURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setOutputURL:)]
        pub fn setOutputURL(&self, output_url: Option<&NSURL>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLCaptureManager")]
    pub struct MTLCaptureManager;

    #[cfg(feature = "Metal_MTLCaptureManager")]
    unsafe impl ClassType for MTLCaptureManager {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLCaptureManager")]
unsafe impl NSObjectProtocol for MTLCaptureManager {}

extern_methods!(
    #[cfg(feature = "Metal_MTLCaptureManager")]
    unsafe impl MTLCaptureManager {
        #[method_id(@__retain_semantics Other sharedCaptureManager)]
        pub unsafe fn sharedCaptureManager() -> Id<MTLCaptureManager>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New newCaptureScopeWithDevice:)]
        pub fn newCaptureScopeWithDevice(
            &self,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Id<ProtocolObject<dyn MTLCaptureScope>>;

        #[method_id(@__retain_semantics New newCaptureScopeWithCommandQueue:)]
        pub fn newCaptureScopeWithCommandQueue(
            &self,
            command_queue: &ProtocolObject<dyn MTLCommandQueue>,
        ) -> Id<ProtocolObject<dyn MTLCaptureScope>>;

        #[method(supportsDestination:)]
        pub fn supportsDestination(&self, destination: MTLCaptureDestination) -> bool;

        #[cfg(all(feature = "Foundation_NSError", feature = "Metal_MTLCaptureDescriptor"))]
        #[method(startCaptureWithDescriptor:error:_)]
        pub fn startCaptureWithDescriptor_error(
            &self,
            descriptor: &MTLCaptureDescriptor,
        ) -> Result<(), Id<NSError>>;

        #[deprecated = "Use startCaptureWithDescriptor:error: instead"]
        #[method(startCaptureWithDevice:)]
        pub fn startCaptureWithDevice(&self, device: &ProtocolObject<dyn MTLDevice>);

        #[deprecated = "Use startCaptureWithDescriptor:error: instead"]
        #[method(startCaptureWithCommandQueue:)]
        pub fn startCaptureWithCommandQueue(
            &self,
            command_queue: &ProtocolObject<dyn MTLCommandQueue>,
        );

        #[deprecated = "Use startCaptureWithDescriptor:error: instead"]
        #[method(startCaptureWithScope:)]
        pub fn startCaptureWithScope(&self, capture_scope: &ProtocolObject<dyn MTLCaptureScope>);

        #[method(stopCapture)]
        pub fn stopCapture(&self);

        #[method_id(@__retain_semantics Other defaultCaptureScope)]
        pub fn defaultCaptureScope(&self) -> Option<Id<ProtocolObject<dyn MTLCaptureScope>>>;

        #[method(setDefaultCaptureScope:)]
        pub fn setDefaultCaptureScope(
            &self,
            default_capture_scope: Option<&ProtocolObject<dyn MTLCaptureScope>>,
        );

        #[method(isCapturing)]
        pub fn isCapturing(&self) -> bool;
    }
);
