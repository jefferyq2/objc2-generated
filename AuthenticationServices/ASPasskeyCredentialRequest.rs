//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequest")]
    pub struct ASPasskeyCredentialRequest;

    #[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequest")]
    unsafe impl ClassType for ASPasskeyCredentialRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequest")]
unsafe impl ASCredentialRequest for ASPasskeyCredentialRequest {}

#[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequest")]
unsafe impl NSCoding for ASPasskeyCredentialRequest {}

#[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequest")]
unsafe impl NSCopying for ASPasskeyCredentialRequest {}

#[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequest")]
unsafe impl NSObjectProtocol for ASPasskeyCredentialRequest {}

#[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequest")]
unsafe impl NSSecureCoding for ASPasskeyCredentialRequest {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequest")]
    unsafe impl ASPasskeyCredentialRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(
            feature = "AuthenticationServices_ASPasskeyCredentialIdentity",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSNumber",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithCredentialIdentity:clientDataHash:userVerificationPreference:supportedAlgorithms:)]
        pub unsafe fn initWithCredentialIdentity_clientDataHash_userVerificationPreference_supportedAlgorithms(
            this: Allocated<Self>,
            credential_identity: &ASPasskeyCredentialIdentity,
            client_data_hash: &NSData,
            user_verification_preference: &ASAuthorizationPublicKeyCredentialUserVerificationPreference,
            supported_algorithms: &NSArray<NSNumber>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "AuthenticationServices_ASPasskeyCredentialIdentity",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSNumber",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other requestWithCredentialIdentity:clientDataHash:userVerificationPreference:supportedAlgorithms:)]
        pub unsafe fn requestWithCredentialIdentity_clientDataHash_userVerificationPreference_supportedAlgorithms(
            credential_identity: &ASPasskeyCredentialIdentity,
            client_data_hash: &NSData,
            user_verification_preference: &ASAuthorizationPublicKeyCredentialUserVerificationPreference,
            supported_algorithms: &NSArray<NSNumber>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other clientDataHash)]
        pub unsafe fn clientDataHash(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other userVerificationPreference)]
        pub unsafe fn userVerificationPreference(
            &self,
        ) -> Id<ASAuthorizationPublicKeyCredentialUserVerificationPreference>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setUserVerificationPreference:)]
        pub unsafe fn setUserVerificationPreference(
            &self,
            user_verification_preference: &ASAuthorizationPublicKeyCredentialUserVerificationPreference,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other supportedAlgorithms)]
        pub unsafe fn supportedAlgorithms(&self) -> Id<NSArray<NSNumber>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASPasskeyCredentialRequest")]
    unsafe impl ASPasskeyCredentialRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
