//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait ASAuthorizationControllerDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorization",
            feature = "AuthenticationServices_ASAuthorizationController"
        ))]
        #[optional]
        #[method(authorizationController:didCompleteWithAuthorization:)]
        unsafe fn authorizationController_didCompleteWithAuthorization(
            &self,
            controller: &ASAuthorizationController,
            authorization: &ASAuthorization,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationController",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(authorizationController:didCompleteWithError:)]
        unsafe fn authorizationController_didCompleteWithError(
            &self,
            controller: &ASAuthorizationController,
            error: &NSError,
        );

        #[cfg(feature = "AuthenticationServices_ASAuthorizationController")]
        #[optional]
        #[method(authorizationController:didCompleteWithCustomMethod:)]
        unsafe fn authorizationController_didCompleteWithCustomMethod(
            &self,
            controller: &ASAuthorizationController,
            method: &ASAuthorizationCustomMethod,
        );
    }

    unsafe impl ProtocolType for dyn ASAuthorizationControllerDelegate {}
);

extern_protocol!(
    pub unsafe trait ASAuthorizationControllerPresentationContextProviding:
        NSObjectProtocol
    {
        #[cfg(feature = "AuthenticationServices_ASAuthorizationController")]
        #[method_id(@__retain_semantics Other presentationAnchorForAuthorizationController:)]
        unsafe fn presentationAnchorForAuthorizationController(
            &self,
            controller: &ASAuthorizationController,
        ) -> Id<ASPresentationAnchor>;
    }

    unsafe impl ProtocolType for dyn ASAuthorizationControllerPresentationContextProviding {}
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum ASAuthorizationControllerRequestOptions {
        ASAuthorizationControllerRequestOptionPreferImmediatelyAvailableCredentials = 1 << 0,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationController")]
    pub struct ASAuthorizationController;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationController")]
    unsafe impl ClassType for ASAuthorizationController {
        type Super = NSObject;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationController")]
unsafe impl NSObjectProtocol for ASAuthorizationController {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationController")]
    unsafe impl ASAuthorizationController {
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationRequest",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other authorizationRequests)]
        pub unsafe fn authorizationRequests(&self) -> Id<NSArray<ASAuthorizationRequest>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn ASAuthorizationControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn ASAuthorizationControllerDelegate>>,
        );

        #[method_id(@__retain_semantics Other presentationContextProvider)]
        pub unsafe fn presentationContextProvider(
            &self,
        ) -> Option<Id<ProtocolObject<dyn ASAuthorizationControllerPresentationContextProviding>>>;

        #[method(setPresentationContextProvider:)]
        pub unsafe fn setPresentationContextProvider(
            &self,
            presentation_context_provider: Option<
                &ProtocolObject<dyn ASAuthorizationControllerPresentationContextProviding>,
            >,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other customAuthorizationMethods)]
        pub unsafe fn customAuthorizationMethods(&self)
            -> Id<NSArray<ASAuthorizationCustomMethod>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setCustomAuthorizationMethods:)]
        pub unsafe fn setCustomAuthorizationMethods(
            &self,
            custom_authorization_methods: &NSArray<ASAuthorizationCustomMethod>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationRequest",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Init initWithAuthorizationRequests:)]
        pub unsafe fn initWithAuthorizationRequests(
            this: Option<Allocated<Self>>,
            authorization_requests: &NSArray<ASAuthorizationRequest>,
        ) -> Id<Self>;

        #[method(performRequests)]
        pub unsafe fn performRequests(&self);

        #[method(performAutoFillAssistedRequests)]
        pub unsafe fn performAutoFillAssistedRequests(&self);

        #[method(performRequestsWithOptions:)]
        pub unsafe fn performRequestsWithOptions(
            &self,
            options: ASAuthorizationControllerRequestOptions,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
