//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait ASAuthorizationWebBrowserPlatformPublicKeyCredentialProvider {
        #[cfg(all(
            feature = "ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest",
            feature = "ASAuthorizationRequest",
            feature = "ASPublicKeyCredentialClientData"
        ))]
        #[method_id(@__retain_semantics Other createCredentialRegistrationRequestWithClientData:name:userID:)]
        unsafe fn createCredentialRegistrationRequestWithClientData_name_userID(
            &self,
            client_data: &ASPublicKeyCredentialClientData,
            name: &NSString,
            user_id: &NSData,
        ) -> Id<ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest>;

        #[cfg(all(
            feature = "ASAuthorizationPlatformPublicKeyCredentialAssertionRequest",
            feature = "ASAuthorizationRequest",
            feature = "ASPublicKeyCredentialClientData"
        ))]
        #[method_id(@__retain_semantics Other createCredentialAssertionRequestWithClientData:)]
        unsafe fn createCredentialAssertionRequestWithClientData(
            &self,
            client_data: &ASPublicKeyCredentialClientData,
        ) -> Id<ASAuthorizationPlatformPublicKeyCredentialAssertionRequest>;
    }

    unsafe impl ProtocolType for dyn ASAuthorizationWebBrowserPlatformPublicKeyCredentialProvider {}
);
