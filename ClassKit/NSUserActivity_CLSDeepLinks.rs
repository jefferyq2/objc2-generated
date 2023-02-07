//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ClassKit::*;
use crate::Foundation::*;

extern_methods!(
    /// CLSDeepLinks
    #[cfg(feature = "Foundation_NSUserActivity")]
    unsafe impl NSUserActivity {
        #[method(isClassKitDeepLink)]
        pub unsafe fn isClassKitDeepLink(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other contextIdentifierPath)]
        pub unsafe fn contextIdentifierPath(&self) -> Option<Id<NSArray<NSString>>>;
    }
);
