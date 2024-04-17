// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "LocalAuthentication", kind = "framework")]
extern "C" {}

#[cfg(feature = "LABase")]
#[path = "LABase.rs"]
mod __LABase;
#[cfg(feature = "LAContext")]
#[path = "LAContext.rs"]
mod __LAContext;
#[cfg(feature = "LAError")]
#[path = "LAError.rs"]
mod __LAError;
#[cfg(feature = "LAPersistedRight")]
#[path = "LAPersistedRight.rs"]
mod __LAPersistedRight;
#[cfg(feature = "LAPrivateKey")]
#[path = "LAPrivateKey.rs"]
mod __LAPrivateKey;
#[cfg(feature = "LAPublicDefines")]
#[path = "LAPublicDefines.rs"]
mod __LAPublicDefines;
#[cfg(feature = "LAPublicKey")]
#[path = "LAPublicKey.rs"]
mod __LAPublicKey;
#[cfg(feature = "LARequirement")]
#[path = "LARequirement.rs"]
mod __LARequirement;
#[cfg(feature = "LARight")]
#[path = "LARight.rs"]
mod __LARight;
#[cfg(feature = "LARightStore")]
#[path = "LARightStore.rs"]
mod __LARightStore;
#[cfg(feature = "LASecret")]
#[path = "LASecret.rs"]
mod __LASecret;

#[cfg(feature = "LAContext")]
pub use self::__LAContext::LAAccessControlOperation;
#[cfg(feature = "LAContext")]
pub use self::__LAContext::LABiometryType;
#[cfg(feature = "LAContext")]
pub use self::__LAContext::LAContext;
#[cfg(feature = "LAContext")]
pub use self::__LAContext::LACredentialType;
#[cfg(feature = "LAContext")]
pub use self::__LAContext::LAPolicy;
#[cfg(feature = "LAContext")]
pub use self::__LAContext::LATouchIDAuthenticationMaximumAllowableReuseDuration;
#[cfg(feature = "LAError")]
pub use self::__LAError::LAError;
#[cfg(feature = "LAError")]
pub use self::__LAError::LAErrorDomain;
#[cfg(all(feature = "LAPersistedRight", feature = "LARight"))]
pub use self::__LAPersistedRight::LAPersistedRight;
#[cfg(feature = "LAPrivateKey")]
pub use self::__LAPrivateKey::LAPrivateKey;
#[cfg(feature = "LAPublicKey")]
pub use self::__LAPublicKey::LAPublicKey;
#[cfg(feature = "LARequirement")]
pub use self::__LARequirement::LAAuthenticationRequirement;
#[cfg(feature = "LARequirement")]
pub use self::__LARequirement::LABiometryFallbackRequirement;
#[cfg(feature = "LARight")]
pub use self::__LARight::LARight;
#[cfg(feature = "LARight")]
pub use self::__LARight::LARightState;
#[cfg(feature = "LARightStore")]
pub use self::__LARightStore::LARightStore;
#[cfg(feature = "LASecret")]
pub use self::__LASecret::LASecret;
