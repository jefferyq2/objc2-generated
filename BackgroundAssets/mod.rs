// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `BackgroundAssets` framework
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

#[link(name = "BackgroundAssets", kind = "framework")]
extern "C" {}

#[path = "BAAppExtensionInfo.rs"]
mod __BAAppExtensionInfo;
#[path = "BABase.rs"]
mod __BABase;
#[path = "BADownload.rs"]
mod __BADownload;
#[path = "BADownloadManager.rs"]
mod __BADownloadManager;
#[path = "BADownloaderExtension.rs"]
mod __BADownloaderExtension;
#[path = "BAError.rs"]
mod __BAError;
#[path = "BATypes.rs"]
mod __BATypes;
#[path = "BAURLDownload.rs"]
mod __BAURLDownload;

#[cfg(feature = "BackgroundAssets_BAAppExtensionInfo")]
pub use self::__BAAppExtensionInfo::BAAppExtensionInfo;
#[cfg(feature = "BackgroundAssets_BADownload")]
pub use self::__BADownload::BADownload;
pub use self::__BADownload::BADownloadState;
pub use self::__BADownload::BADownloaderPriority;
pub use self::__BADownload::BADownloaderPriorityDefault;
pub use self::__BADownload::BADownloaderPriorityMax;
pub use self::__BADownload::BADownloaderPriorityMin;
#[cfg(feature = "BackgroundAssets_BADownloadManager")]
pub use self::__BADownloadManager::BADownloadManager;
pub use self::__BADownloadManager::BADownloadManagerDelegate;
pub use self::__BADownloaderExtension::BADownloaderExtension;
pub use self::__BAError::BAErrorCode;
#[cfg(feature = "Foundation_NSString")]
pub use self::__BAError::BAErrorDomain;
pub use self::__BATypes::BAContentRequest;
#[cfg(feature = "BackgroundAssets_BAURLDownload")]
pub use self::__BAURLDownload::BAURLDownload;
