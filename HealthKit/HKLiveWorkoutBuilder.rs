//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait HKLiveWorkoutBuilderDelegate: NSObjectProtocol {
        #[cfg(all(feature = "HKObjectType", feature = "HKWorkoutBuilder"))]
        #[method(workoutBuilder:didCollectDataOfTypes:)]
        unsafe fn workoutBuilder_didCollectDataOfTypes(
            &self,
            workout_builder: &HKLiveWorkoutBuilder,
            collected_types: &NSSet<HKSampleType>,
        );

        #[cfg(feature = "HKWorkoutBuilder")]
        #[method(workoutBuilderDidCollectEvent:)]
        unsafe fn workoutBuilderDidCollectEvent(&self, workout_builder: &HKLiveWorkoutBuilder);

        #[cfg(all(feature = "HKWorkoutActivity", feature = "HKWorkoutBuilder"))]
        #[optional]
        #[method(workoutBuilder:didBeginActivity:)]
        unsafe fn workoutBuilder_didBeginActivity(
            &self,
            workout_builder: &HKLiveWorkoutBuilder,
            workout_activity: &HKWorkoutActivity,
        );

        #[cfg(all(feature = "HKWorkoutActivity", feature = "HKWorkoutBuilder"))]
        #[optional]
        #[method(workoutBuilder:didEndActivity:)]
        unsafe fn workoutBuilder_didEndActivity(
            &self,
            workout_builder: &HKLiveWorkoutBuilder,
            workout_activity: &HKWorkoutActivity,
        );
    }

    unsafe impl ProtocolType for dyn HKLiveWorkoutBuilderDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKWorkoutBuilder")]
    pub struct HKLiveWorkoutBuilder;

    #[cfg(feature = "HKWorkoutBuilder")]
    unsafe impl ClassType for HKLiveWorkoutBuilder {
        #[inherits(NSObject)]
        type Super = HKWorkoutBuilder;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HKWorkoutBuilder")]
unsafe impl NSObjectProtocol for HKLiveWorkoutBuilder {}

extern_methods!(
    #[cfg(feature = "HKWorkoutBuilder")]
    unsafe impl HKLiveWorkoutBuilder {
        #[cfg(all(
            feature = "HKDevice",
            feature = "HKHealthStore",
            feature = "HKWorkoutConfiguration"
        ))]
        #[method_id(@__retain_semantics Init initWithHealthStore:configuration:device:)]
        pub unsafe fn initWithHealthStore_configuration_device(
            this: Allocated<Self>,
            health_store: &HKHealthStore,
            configuration: &HKWorkoutConfiguration,
            device: Option<&HKDevice>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn HKLiveWorkoutBuilderDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn HKLiveWorkoutBuilderDelegate>>,
        );

        #[cfg(feature = "HKWorkoutSession")]
        #[method_id(@__retain_semantics Other workoutSession)]
        pub unsafe fn workoutSession(&self) -> Option<Id<HKWorkoutSession>>;

        #[method(shouldCollectWorkoutEvents)]
        pub unsafe fn shouldCollectWorkoutEvents(&self) -> bool;

        #[method(setShouldCollectWorkoutEvents:)]
        pub unsafe fn setShouldCollectWorkoutEvents(&self, should_collect_workout_events: bool);

        #[cfg(feature = "HKLiveWorkoutDataSource")]
        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<HKLiveWorkoutDataSource>>;

        #[cfg(feature = "HKLiveWorkoutDataSource")]
        #[method(setDataSource:)]
        pub unsafe fn setDataSource(&self, data_source: Option<&HKLiveWorkoutDataSource>);

        #[method(elapsedTime)]
        pub unsafe fn elapsedTime(&self) -> NSTimeInterval;

        #[cfg(feature = "HKWorkoutActivity")]
        #[method_id(@__retain_semantics Other currentWorkoutActivity)]
        pub unsafe fn currentWorkoutActivity(&self) -> Option<Id<HKWorkoutActivity>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKWorkoutBuilder`
    #[cfg(feature = "HKWorkoutBuilder")]
    unsafe impl HKLiveWorkoutBuilder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKWorkoutBuilder")]
    unsafe impl HKLiveWorkoutBuilder {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
