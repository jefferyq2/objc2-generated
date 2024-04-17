//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASAuthorizationRequest")]
    pub struct ASAuthorizationPlatformPublicKeyCredentialAssertionRequest;

    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ClassType for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {
        #[inherits(NSObject)]
        type Super = ASAuthorizationRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "ASAuthorizationPublicKeyCredentialAssertionRequest",
    feature = "ASAuthorizationRequest"
))]
unsafe impl ASAuthorizationPublicKeyCredentialAssertionRequest
    for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest
{
}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSCoding for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSCopying for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSObjectProtocol for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl NSSecureCoding for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}

extern_methods!(
    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {
        #[cfg(feature = "ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
        #[method_id(@__retain_semantics Other allowedCredentials)]
        pub unsafe fn allowedCredentials(
            &self,
        ) -> Id<NSArray<ASAuthorizationPlatformPublicKeyCredentialDescriptor>>;

        #[cfg(feature = "ASAuthorizationPlatformPublicKeyCredentialDescriptor")]
        #[method(setAllowedCredentials:)]
        pub unsafe fn setAllowedCredentials(
            &self,
            allowed_credentials: &NSArray<ASAuthorizationPlatformPublicKeyCredentialDescriptor>,
        );

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput")]
        #[method_id(@__retain_semantics Other largeBlob)]
        pub unsafe fn largeBlob(
            &self,
        ) -> Option<Id<ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput>>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput")]
        #[method(setLargeBlob:)]
        pub unsafe fn setLargeBlob(
            &self,
            large_blob: Option<&ASAuthorizationPublicKeyCredentialLargeBlobAssertionInput>,
        );

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}
);

#[cfg(all(
    feature = "ASAuthorizationRequest",
    feature = "ASAuthorizationWebBrowserExternallyAuthenticatableRequest"
))]
unsafe impl ASAuthorizationWebBrowserExternallyAuthenticatableRequest
    for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest
{
}

extern_methods!(
    #[cfg(feature = "ASAuthorizationRequest")]
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialAssertionRequest {}
);

#[cfg(all(
    feature = "ASAuthorizationRequest",
    feature = "ASAuthorizationWebBrowserPlatformPublicKeyCredentialAssertionRequest"
))]
unsafe impl ASAuthorizationWebBrowserPlatformPublicKeyCredentialAssertionRequest
    for ASAuthorizationPlatformPublicKeyCredentialAssertionRequest
{
}
