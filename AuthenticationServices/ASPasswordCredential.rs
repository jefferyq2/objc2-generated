//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasswordCredential;

    unsafe impl ClassType for ASPasswordCredential {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "ASAuthorizationCredential")]
unsafe impl ASAuthorizationCredential for ASPasswordCredential {}

unsafe impl NSCoding for ASPasswordCredential {}

unsafe impl NSCopying for ASPasswordCredential {}

unsafe impl NSObjectProtocol for ASPasswordCredential {}

unsafe impl NSSecureCoding for ASPasswordCredential {}

extern_methods!(
    unsafe impl ASPasswordCredential {
        #[method_id(@__retain_semantics Init initWithUser:password:)]
        pub unsafe fn initWithUser_password(
            this: Allocated<Self>,
            user: &NSString,
            password: &NSString,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other credentialWithUser:password:)]
        pub unsafe fn credentialWithUser_password(user: &NSString, password: &NSString)
            -> Id<Self>;

        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other password)]
        pub unsafe fn password(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASPasswordCredential {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
