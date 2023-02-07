//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionWebBrowserSessionManager")]
    pub struct ASWebAuthenticationSessionWebBrowserSessionManager;

    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionWebBrowserSessionManager")]
    unsafe impl ClassType for ASWebAuthenticationSessionWebBrowserSessionManager {
        type Super = NSObject;
    }
);

#[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionWebBrowserSessionManager")]
unsafe impl NSObjectProtocol for ASWebAuthenticationSessionWebBrowserSessionManager {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionWebBrowserSessionManager")]
    unsafe impl ASWebAuthenticationSessionWebBrowserSessionManager {
        #[method_id(@__retain_semantics Other sharedManager)]
        pub unsafe fn sharedManager() -> Id<ASWebAuthenticationSessionWebBrowserSessionManager>;

        #[method_id(@__retain_semantics Other sessionHandler)]
        pub unsafe fn sessionHandler(
            &self,
        ) -> Id<ProtocolObject<dyn ASWebAuthenticationSessionWebBrowserSessionHandling>>;

        #[method(setSessionHandler:)]
        pub unsafe fn setSessionHandler(
            &self,
            session_handler: &ProtocolObject<
                dyn ASWebAuthenticationSessionWebBrowserSessionHandling,
            >,
        );

        #[method(wasLaunchedByAuthenticationServices)]
        pub unsafe fn wasLaunchedByAuthenticationServices(&self) -> bool;
    }
);
