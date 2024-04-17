//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type SKAdNetworkCoarseConversionValue = NSString;

extern "C" {
    pub static SKAdNetworkCoarseConversionValueHigh: &'static SKAdNetworkCoarseConversionValue;
}

extern "C" {
    pub static SKAdNetworkCoarseConversionValueMedium: &'static SKAdNetworkCoarseConversionValue;
}

extern "C" {
    pub static SKAdNetworkCoarseConversionValueLow: &'static SKAdNetworkCoarseConversionValue;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKAdNetwork;

    unsafe impl ClassType for SKAdNetwork {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for SKAdNetwork {}

extern_methods!(
    unsafe impl SKAdNetwork {
        #[cfg(all(feature = "SKAdImpression", feature = "block2"))]
        #[method(startImpression:completionHandler:)]
        pub unsafe fn startImpression_completionHandler(
            impression: &SKAdImpression,
            completion: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(all(feature = "SKAdImpression", feature = "block2"))]
        #[method(endImpression:completionHandler:)]
        pub unsafe fn endImpression_completionHandler(
            impression: &SKAdImpression,
            completion: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[deprecated = "Use updatePostbackConversionValue:completionHandler: instead"]
        #[method(registerAppForAdNetworkAttribution)]
        pub unsafe fn registerAppForAdNetworkAttribution();

        #[deprecated = "Use updatePostbackConversionValue:completionHandler: instead"]
        #[method(updateConversionValue:)]
        pub unsafe fn updateConversionValue(conversion_value: NSInteger);

        #[cfg(feature = "block2")]
        #[method(updatePostbackConversionValue:completionHandler:)]
        pub unsafe fn updatePostbackConversionValue_completionHandler(
            conversion_value: NSInteger,
            completion: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(updatePostbackConversionValue:coarseValue:completionHandler:)]
        pub unsafe fn updatePostbackConversionValue_coarseValue_completionHandler(
            fine_value: NSInteger,
            coarse_value: &SKAdNetworkCoarseConversionValue,
            completion: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(updatePostbackConversionValue:coarseValue:lockWindow:completionHandler:)]
        pub unsafe fn updatePostbackConversionValue_coarseValue_lockWindow_completionHandler(
            fine_value: NSInteger,
            coarse_value: &SKAdNetworkCoarseConversionValue,
            lock_window: bool,
            completion: Option<&Block<dyn Fn(*mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKAdNetwork {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    pub static SKStoreProductParameterAdNetworkAttributionSignature: &'static NSString;
}

extern "C" {
    pub static SKStoreProductParameterAdNetworkCampaignIdentifier: &'static NSString;
}

extern "C" {
    pub static SKStoreProductParameterAdNetworkSourceIdentifier: &'static NSString;
}

extern "C" {
    pub static SKStoreProductParameterAdNetworkIdentifier: &'static NSString;
}

extern "C" {
    pub static SKStoreProductParameterAdNetworkNonce: &'static NSString;
}

extern "C" {
    pub static SKStoreProductParameterAdNetworkTimestamp: &'static NSString;
}

extern "C" {
    pub static SKStoreProductParameterAdNetworkSourceAppStoreIdentifier: &'static NSString;
}

extern "C" {
    pub static SKStoreProductParameterAdNetworkVersion: &'static NSString;
}
