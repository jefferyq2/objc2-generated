// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `ServiceManagement` framework

#[link(name = "ServiceManagement", kind = "framework")]
extern "C" {}

#[path = "SMAppService.rs"]
mod __SMAppService;
#[path = "SMErrors.rs"]
mod __SMErrors;
#[path = "SMLoginItem.rs"]
mod __SMLoginItem;

#[cfg(feature = "ServiceManagement_SMAppService")]
pub use self::__SMAppService::SMAppService;
pub use self::__SMAppService::SMAppServiceStatus;
pub use self::__SMAppService::{
    SMAppServiceStatusEnabled, SMAppServiceStatusNotFound, SMAppServiceStatusNotRegistered,
    SMAppServiceStatusRequiresApproval,
};
pub use self::__SMErrors::{
    kSMErrorAlreadyRegistered, kSMErrorAuthorizationFailure, kSMErrorInternalFailure,
    kSMErrorInvalidPlist, kSMErrorInvalidSignature, kSMErrorJobMustBeEnabled, kSMErrorJobNotFound,
    kSMErrorJobPlistNotFound, kSMErrorLaunchDeniedByUser, kSMErrorServiceUnavailable,
    kSMErrorToolNotValid,
};
