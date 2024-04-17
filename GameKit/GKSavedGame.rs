//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKSavedGame;

    unsafe impl ClassType for GKSavedGame {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for GKSavedGame {}

unsafe impl NSObjectProtocol for GKSavedGame {}

extern_methods!(
    unsafe impl GKSavedGame {
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other deviceName)]
        pub unsafe fn deviceName(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other modificationDate)]
        pub unsafe fn modificationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "block2")]
        #[method(loadDataWithCompletionHandler:)]
        pub unsafe fn loadDataWithCompletionHandler(
            &self,
            handler: Option<&Block<dyn Fn(*mut NSData, *mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKSavedGame {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// GKSavedGame
    #[cfg(all(
        feature = "GKBasePlayer",
        feature = "GKLocalPlayer",
        feature = "GKPlayer"
    ))]
    unsafe impl GKLocalPlayer {
        #[cfg(feature = "block2")]
        #[method(fetchSavedGamesWithCompletionHandler:)]
        pub unsafe fn fetchSavedGamesWithCompletionHandler(
            &self,
            handler: Option<&Block<dyn Fn(*mut NSArray<GKSavedGame>, *mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(saveGameData:withName:completionHandler:)]
        pub unsafe fn saveGameData_withName_completionHandler(
            &self,
            data: &NSData,
            name: &NSString,
            handler: Option<&Block<dyn Fn(*mut GKSavedGame, *mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(deleteSavedGamesWithName:completionHandler:)]
        pub unsafe fn deleteSavedGamesWithName_completionHandler(
            &self,
            name: &NSString,
            handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(resolveConflictingSavedGames:withData:completionHandler:)]
        pub unsafe fn resolveConflictingSavedGames_withData_completionHandler(
            &self,
            conflicting_saved_games: &NSArray<GKSavedGame>,
            data: &NSData,
            handler: Option<&Block<dyn Fn(*mut NSArray<GKSavedGame>, *mut NSError)>>,
        );
    }
);

#[cfg(all(
    feature = "GKBasePlayer",
    feature = "GKLocalPlayer",
    feature = "GKPlayer",
    feature = "GKSavedGameListener"
))]
unsafe impl GKSavedGameListener for GKLocalPlayer {}
