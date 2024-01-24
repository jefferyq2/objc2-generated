//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKArcadeService")]
    pub struct SKArcadeService;

    #[cfg(feature = "StoreKit_SKArcadeService")]
    unsafe impl ClassType for SKArcadeService {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKArcadeService")]
unsafe impl NSObjectProtocol for SKArcadeService {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKArcadeService")]
    unsafe impl SKArcadeService {
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(registerArcadeAppWithRandomFromLib:randomFromLibLength:resultHandler:)]
        pub unsafe fn registerArcadeAppWithRandomFromLib_randomFromLibLength_resultHandler(
            random_from_lib: &NSData,
            random_from_lib_length: u32,
            result_handler: &Block<dyn Fn(*mut NSData, u32, *mut NSData, u32, *mut NSError)>,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(arcadeSubscriptionStatusWithNonce:resultHandler:)]
        pub unsafe fn arcadeSubscriptionStatusWithNonce_resultHandler(
            nonce: u64,
            result_handler: &Block<dyn Fn(*mut NSData, u32, *mut NSData, u32, *mut NSError)>,
        );

        #[method(repairArcadeApp)]
        pub unsafe fn repairArcadeApp();
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "StoreKit_SKArcadeService")]
    unsafe impl SKArcadeService {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
