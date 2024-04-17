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

#[link(name = "BackgroundAssets", kind = "framework")]
extern "C" {}

#[cfg(feature = "BAAppExtensionInfo")]
#[path = "BAAppExtensionInfo.rs"]
mod __BAAppExtensionInfo;
#[cfg(feature = "BABase")]
#[path = "BABase.rs"]
mod __BABase;
#[cfg(feature = "BADownload")]
#[path = "BADownload.rs"]
mod __BADownload;
#[cfg(feature = "BADownloadManager")]
#[path = "BADownloadManager.rs"]
mod __BADownloadManager;
#[cfg(feature = "BADownloaderExtension")]
#[path = "BADownloaderExtension.rs"]
mod __BADownloaderExtension;
#[cfg(feature = "BAError")]
#[path = "BAError.rs"]
mod __BAError;
#[cfg(feature = "BATypes")]
#[path = "BATypes.rs"]
mod __BATypes;
#[cfg(feature = "BAURLDownload")]
#[path = "BAURLDownload.rs"]
mod __BAURLDownload;

#[cfg(feature = "BAAppExtensionInfo")]
pub use self::__BAAppExtensionInfo::BAAppExtensionInfo;
#[cfg(feature = "BADownload")]
pub use self::__BADownload::BADownload;
#[cfg(feature = "BADownload")]
pub use self::__BADownload::BADownloadState;
#[cfg(feature = "BADownload")]
pub use self::__BADownload::BADownloaderPriority;
#[cfg(feature = "BADownload")]
pub use self::__BADownload::BADownloaderPriorityDefault;
#[cfg(feature = "BADownload")]
pub use self::__BADownload::BADownloaderPriorityMax;
#[cfg(feature = "BADownload")]
pub use self::__BADownload::BADownloaderPriorityMin;
#[cfg(feature = "BADownloadManager")]
pub use self::__BADownloadManager::BADownloadManager;
#[cfg(feature = "BADownloadManager")]
pub use self::__BADownloadManager::BADownloadManagerDelegate;
#[cfg(feature = "BADownloaderExtension")]
pub use self::__BADownloaderExtension::BADownloaderExtension;
#[cfg(feature = "BAError")]
pub use self::__BAError::BAErrorCode;
#[cfg(feature = "BAError")]
pub use self::__BAError::BAErrorDomain;
#[cfg(feature = "BATypes")]
pub use self::__BATypes::BAContentRequest;
#[cfg(all(feature = "BADownload", feature = "BAURLDownload"))]
pub use self::__BAURLDownload::BAURLDownload;
