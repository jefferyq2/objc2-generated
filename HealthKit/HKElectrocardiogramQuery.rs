//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKElectrocardiogramVoltageMeasurement;

    unsafe impl ClassType for HKElectrocardiogramVoltageMeasurement {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for HKElectrocardiogramVoltageMeasurement {}

unsafe impl NSObjectProtocol for HKElectrocardiogramVoltageMeasurement {}

extern_methods!(
    unsafe impl HKElectrocardiogramVoltageMeasurement {
        #[method(timeSinceSampleStart)]
        pub unsafe fn timeSinceSampleStart(&self) -> NSTimeInterval;

        #[cfg(all(feature = "HKElectrocardiogram", feature = "HKQuantity"))]
        #[method_id(@__retain_semantics Other quantityForLead:)]
        pub unsafe fn quantityForLead(
            &self,
            lead: HKElectrocardiogramLead,
        ) -> Option<Id<HKQuantity>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKElectrocardiogramVoltageMeasurement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKElectrocardiogramQuery;

    #[cfg(feature = "HKQuery")]
    unsafe impl ClassType for HKElectrocardiogramQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKElectrocardiogramQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKElectrocardiogramQuery {
        #[cfg(all(
            feature = "HKElectrocardiogram",
            feature = "HKObject",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithElectrocardiogram:dataHandler:)]
        pub unsafe fn initWithElectrocardiogram_dataHandler(
            this: Allocated<Self>,
            electrocardiogram: &HKElectrocardiogram,
            data_handler: &Block<
                dyn Fn(
                    NonNull<HKElectrocardiogramQuery>,
                    *mut HKElectrocardiogramVoltageMeasurement,
                    Bool,
                    *mut NSError,
                ),
            >,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKElectrocardiogramQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKElectrocardiogramQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
