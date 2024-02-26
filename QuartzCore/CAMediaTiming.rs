//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::QuartzCore::*;

#[cfg(feature = "Foundation_NSString")]
typed_enum!(
    pub type CAMediaTimingFillMode = NSString;
);

extern_protocol!(
    pub unsafe trait CAMediaTiming {
        #[method(beginTime)]
        unsafe fn beginTime(&self) -> CFTimeInterval;

        #[method(setBeginTime:)]
        unsafe fn setBeginTime(&self, begin_time: CFTimeInterval);

        #[method(duration)]
        unsafe fn duration(&self) -> CFTimeInterval;

        #[method(setDuration:)]
        unsafe fn setDuration(&self, duration: CFTimeInterval);

        #[method(speed)]
        unsafe fn speed(&self) -> c_float;

        #[method(setSpeed:)]
        unsafe fn setSpeed(&self, speed: c_float);

        #[method(timeOffset)]
        unsafe fn timeOffset(&self) -> CFTimeInterval;

        #[method(setTimeOffset:)]
        unsafe fn setTimeOffset(&self, time_offset: CFTimeInterval);

        #[method(repeatCount)]
        unsafe fn repeatCount(&self) -> c_float;

        #[method(setRepeatCount:)]
        unsafe fn setRepeatCount(&self, repeat_count: c_float);

        #[method(repeatDuration)]
        unsafe fn repeatDuration(&self) -> CFTimeInterval;

        #[method(setRepeatDuration:)]
        unsafe fn setRepeatDuration(&self, repeat_duration: CFTimeInterval);

        #[method(autoreverses)]
        unsafe fn autoreverses(&self) -> bool;

        #[method(setAutoreverses:)]
        unsafe fn setAutoreverses(&self, autoreverses: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fillMode)]
        unsafe fn fillMode(&self) -> Id<CAMediaTimingFillMode>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFillMode:)]
        unsafe fn setFillMode(&self, fill_mode: &CAMediaTimingFillMode);
    }

    unsafe impl ProtocolType for dyn CAMediaTiming {}
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(kCAFillModeForwards: &'static CAMediaTimingFillMode);

#[cfg(feature = "Foundation_NSString")]
extern_static!(kCAFillModeBackwards: &'static CAMediaTimingFillMode);

#[cfg(feature = "Foundation_NSString")]
extern_static!(kCAFillModeBoth: &'static CAMediaTimingFillMode);

#[cfg(feature = "Foundation_NSString")]
extern_static!(kCAFillModeRemoved: &'static CAMediaTimingFillMode);
