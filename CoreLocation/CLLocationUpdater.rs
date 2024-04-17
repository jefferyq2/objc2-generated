//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CLLiveUpdateConfiguration(pub NSInteger);
impl CLLiveUpdateConfiguration {
    #[doc(alias = "CLLiveUpdateConfigurationDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "CLLiveUpdateConfigurationAutomotiveNavigation")]
    pub const AutomotiveNavigation: Self = Self(1);
    #[doc(alias = "CLLiveUpdateConfigurationOtherNavigation")]
    pub const OtherNavigation: Self = Self(2);
    #[doc(alias = "CLLiveUpdateConfigurationFitness")]
    pub const Fitness: Self = Self(3);
    #[doc(alias = "CLLiveUpdateConfigurationAirborne")]
    pub const Airborne: Self = Self(4);
}

unsafe impl Encode for CLLiveUpdateConfiguration {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CLLiveUpdateConfiguration {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLUpdate;

    unsafe impl ClassType for CLUpdate {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CLUpdate {}

extern_methods!(
    unsafe impl CLUpdate {
        #[method(isStationary)]
        pub unsafe fn isStationary(&self) -> bool;

        #[cfg(feature = "CLLocation")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Id<CLLocation>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CLUpdate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLLocationUpdater;

    unsafe impl ClassType for CLLocationUpdater {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CLLocationUpdater {}

extern_methods!(
    unsafe impl CLLocationUpdater {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(resume)]
        pub unsafe fn resume(&self);

        #[method(pause)]
        pub unsafe fn pause(&self);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
    }
);
