//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    #[deprecated = "Use CarPlay framework"]
    pub unsafe trait MPPlayableContentDelegate: NSObjectProtocol {
        #[cfg(all(feature = "MPPlayableContentManager", feature = "block2"))]
        #[deprecated = "Use CarPlay framework"]
        #[optional]
        #[method(playableContentManager:initiatePlaybackOfContentItemAtIndexPath:completionHandler:)]
        unsafe fn playableContentManager_initiatePlaybackOfContentItemAtIndexPath_completionHandler(
            &self,
            content_manager: &MPPlayableContentManager,
            index_path: &NSIndexPath,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "MPPlayableContentManager", feature = "block2"))]
        #[deprecated = "Use Intents framework for initiating playback queues."]
        #[optional]
        #[method(playableContentManager:initializePlaybackQueueWithCompletionHandler:)]
        unsafe fn playableContentManager_initializePlaybackQueueWithCompletionHandler(
            &self,
            content_manager: &MPPlayableContentManager,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "MPPlayableContentManager", feature = "block2"))]
        #[deprecated = "Use Intents framework for initiating playback queues."]
        #[optional]
        #[method(playableContentManager:initializePlaybackQueueWithContentItems:completionHandler:)]
        unsafe fn playableContentManager_initializePlaybackQueueWithContentItems_completionHandler(
            &self,
            content_manager: &MPPlayableContentManager,
            content_items: Option<&NSArray>,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(
            feature = "MPPlayableContentManager",
            feature = "MPPlayableContentManagerContext"
        ))]
        #[deprecated = "Use CarPlay framework"]
        #[optional]
        #[method(playableContentManager:didUpdateContext:)]
        unsafe fn playableContentManager_didUpdateContext(
            &self,
            content_manager: &MPPlayableContentManager,
            context: &MPPlayableContentManagerContext,
        );
    }

    unsafe impl ProtocolType for dyn MPPlayableContentDelegate {}
);
