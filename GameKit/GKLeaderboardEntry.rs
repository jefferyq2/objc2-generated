//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKLeaderboardEntry;

    unsafe impl ClassType for GKLeaderboardEntry {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for GKLeaderboardEntry {}

extern_methods!(
    unsafe impl GKLeaderboardEntry {
        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method_id(@__retain_semantics Other player)]
        pub unsafe fn player(&self) -> Id<GKPlayer>;

        #[method(rank)]
        pub unsafe fn rank(&self) -> NSInteger;

        #[method(score)]
        pub unsafe fn score(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other formattedScore)]
        pub unsafe fn formattedScore(&self) -> Id<NSString>;

        #[method(context)]
        pub unsafe fn context(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Id<NSDate>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKLeaderboardEntry {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
