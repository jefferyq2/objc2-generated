//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLMonitor")]
    pub struct CLMonitor;

    #[cfg(feature = "CoreLocation_CLMonitor")]
    unsafe impl ClassType for CLMonitor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreLocation_CLMonitor")]
unsafe impl Send for CLMonitor {}

#[cfg(feature = "CoreLocation_CLMonitor")]
unsafe impl Sync for CLMonitor {}

#[cfg(feature = "CoreLocation_CLMonitor")]
unsafe impl NSObjectProtocol for CLMonitor {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLMonitor")]
    unsafe impl CLMonitor {
        #[cfg(feature = "CoreLocation_CLMonitorConfiguration")]
        #[method(requestMonitorWithConfiguration:completion:)]
        pub unsafe fn requestMonitorWithConfiguration_completion(
            config: &CLMonitorConfiguration,
            completion_handler: &Block<dyn Fn(NonNull<CLMonitor>)>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other monitoredIdentifiers)]
        pub unsafe fn monitoredIdentifiers(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "CoreLocation_CLCondition", feature = "Foundation_NSString"))]
        #[method(addConditionForMonitoring:identifier:)]
        pub unsafe fn addConditionForMonitoring_identifier(
            &self,
            condition: &CLCondition,
            identifier: &NSString,
        );

        #[cfg(all(feature = "CoreLocation_CLCondition", feature = "Foundation_NSString"))]
        #[method(addConditionForMonitoring:identifier:assumedState:)]
        pub unsafe fn addConditionForMonitoring_identifier_assumedState(
            &self,
            condition: &CLCondition,
            identifier: &NSString,
            state: CLMonitoringState,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeConditionFromMonitoringWithIdentifier:)]
        pub unsafe fn removeConditionFromMonitoringWithIdentifier(&self, identifier: &NSString);

        #[cfg(all(
            feature = "CoreLocation_CLMonitoringRecord",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other monitoringRecordForIdentifier:)]
        pub unsafe fn monitoringRecordForIdentifier(
            &self,
            identifier: &NSString,
        ) -> Option<Id<CLMonitoringRecord>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
