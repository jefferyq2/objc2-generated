//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationAppleIDProviderCredentialState(pub NSInteger);
impl ASAuthorizationAppleIDProviderCredentialState {
    pub const ASAuthorizationAppleIDProviderCredentialRevoked: Self = Self(0);
    pub const ASAuthorizationAppleIDProviderCredentialAuthorized: Self = Self(1);
    pub const ASAuthorizationAppleIDProviderCredentialNotFound: Self = Self(2);
    pub const ASAuthorizationAppleIDProviderCredentialTransferred: Self = Self(3);
}

unsafe impl Encode for ASAuthorizationAppleIDProviderCredentialState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationAppleIDProviderCredentialState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static ASAuthorizationAppleIDProviderCredentialRevokedNotification:
        &'static NSNotificationName;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationAppleIDProvider;

    unsafe impl ClassType for ASAuthorizationAppleIDProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "ASAuthorizationProvider")]
unsafe impl ASAuthorizationProvider for ASAuthorizationAppleIDProvider {}

unsafe impl NSObjectProtocol for ASAuthorizationAppleIDProvider {}

extern_methods!(
    unsafe impl ASAuthorizationAppleIDProvider {
        #[cfg(all(
            feature = "ASAuthorizationAppleIDRequest",
            feature = "ASAuthorizationOpenIDRequest",
            feature = "ASAuthorizationRequest"
        ))]
        #[method_id(@__retain_semantics Other createRequest)]
        pub unsafe fn createRequest(&self) -> Id<ASAuthorizationAppleIDRequest>;

        #[cfg(feature = "block2")]
        #[method(getCredentialStateForUserID:completion:)]
        pub unsafe fn getCredentialStateForUserID_completion(
            &self,
            user_id: &NSString,
            completion: &Block<dyn Fn(ASAuthorizationAppleIDProviderCredentialState, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASAuthorizationAppleIDProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
