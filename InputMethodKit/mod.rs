// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `InputMethodKit` framework

#[path = "../../fixes/InputMethodKit/mod.rs"]
mod fixes;
pub use self::fixes::*;

#[cfg_attr(feature = "apple", link(name = "InputMethodKit", kind = "framework"))]
extern "C" {}

#[path = "IMKCandidates.rs"]
mod __IMKCandidates;
#[path = "IMKInputController.rs"]
mod __IMKInputController;
#[path = "IMKServer.rs"]
mod __IMKServer;

pub use self::__IMKCandidates::IMKCandidatePanelType;
#[cfg(feature = "InputMethodKit_IMKCandidates")]
pub use self::__IMKCandidates::IMKCandidates;
pub use self::__IMKCandidates::IMKCandidatesLocationHint;
pub use self::__IMKCandidates::IMKStyleType;
pub use self::__IMKCandidates::{kIMKAnnotation, kIMKMain, kIMKSubList};
pub use self::__IMKCandidates::{
    kIMKLocateCandidatesAboveHint, kIMKLocateCandidatesBelowHint, kIMKLocateCandidatesLeftHint,
    kIMKLocateCandidatesRightHint,
};
pub use self::__IMKCandidates::{
    kIMKScrollingGridCandidatePanel, kIMKSingleColumnScrollingCandidatePanel,
    kIMKSingleRowSteppingCandidatePanel,
};
#[cfg(feature = "InputMethodKit_IMKInputController")]
pub use self::__IMKInputController::IMKInputController;
pub use self::__IMKInputController::IMKMouseHandling;
pub use self::__IMKInputController::IMKStateSetting;
#[cfg(feature = "InputMethodKit_IMKServer")]
pub use self::__IMKServer::IMKServer;
