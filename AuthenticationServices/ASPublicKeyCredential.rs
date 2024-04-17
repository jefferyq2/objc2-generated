//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    #[cfg(feature = "ASAuthorizationCredential")]
    pub unsafe trait ASPublicKeyCredential: ASAuthorizationCredential {
        #[method_id(@__retain_semantics Other rawClientDataJSON)]
        unsafe fn rawClientDataJSON(&self) -> Id<NSData>;

        #[method_id(@__retain_semantics Other credentialID)]
        unsafe fn credentialID(&self) -> Id<NSData>;
    }

    #[cfg(feature = "ASAuthorizationCredential")]
    unsafe impl ProtocolType for dyn ASPublicKeyCredential {}
);
