// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `ExternalAccessory` framework
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

#[link(name = "ExternalAccessory", kind = "framework")]
extern "C" {}

#[path = "EAAccessory.rs"]
mod __EAAccessory;
#[path = "EAAccessoryManager.rs"]
mod __EAAccessoryManager;
#[path = "EASession.rs"]
mod __EASession;
#[path = "EAWiFiUnconfiguredAccessory.rs"]
mod __EAWiFiUnconfiguredAccessory;
#[path = "EAWiFiUnconfiguredAccessoryBrowser.rs"]
mod __EAWiFiUnconfiguredAccessoryBrowser;
#[path = "ExternalAccessoryDefines.rs"]
mod __ExternalAccessoryDefines;

#[cfg(feature = "ExternalAccessory_EAAccessory")]
pub use self::__EAAccessory::EAAccessory;
pub use self::__EAAccessory::EAAccessoryDelegate;
pub use self::__EAAccessory::EAConnectionIDNone;
#[cfg(feature = "Foundation_NSString")]
pub use self::__EAAccessoryManager::EAAccessoryDidConnectNotification;
#[cfg(feature = "Foundation_NSString")]
pub use self::__EAAccessoryManager::EAAccessoryDidDisconnectNotification;
#[cfg(feature = "Foundation_NSString")]
pub use self::__EAAccessoryManager::EAAccessoryKey;
#[cfg(feature = "ExternalAccessory_EAAccessoryManager")]
pub use self::__EAAccessoryManager::EAAccessoryManager;
#[cfg(feature = "Foundation_NSString")]
pub use self::__EAAccessoryManager::EAAccessorySelectedKey;
#[cfg(feature = "Foundation_NSError")]
pub use self::__EAAccessoryManager::EABluetoothAccessoryPickerCompletion;
pub use self::__EAAccessoryManager::EABluetoothAccessoryPickerErrorCode;
#[cfg(feature = "Foundation_NSString")]
pub use self::__EAAccessoryManager::EABluetoothAccessoryPickerErrorDomain;
#[cfg(feature = "ExternalAccessory_EASession")]
pub use self::__EASession::EASession;
#[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessory")]
pub use self::__EAWiFiUnconfiguredAccessory::EAWiFiUnconfiguredAccessory;
pub use self::__EAWiFiUnconfiguredAccessory::EAWiFiUnconfiguredAccessoryProperties;
#[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
pub use self::__EAWiFiUnconfiguredAccessoryBrowser::EAWiFiUnconfiguredAccessoryBrowser;
pub use self::__EAWiFiUnconfiguredAccessoryBrowser::EAWiFiUnconfiguredAccessoryBrowserDelegate;
pub use self::__EAWiFiUnconfiguredAccessoryBrowser::EAWiFiUnconfiguredAccessoryBrowserState;
pub use self::__EAWiFiUnconfiguredAccessoryBrowser::EAWiFiUnconfiguredAccessoryConfigurationStatus;
