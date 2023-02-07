//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ExceptionHandling::*;
use crate::Foundation::*;

extern_static!(NSUncaughtSystemExceptionException: Option<&'static NSString>);

extern_static!(NSUncaughtRuntimeErrorException: Option<&'static NSString>);

extern_static!(NSStackTraceKey: Option<&'static NSString>);

extern_fn!(
    pub unsafe fn NSExceptionHandlerResume();
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        NSLogUncaughtExceptionMask = 1 << 0,
        NSHandleUncaughtExceptionMask = 1 << 1,
        NSLogUncaughtSystemExceptionMask = 1 << 2,
        NSHandleUncaughtSystemExceptionMask = 1 << 3,
        NSLogUncaughtRuntimeErrorMask = 1 << 4,
        NSHandleUncaughtRuntimeErrorMask = 1 << 5,
        NSLogTopLevelExceptionMask = 1 << 6,
        NSHandleTopLevelExceptionMask = 1 << 7,
        NSLogOtherExceptionMask = 1 << 8,
        NSHandleOtherExceptionMask = 1 << 9,
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        NSHangOnUncaughtExceptionMask = 1 << 0,
        NSHangOnUncaughtSystemExceptionMask = 1 << 1,
        NSHangOnUncaughtRuntimeErrorMask = 1 << 2,
        NSHangOnTopLevelExceptionMask = 1 << 3,
        NSHangOnOtherExceptionMask = 1 << 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ExceptionHandling_NSExceptionHandler")]
    pub struct NSExceptionHandler;

    #[cfg(feature = "ExceptionHandling_NSExceptionHandler")]
    unsafe impl ClassType for NSExceptionHandler {
        type Super = NSObject;
    }
);

#[cfg(feature = "ExceptionHandling_NSExceptionHandler")]
unsafe impl NSObjectProtocol for NSExceptionHandler {}

extern_methods!(
    #[cfg(feature = "ExceptionHandling_NSExceptionHandler")]
    unsafe impl NSExceptionHandler {
        #[method_id(@__retain_semantics Other defaultExceptionHandler)]
        pub unsafe fn defaultExceptionHandler() -> Option<Id<NSExceptionHandler>>;

        #[method(setExceptionHandlingMask:)]
        pub unsafe fn setExceptionHandlingMask(&self, a_mask: NSUInteger);

        #[method(exceptionHandlingMask)]
        pub unsafe fn exceptionHandlingMask(&self) -> NSUInteger;

        #[method(setExceptionHangingMask:)]
        pub unsafe fn setExceptionHangingMask(&self, a_mask: NSUInteger);

        #[method(exceptionHangingMask)]
        pub unsafe fn exceptionHangingMask(&self) -> NSUInteger;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, an_object: Option<&Object>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<Object>>;
    }
);
