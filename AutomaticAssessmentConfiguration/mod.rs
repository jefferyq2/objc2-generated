//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "AEAssessmentApplication.rs"]
mod __AEAssessmentApplication;
#[path = "AEAssessmentConfiguration.rs"]
mod __AEAssessmentConfiguration;
#[path = "AEAssessmentParticipantConfiguration.rs"]
mod __AEAssessmentParticipantConfiguration;
#[path = "AEAssessmentSession.rs"]
mod __AEAssessmentSession;
#[path = "AEAssessmentSessionDelegate.rs"]
mod __AEAssessmentSessionDelegate;
#[path = "AEErrors.rs"]
mod __AEErrors;
#[path = "AEVisibility.rs"]
mod __AEVisibility;

#[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentApplication")]
pub use self::__AEAssessmentApplication::AEAssessmentApplication;
#[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentConfiguration")]
pub use self::__AEAssessmentConfiguration::AEAssessmentConfiguration;
pub use self::__AEAssessmentConfiguration::{
    AEAutocorrectMode, AEAutocorrectModeNone, AEAutocorrectModePunctuation,
    AEAutocorrectModeSpelling,
};
#[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentParticipantConfiguration")]
pub use self::__AEAssessmentParticipantConfiguration::AEAssessmentParticipantConfiguration;
#[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentSession")]
pub use self::__AEAssessmentSession::AEAssessmentSession;
pub use self::__AEAssessmentSessionDelegate::AEAssessmentSessionDelegate;
pub use self::__AEErrors::AEAssessmentErrorDomain;
pub use self::__AEErrors::{
    AEAssessmentErrorCode, AEAssessmentErrorUnknown, AEAssessmentErrorUnsupportedPlatform,
};