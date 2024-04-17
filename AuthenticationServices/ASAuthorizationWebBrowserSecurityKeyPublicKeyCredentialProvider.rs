//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait ASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialProvider {
        #[cfg(all(
            feature = "ASAuthorizationRequest",
            feature = "ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest",
            feature = "ASPublicKeyCredentialClientData"
        ))]
        #[method_id(@__retain_semantics Other createCredentialRegistrationRequestWithClientData:displayName:name:userID:)]
        unsafe fn createCredentialRegistrationRequestWithClientData_displayName_name_userID(
            &self,
            client_data: &ASPublicKeyCredentialClientData,
            display_name: &NSString,
            name: &NSString,
            user_id: &NSData,
        ) -> Id<ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest>;

        #[cfg(all(
            feature = "ASAuthorizationRequest",
            feature = "ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest",
            feature = "ASPublicKeyCredentialClientData"
        ))]
        #[method_id(@__retain_semantics Other createCredentialAssertionRequestWithClientData:)]
        unsafe fn createCredentialAssertionRequestWithClientData(
            &self,
            client_data: &ASPublicKeyCredentialClientData,
        ) -> Id<ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest>;
    }

    unsafe impl ProtocolType for dyn ASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialProvider {}
);
