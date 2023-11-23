// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `FileProviderUI` framework

#[cfg_attr(feature = "apple", link(name = "FileProviderUI", kind = "framework"))]
extern "C" {}

#[path = "FPUIActionExtensionContext.rs"]
mod __FPUIActionExtensionContext;
#[path = "FPUIActionExtensionViewController.rs"]
mod __FPUIActionExtensionViewController;
#[path = "FPUIBase.rs"]
mod __FPUIBase;

#[cfg(feature = "FileProviderUI_FPUIActionExtensionContext")]
pub use self::__FPUIActionExtensionContext::FPUIActionExtensionContext;
pub use self::__FPUIActionExtensionContext::FPUIActionIdentifier;
pub use self::__FPUIActionExtensionContext::FPUIErrorDomain;
pub use self::__FPUIActionExtensionContext::FPUIExtensionErrorCode;
pub use self::__FPUIActionExtensionContext::{
    FPUIExtensionErrorCodeFailed, FPUIExtensionErrorCodeUserCancelled,
};
#[cfg(feature = "FileProviderUI_FPUIActionExtensionViewController")]
pub use self::__FPUIActionExtensionViewController::FPUIActionExtensionViewController;
