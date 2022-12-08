//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct ASAccountAuthenticationModificationControllerDelegate;

    unsafe impl ASAccountAuthenticationModificationControllerDelegate {
        #[optional]
        #[method(accountAuthenticationModificationController:didSuccessfullyCompleteRequest:withUserInfo:)]
        pub unsafe fn accountAuthenticationModificationController_didSuccessfullyCompleteRequest_withUserInfo(
            &self,
            controller: &ASAccountAuthenticationModificationController,
            request: &ASAccountAuthenticationModificationRequest,
            userInfo: Option<&NSDictionary>,
        );

        #[optional]
        #[method(accountAuthenticationModificationController:didFailRequest:withError:)]
        pub unsafe fn accountAuthenticationModificationController_didFailRequest_withError(
            &self,
            controller: &ASAccountAuthenticationModificationController,
            request: &ASAccountAuthenticationModificationRequest,
            error: &NSError,
        );
    }
);

extern_protocol!(
    pub struct ASAccountAuthenticationModificationControllerPresentationContextProviding;

    unsafe impl ASAccountAuthenticationModificationControllerPresentationContextProviding {
        #[method_id(@__retain_semantics Other presentationAnchorForAccountAuthenticationModificationController:)]
        pub unsafe fn presentationAnchorForAccountAuthenticationModificationController(
            &self,
            controller: &ASAccountAuthenticationModificationController,
        ) -> Id<ASPresentationAnchor, Shared>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct ASAccountAuthenticationModificationController;

    unsafe impl ClassType for ASAccountAuthenticationModificationController {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl ASAccountAuthenticationModificationController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ASAccountAuthenticationModificationControllerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ASAccountAuthenticationModificationControllerDelegate>,
        );

        #[method_id(@__retain_semantics Other presentationContextProvider)]
        pub unsafe fn presentationContextProvider(
            &self,
        ) -> Option<
            Id<ASAccountAuthenticationModificationControllerPresentationContextProviding, Shared>,
        >;

        #[method(setPresentationContextProvider:)]
        pub unsafe fn setPresentationContextProvider(
            &self,
            presentationContextProvider: Option<
                &ASAccountAuthenticationModificationControllerPresentationContextProviding,
            >,
        );

        #[method(performRequest:)]
        pub unsafe fn performRequest(&self, request: &ASAccountAuthenticationModificationRequest);
    }
);