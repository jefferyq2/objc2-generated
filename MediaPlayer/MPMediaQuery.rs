//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MPMediaGrouping {
        MPMediaGroupingTitle = 0,
        MPMediaGroupingAlbum = 1,
        MPMediaGroupingArtist = 2,
        MPMediaGroupingAlbumArtist = 3,
        MPMediaGroupingComposer = 4,
        MPMediaGroupingGenre = 5,
        MPMediaGroupingPlaylist = 6,
        MPMediaGroupingPodcastTitle = 7,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMediaQuery")]
    pub struct MPMediaQuery;

    #[cfg(feature = "MediaPlayer_MPMediaQuery")]
    unsafe impl ClassType for MPMediaQuery {
        type Super = NSObject;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaQuery")]
unsafe impl NSCoding for MPMediaQuery {}

#[cfg(feature = "MediaPlayer_MPMediaQuery")]
unsafe impl NSObjectProtocol for MPMediaQuery {}

#[cfg(feature = "MediaPlayer_MPMediaQuery")]
unsafe impl NSSecureCoding for MPMediaQuery {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMediaQuery")]
    unsafe impl MPMediaQuery {
        #[cfg(all(feature = "Foundation_NSSet", feature = "MediaPlayer_MPMediaPredicate"))]
        #[method_id(@__retain_semantics Init initWithFilterPredicates:)]
        pub unsafe fn initWithFilterPredicates(
            this: Option<Allocated<Self>>,
            filter_predicates: Option<&NSSet<MPMediaPredicate>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "MediaPlayer_MPMediaPredicate"))]
        #[method_id(@__retain_semantics Other filterPredicates)]
        pub unsafe fn filterPredicates(&self) -> Option<Id<NSSet<MPMediaPredicate>>>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "MediaPlayer_MPMediaPredicate"))]
        #[method(setFilterPredicates:)]
        pub unsafe fn setFilterPredicates(
            &self,
            filter_predicates: Option<&NSSet<MPMediaPredicate>>,
        );

        #[cfg(feature = "MediaPlayer_MPMediaPredicate")]
        #[method(addFilterPredicate:)]
        pub unsafe fn addFilterPredicate(&self, predicate: &MPMediaPredicate);

        #[cfg(feature = "MediaPlayer_MPMediaPredicate")]
        #[method(removeFilterPredicate:)]
        pub unsafe fn removeFilterPredicate(&self, predicate: &MPMediaPredicate);

        #[cfg(all(feature = "Foundation_NSArray", feature = "MediaPlayer_MPMediaItem"))]
        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Option<Id<NSArray<MPMediaItem>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MediaPlayer_MPMediaItemCollection"
        ))]
        #[method_id(@__retain_semantics Other collections)]
        pub unsafe fn collections(&self) -> Option<Id<NSArray<MPMediaItemCollection>>>;

        #[method(groupingType)]
        pub unsafe fn groupingType(&self) -> MPMediaGrouping;

        #[method(setGroupingType:)]
        pub unsafe fn setGroupingType(&self, grouping_type: MPMediaGrouping);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MediaPlayer_MPMediaQuerySection"
        ))]
        #[method_id(@__retain_semantics Other itemSections)]
        pub unsafe fn itemSections(&self) -> Option<Id<NSArray<MPMediaQuerySection>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MediaPlayer_MPMediaQuerySection"
        ))]
        #[method_id(@__retain_semantics Other collectionSections)]
        pub unsafe fn collectionSections(&self) -> Option<Id<NSArray<MPMediaQuerySection>>>;

        #[method_id(@__retain_semantics Other albumsQuery)]
        pub unsafe fn albumsQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other artistsQuery)]
        pub unsafe fn artistsQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other songsQuery)]
        pub unsafe fn songsQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other playlistsQuery)]
        pub unsafe fn playlistsQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other podcastsQuery)]
        pub unsafe fn podcastsQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other audiobooksQuery)]
        pub unsafe fn audiobooksQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other compilationsQuery)]
        pub unsafe fn compilationsQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other composersQuery)]
        pub unsafe fn composersQuery() -> Id<MPMediaQuery>;

        #[method_id(@__retain_semantics Other genresQuery)]
        pub unsafe fn genresQuery() -> Id<MPMediaQuery>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMediaPredicate")]
    pub struct MPMediaPredicate;

    #[cfg(feature = "MediaPlayer_MPMediaPredicate")]
    unsafe impl ClassType for MPMediaPredicate {
        type Super = NSObject;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaPredicate")]
unsafe impl NSCoding for MPMediaPredicate {}

#[cfg(feature = "MediaPlayer_MPMediaPredicate")]
unsafe impl NSObjectProtocol for MPMediaPredicate {}

#[cfg(feature = "MediaPlayer_MPMediaPredicate")]
unsafe impl NSSecureCoding for MPMediaPredicate {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMediaPredicate")]
    unsafe impl MPMediaPredicate {}
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MPMediaPredicateComparison {
        MPMediaPredicateComparisonEqualTo = 0,
        MPMediaPredicateComparisonContains = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMediaPropertyPredicate")]
    pub struct MPMediaPropertyPredicate;

    #[cfg(feature = "MediaPlayer_MPMediaPropertyPredicate")]
    unsafe impl ClassType for MPMediaPropertyPredicate {
        #[inherits(NSObject)]
        type Super = MPMediaPredicate;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaPropertyPredicate")]
unsafe impl NSCoding for MPMediaPropertyPredicate {}

#[cfg(feature = "MediaPlayer_MPMediaPropertyPredicate")]
unsafe impl NSObjectProtocol for MPMediaPropertyPredicate {}

#[cfg(feature = "MediaPlayer_MPMediaPropertyPredicate")]
unsafe impl NSSecureCoding for MPMediaPropertyPredicate {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMediaPropertyPredicate")]
    unsafe impl MPMediaPropertyPredicate {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other predicateWithValue:forProperty:)]
        pub unsafe fn predicateWithValue_forProperty(
            value: Option<&Object>,
            property: &NSString,
        ) -> Id<MPMediaPropertyPredicate>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other predicateWithValue:forProperty:comparisonType:)]
        pub unsafe fn predicateWithValue_forProperty_comparisonType(
            value: Option<&Object>,
            property: &NSString,
            comparison_type: MPMediaPredicateComparison,
        ) -> Id<MPMediaPropertyPredicate>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other property)]
        pub unsafe fn property(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Id<Object>>;

        #[method(comparisonType)]
        pub unsafe fn comparisonType(&self) -> MPMediaPredicateComparison;
    }
);

extern_methods!(
    /// MPMediaQueryAdditions
    #[cfg(feature = "MediaPlayer_MPMediaItem")]
    unsafe impl MPMediaItem {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other persistentIDPropertyForGroupingType:)]
        pub unsafe fn persistentIDPropertyForGroupingType(
            grouping_type: MPMediaGrouping,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other titlePropertyForGroupingType:)]
        pub unsafe fn titlePropertyForGroupingType(grouping_type: MPMediaGrouping) -> Id<NSString>;
    }
);