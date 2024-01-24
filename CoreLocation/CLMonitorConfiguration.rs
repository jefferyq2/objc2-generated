//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLMonitorConfiguration")]
    pub struct CLMonitorConfiguration;

    #[cfg(feature = "CoreLocation_CLMonitorConfiguration")]
    unsafe impl ClassType for CLMonitorConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreLocation_CLMonitorConfiguration")]
unsafe impl Send for CLMonitorConfiguration {}

#[cfg(feature = "CoreLocation_CLMonitorConfiguration")]
unsafe impl Sync for CLMonitorConfiguration {}

#[cfg(feature = "CoreLocation_CLMonitorConfiguration")]
unsafe impl NSObjectProtocol for CLMonitorConfiguration {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLMonitorConfiguration")]
    unsafe impl CLMonitorConfiguration {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "CoreLocation_CLMonitor",
            feature = "CoreLocation_CLMonitoringEvent"
        ))]
        #[method(eventHandler)]
        pub unsafe fn eventHandler(
            &self,
        ) -> NonNull<Block<dyn Fn(NonNull<CLMonitor>, NonNull<CLMonitoringEvent>)>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreLocation_CLMonitorConfiguration")]
    unsafe impl CLMonitorConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
