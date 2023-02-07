//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_protocol!(
    pub unsafe trait GCDevicePhysicalInputState: NSObjectProtocol {
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Option<Id<ProtocolObject<dyn GCDevice>>>;

        #[method(lastEventTimestamp)]
        unsafe fn lastEventTimestamp(&self) -> NSTimeInterval;

        #[method(lastEventLatency)]
        unsafe fn lastEventLatency(&self) -> NSTimeInterval;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameController_GCPhysicalInputElementCollection"
        ))]
        #[method_id(@__retain_semantics Other elements)]
        unsafe fn elements(
            &self,
        ) -> Id<
            GCPhysicalInputElementCollection<NSString, ProtocolObject<dyn GCPhysicalInputElement>>,
        >;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameController_GCPhysicalInputElementCollection"
        ))]
        #[method_id(@__retain_semantics Other buttons)]
        unsafe fn buttons(
            &self,
        ) -> Id<GCPhysicalInputElementCollection<NSString, ProtocolObject<dyn GCButtonElement>>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameController_GCPhysicalInputElementCollection"
        ))]
        #[method_id(@__retain_semantics Other axes)]
        unsafe fn axes(
            &self,
        ) -> Id<GCPhysicalInputElementCollection<NSString, ProtocolObject<dyn GCAxisElement>>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameController_GCPhysicalInputElementCollection"
        ))]
        #[method_id(@__retain_semantics Other switches)]
        unsafe fn switches(
            &self,
        ) -> Id<GCPhysicalInputElementCollection<NSString, ProtocolObject<dyn GCSwitchElement>>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameController_GCPhysicalInputElementCollection"
        ))]
        #[method_id(@__retain_semantics Other dpads)]
        unsafe fn dpads(
            &self,
        ) -> Id<GCPhysicalInputElementCollection<NSString, ProtocolObject<dyn GCDirectionPadElement>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        unsafe fn objectForKeyedSubscript(
            &self,
            key: &NSString,
        ) -> Option<Id<ProtocolObject<dyn GCPhysicalInputElement>>>;
    }

    unsafe impl ProtocolType for dyn GCDevicePhysicalInputState {}
);
