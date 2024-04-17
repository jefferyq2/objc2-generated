//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "SKRequest")]
    pub struct SKReceiptRefreshRequest;

    #[cfg(feature = "SKRequest")]
    unsafe impl ClassType for SKReceiptRefreshRequest {
        #[inherits(NSObject)]
        type Super = SKRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "SKRequest")]
unsafe impl NSObjectProtocol for SKReceiptRefreshRequest {}

extern_methods!(
    #[cfg(feature = "SKRequest")]
    unsafe impl SKReceiptRefreshRequest {
        #[method_id(@__retain_semantics Init initWithReceiptProperties:)]
        pub unsafe fn initWithReceiptProperties(
            this: Allocated<Self>,
            properties: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other receiptProperties)]
        pub unsafe fn receiptProperties(&self) -> Option<Id<NSDictionary<NSString, AnyObject>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "SKRequest")]
    unsafe impl SKReceiptRefreshRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    pub fn SKTerminateForInvalidReceipt();
}

extern "C" {
    pub static SKReceiptPropertyIsExpired: &'static NSString;
}

extern "C" {
    pub static SKReceiptPropertyIsRevoked: &'static NSString;
}

extern "C" {
    pub static SKReceiptPropertyIsVolumePurchase: &'static NSString;
}
