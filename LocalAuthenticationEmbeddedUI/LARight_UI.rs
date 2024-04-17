//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;
use objc2_local_authentication::*;

use crate::*;

extern_category!(
    /// Category "UI" on [`LARight`].
    #[doc(alias = "UI")]
    pub unsafe trait LARightUI {
        #[cfg(all(
            feature = "LAPresentationContext",
            feature = "block2",
            feature = "objc2-app-kit"
        ))]
        #[method(authorizeWithLocalizedReason:inPresentationContext:completion:)]
        unsafe fn authorizeWithLocalizedReason_inPresentationContext_completion(
            &self,
            localized_reason: &NSString,
            presentation_context: &LAPresentationContext,
            handler: &Block<dyn Fn(*mut NSError)>,
        );
    }

    unsafe impl LARightUI for LARight {}
);
