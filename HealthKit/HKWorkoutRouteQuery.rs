//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKWorkoutRouteQuery")]
    pub struct HKWorkoutRouteQuery;

    #[cfg(feature = "HealthKit_HKWorkoutRouteQuery")]
    unsafe impl ClassType for HKWorkoutRouteQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKWorkoutRouteQuery")]
unsafe impl NSObjectProtocol for HKWorkoutRouteQuery {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKWorkoutRouteQuery")]
    unsafe impl HKWorkoutRouteQuery {
        #[cfg(all(
            feature = "CoreLocation_CLLocation",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKWorkoutRoute"
        ))]
        #[method_id(@__retain_semantics Init initWithRoute:dataHandler:)]
        pub unsafe fn initWithRoute_dataHandler(
            this: Allocated<Self>,
            workout_route: &HKWorkoutRoute,
            data_handler: &Block<
                dyn Fn(NonNull<HKWorkoutRouteQuery>, *mut NSArray<CLLocation>, Bool, *mut NSError),
            >,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CoreLocation_CLLocation",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDateInterval",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKWorkoutRoute"
        ))]
        #[method_id(@__retain_semantics Init initWithRoute:dateInterval:dataHandler:)]
        pub unsafe fn initWithRoute_dateInterval_dataHandler(
            this: Allocated<Self>,
            workout_route: &HKWorkoutRoute,
            date_interval: &NSDateInterval,
            data_handler: &Block<
                dyn Fn(NonNull<HKWorkoutRouteQuery>, *mut NSArray<CLLocation>, Bool, *mut NSError),
            >,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HealthKit_HKWorkoutRouteQuery")]
    unsafe impl HKWorkoutRouteQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKWorkoutRouteQuery")]
    unsafe impl HKWorkoutRouteQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
