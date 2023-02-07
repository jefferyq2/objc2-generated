//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKScore")]
    #[deprecated = "Replaced by GKLeaderboardScore"]
    pub struct GKScore;

    #[cfg(feature = "GameKit_GKScore")]
    unsafe impl ClassType for GKScore {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKScore")]
unsafe impl NSCoding for GKScore {}

#[cfg(feature = "GameKit_GKScore")]
unsafe impl NSObjectProtocol for GKScore {}

#[cfg(feature = "GameKit_GKScore")]
unsafe impl NSSecureCoding for GKScore {}

extern_methods!(
    #[cfg(feature = "GameKit_GKScore")]
    unsafe impl GKScore {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithLeaderboardIdentifier:)]
        pub unsafe fn initWithLeaderboardIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "GameKit_GKPlayer"))]
        #[method_id(@__retain_semantics Init initWithLeaderboardIdentifier:player:)]
        pub unsafe fn initWithLeaderboardIdentifier_player(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            player: &GKPlayer,
        ) -> Id<Self>;

        #[method(value)]
        pub unsafe fn value(&self) -> i64;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: i64);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other formattedValue)]
        pub unsafe fn formattedValue(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other leaderboardIdentifier)]
        pub unsafe fn leaderboardIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLeaderboardIdentifier:)]
        pub unsafe fn setLeaderboardIdentifier(&self, leaderboard_identifier: &NSString);

        #[method(context)]
        pub unsafe fn context(&self) -> u64;

        #[method(setContext:)]
        pub unsafe fn setContext(&self, context: u64);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Id<NSDate>;

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method_id(@__retain_semantics Other player)]
        pub unsafe fn player(&self) -> Option<Id<GKPlayer>>;

        #[method(rank)]
        pub unsafe fn rank(&self) -> NSInteger;

        #[method(shouldSetDefaultLeaderboard)]
        pub unsafe fn shouldSetDefaultLeaderboard(&self) -> bool;

        #[method(setShouldSetDefaultLeaderboard:)]
        pub unsafe fn setShouldSetDefaultLeaderboard(&self, should_set_default_leaderboard: bool);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(reportScores:withCompletionHandler:)]
        pub unsafe fn reportScores_withCompletionHandler(
            scores: &NSArray<GKScore>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);

extern_methods!(
    /// Deprecated
    #[cfg(feature = "GameKit_GKScore")]
    unsafe impl GKScore {
        #[cfg(feature = "Foundation_NSError")]
        #[deprecated = "Use +reportScores:withCompletionhandler: instead"]
        #[method(reportScoreWithCompletionHandler:)]
        pub unsafe fn reportScoreWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use initWithLeaderboardIdentifier: instead"]
        #[method_id(@__retain_semantics Init initWithCategory:)]
        pub unsafe fn initWithCategory(
            this: Option<Allocated<Self>>,
            category: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use leaderboardIdentifier instead"]
        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use leaderboardIdentifier instead"]
        #[method(setCategory:)]
        pub unsafe fn setCategory(&self, category: Option<&NSString>);
    }
);

extern_methods!(
    /// Obsoleted
    #[cfg(feature = "GameKit_GKScore")]
    unsafe impl GKScore {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithLeaderboardIdentifier:forPlayer:)]
        pub unsafe fn initWithLeaderboardIdentifier_forPlayer(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            player_id: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "use player instead"]
        #[method_id(@__retain_semantics Other playerID)]
        pub unsafe fn playerID(&self) -> Option<Id<NSString>>;
    }
);
