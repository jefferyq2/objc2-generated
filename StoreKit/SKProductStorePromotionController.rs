//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

ns_closed_enum!(
    #[underlying(NSInteger)]
    pub enum SKProductStorePromotionVisibility {
        SKProductStorePromotionVisibilityDefault = 0,
        SKProductStorePromotionVisibilityShow = 1,
        SKProductStorePromotionVisibilityHide = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKProductStorePromotionController")]
    pub struct SKProductStorePromotionController;

    #[cfg(feature = "StoreKit_SKProductStorePromotionController")]
    unsafe impl ClassType for SKProductStorePromotionController {
        type Super = NSObject;
    }
);

#[cfg(feature = "StoreKit_SKProductStorePromotionController")]
unsafe impl NSObjectProtocol for SKProductStorePromotionController {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKProductStorePromotionController")]
    unsafe impl SKProductStorePromotionController {
        #[method_id(@__retain_semantics Other defaultController)]
        pub unsafe fn defaultController() -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "StoreKit_SKProduct"))]
        #[method(fetchStorePromotionVisibilityForProduct:completionHandler:)]
        pub unsafe fn fetchStorePromotionVisibilityForProduct_completionHandler(
            &self,
            product: &SKProduct,
            completion_handler: Option<
                &Block<(SKProductStorePromotionVisibility, *mut NSError), ()>,
            >,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "StoreKit_SKProduct"))]
        #[method(updateStorePromotionVisibility:forProduct:completionHandler:)]
        pub unsafe fn updateStorePromotionVisibility_forProduct_completionHandler(
            &self,
            promotion_visibility: SKProductStorePromotionVisibility,
            product: &SKProduct,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "StoreKit_SKProduct"
        ))]
        #[method(fetchStorePromotionOrderWithCompletionHandler:)]
        pub unsafe fn fetchStorePromotionOrderWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(NonNull<NSArray<SKProduct>>, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "StoreKit_SKProduct"
        ))]
        #[method(updateStorePromotionOrder:completionHandler:)]
        pub unsafe fn updateStorePromotionOrder_completionHandler(
            &self,
            promotion_order: &NSArray<SKProduct>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);
