//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKLeaderboardEntry")]
    pub struct GKLeaderboardEntry;

    #[cfg(feature = "GameKit_GKLeaderboardEntry")]
    unsafe impl ClassType for GKLeaderboardEntry {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKLeaderboardEntry")]
unsafe impl NSObjectProtocol for GKLeaderboardEntry {}

extern_methods!(
    #[cfg(feature = "GameKit_GKLeaderboardEntry")]
    unsafe impl GKLeaderboardEntry {
        #[cfg(feature = "GameKit_GKPlayer")]
        #[method_id(@__retain_semantics Other player)]
        pub unsafe fn player(&self) -> Id<GKPlayer>;

        #[method(rank)]
        pub unsafe fn rank(&self) -> NSInteger;

        #[method(score)]
        pub unsafe fn score(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other formattedScore)]
        pub unsafe fn formattedScore(&self) -> Id<NSString>;

        #[method(context)]
        pub unsafe fn context(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Id<NSDate>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
