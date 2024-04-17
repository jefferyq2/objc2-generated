//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKInviteRecipientResponse(pub NSInteger);
impl GKInviteRecipientResponse {
    #[doc(alias = "GKInviteRecipientResponseAccepted")]
    pub const Accepted: Self = Self(0);
    #[doc(alias = "GKInviteRecipientResponseDeclined")]
    pub const Declined: Self = Self(1);
    #[doc(alias = "GKInviteRecipientResponseFailed")]
    pub const Failed: Self = Self(2);
    #[doc(alias = "GKInviteRecipientResponseIncompatible")]
    pub const Incompatible: Self = Self(3);
    #[doc(alias = "GKInviteRecipientResponseUnableToConnect")]
    pub const UnableToConnect: Self = Self(4);
    #[doc(alias = "GKInviteRecipientResponseNoAnswer")]
    pub const NoAnswer: Self = Self(5);
    pub const GKInviteeResponseAccepted: Self = Self(GKInviteRecipientResponse::Accepted.0);
    pub const GKInviteeResponseDeclined: Self = Self(GKInviteRecipientResponse::Declined.0);
    pub const GKInviteeResponseFailed: Self = Self(GKInviteRecipientResponse::Failed.0);
    pub const GKInviteeResponseIncompatible: Self = Self(GKInviteRecipientResponse::Incompatible.0);
    pub const GKInviteeResponseUnableToConnect: Self =
        Self(GKInviteRecipientResponse::UnableToConnect.0);
    pub const GKInviteeResponseNoAnswer: Self = Self(GKInviteRecipientResponse::NoAnswer.0);
}

unsafe impl Encode for GKInviteRecipientResponse {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GKInviteRecipientResponse {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub type GKInviteeResponse = GKInviteRecipientResponse;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GKMatchType(pub NSUInteger);
impl GKMatchType {
    #[doc(alias = "GKMatchTypePeerToPeer")]
    pub const PeerToPeer: Self = Self(0);
    #[doc(alias = "GKMatchTypeHosted")]
    pub const Hosted: Self = Self(1);
    #[doc(alias = "GKMatchTypeTurnBased")]
    pub const TurnBased: Self = Self(2);
}

unsafe impl Encode for GKMatchType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for GKMatchType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKMatchRequest;

    unsafe impl ClassType for GKMatchRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for GKMatchRequest {}

extern_methods!(
    unsafe impl GKMatchRequest {
        #[method(minPlayers)]
        pub unsafe fn minPlayers(&self) -> NSUInteger;

        #[method(setMinPlayers:)]
        pub unsafe fn setMinPlayers(&self, min_players: NSUInteger);

        #[method(maxPlayers)]
        pub unsafe fn maxPlayers(&self) -> NSUInteger;

        #[method(setMaxPlayers:)]
        pub unsafe fn setMaxPlayers(&self, max_players: NSUInteger);

        #[method(playerGroup)]
        pub unsafe fn playerGroup(&self) -> NSUInteger;

        #[method(setPlayerGroup:)]
        pub unsafe fn setPlayerGroup(&self, player_group: NSUInteger);

        #[method(playerAttributes)]
        pub unsafe fn playerAttributes(&self) -> u32;

        #[method(setPlayerAttributes:)]
        pub unsafe fn setPlayerAttributes(&self, player_attributes: u32);

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method_id(@__retain_semantics Other recipients)]
        pub unsafe fn recipients(&self) -> Option<Id<NSArray<GKPlayer>>>;

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method(setRecipients:)]
        pub unsafe fn setRecipients(&self, recipients: Option<&NSArray<GKPlayer>>);

        #[method_id(@__retain_semantics Other inviteMessage)]
        pub unsafe fn inviteMessage(&self) -> Option<Id<NSString>>;

        #[method(setInviteMessage:)]
        pub unsafe fn setInviteMessage(&self, invite_message: Option<&NSString>);

        #[method(defaultNumberOfPlayers)]
        pub unsafe fn defaultNumberOfPlayers(&self) -> NSUInteger;

        #[method(setDefaultNumberOfPlayers:)]
        pub unsafe fn setDefaultNumberOfPlayers(&self, default_number_of_players: NSUInteger);

        #[deprecated]
        #[method(restrictToAutomatch)]
        pub unsafe fn restrictToAutomatch(&self) -> bool;

        #[deprecated]
        #[method(setRestrictToAutomatch:)]
        pub unsafe fn setRestrictToAutomatch(&self, restrict_to_automatch: bool);

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer", feature = "block2"))]
        #[method(recipientResponseHandler)]
        pub unsafe fn recipientResponseHandler(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<GKPlayer>, GKInviteRecipientResponse)>;

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer", feature = "block2"))]
        #[method(setRecipientResponseHandler:)]
        pub unsafe fn setRecipientResponseHandler(
            &self,
            recipient_response_handler: Option<
                &Block<dyn Fn(NonNull<GKPlayer>, GKInviteRecipientResponse)>,
            >,
        );

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(inviteeResponseHandler)]
        pub unsafe fn inviteeResponseHandler(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<NSString>, GKInviteeResponse)>;

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(setInviteeResponseHandler:)]
        pub unsafe fn setInviteeResponseHandler(
            &self,
            invitee_response_handler: Option<&Block<dyn Fn(NonNull<NSString>, GKInviteeResponse)>>,
        );

        #[method(maxPlayersAllowedForMatchOfType:)]
        pub unsafe fn maxPlayersAllowedForMatchOfType(match_type: GKMatchType) -> NSUInteger;

        #[deprecated]
        #[method_id(@__retain_semantics Other playersToInvite)]
        pub unsafe fn playersToInvite(&self) -> Option<Id<NSArray<NSString>>>;

        #[deprecated]
        #[method(setPlayersToInvite:)]
        pub unsafe fn setPlayersToInvite(&self, players_to_invite: Option<&NSArray<NSString>>);

        #[method_id(@__retain_semantics Other queueName)]
        pub unsafe fn queueName(&self) -> Option<Id<NSString>>;

        #[method(setQueueName:)]
        pub unsafe fn setQueueName(&self, queue_name: Option<&NSString>);

        #[cfg(feature = "GKDefines")]
        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(&self) -> Option<Id<GKMatchProperties>>;

        #[cfg(feature = "GKDefines")]
        #[method(setProperties:)]
        pub unsafe fn setProperties(&self, properties: Option<&GKMatchProperties>);

        #[cfg(all(feature = "GKBasePlayer", feature = "GKDefines", feature = "GKPlayer"))]
        #[method_id(@__retain_semantics Other recipientProperties)]
        pub unsafe fn recipientProperties(
            &self,
        ) -> Option<Id<NSDictionary<GKPlayer, GKMatchProperties>>>;

        #[cfg(all(feature = "GKBasePlayer", feature = "GKDefines", feature = "GKPlayer"))]
        #[method(setRecipientProperties:)]
        pub unsafe fn setRecipientProperties(
            &self,
            recipient_properties: Option<&NSDictionary<GKPlayer, GKMatchProperties>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKMatchRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKInvite;

    unsafe impl ClassType for GKInvite {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for GKInvite {}

extern_methods!(
    unsafe impl GKInvite {
        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method_id(@__retain_semantics Other sender)]
        pub unsafe fn sender(&self) -> Id<GKPlayer>;

        #[method(isHosted)]
        pub unsafe fn isHosted(&self) -> bool;

        #[method(playerGroup)]
        pub unsafe fn playerGroup(&self) -> NSUInteger;

        #[method(playerAttributes)]
        pub unsafe fn playerAttributes(&self) -> u32;

        #[deprecated]
        #[method_id(@__retain_semantics Other inviter)]
        pub unsafe fn inviter(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKInvite {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait GKInviteEventListener {
        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[optional]
        #[method(player:didAcceptInvite:)]
        unsafe fn player_didAcceptInvite(&self, player: &GKPlayer, invite: &GKInvite);

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[optional]
        #[method(player:didRequestMatchWithRecipients:)]
        unsafe fn player_didRequestMatchWithRecipients(
            &self,
            player: &GKPlayer,
            recipient_players: &NSArray<GKPlayer>,
        );

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[deprecated]
        #[optional]
        #[method(player:didRequestMatchWithPlayers:)]
        unsafe fn player_didRequestMatchWithPlayers(
            &self,
            player: &GKPlayer,
            player_i_ds_to_invite: &NSArray<NSString>,
        );
    }

    unsafe impl ProtocolType for dyn GKInviteEventListener {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKMatchedPlayers;

    unsafe impl ClassType for GKMatchedPlayers {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for GKMatchedPlayers {}

extern_methods!(
    unsafe impl GKMatchedPlayers {
        #[cfg(feature = "GKDefines")]
        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(&self) -> Option<Id<GKMatchProperties>>;

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method_id(@__retain_semantics Other players)]
        pub unsafe fn players(&self) -> Id<NSArray<GKPlayer>>;

        #[cfg(all(feature = "GKBasePlayer", feature = "GKDefines", feature = "GKPlayer"))]
        #[method_id(@__retain_semantics Other playerProperties)]
        pub unsafe fn playerProperties(
            &self,
        ) -> Option<Id<NSDictionary<GKPlayer, GKMatchProperties>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKMatchedPlayers {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKMatchmaker;

    unsafe impl ClassType for GKMatchmaker {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for GKMatchmaker {}

extern_methods!(
    unsafe impl GKMatchmaker {
        #[method_id(@__retain_semantics Other sharedMatchmaker)]
        pub unsafe fn sharedMatchmaker() -> Id<GKMatchmaker>;

        #[cfg(all(feature = "GKMatch", feature = "block2"))]
        #[method(matchForInvite:completionHandler:)]
        pub unsafe fn matchForInvite_completionHandler(
            &self,
            invite: &GKInvite,
            completion_handler: Option<&Block<dyn Fn(*mut GKMatch, *mut NSError)>>,
        );

        #[cfg(all(feature = "GKMatch", feature = "block2"))]
        #[method(findMatchForRequest:withCompletionHandler:)]
        pub unsafe fn findMatchForRequest_withCompletionHandler(
            &self,
            request: &GKMatchRequest,
            completion_handler: Option<&Block<dyn Fn(*mut GKMatch, *mut NSError)>>,
        );

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer", feature = "block2"))]
        #[method(findPlayersForHostedRequest:withCompletionHandler:)]
        pub unsafe fn findPlayersForHostedRequest_withCompletionHandler(
            &self,
            request: &GKMatchRequest,
            completion_handler: Option<&Block<dyn Fn(*mut NSArray<GKPlayer>, *mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(findMatchedPlayers:withCompletionHandler:)]
        pub unsafe fn findMatchedPlayers_withCompletionHandler(
            &self,
            request: &GKMatchRequest,
            completion_handler: &Block<dyn Fn(*mut GKMatchedPlayers, *mut NSError)>,
        );

        #[cfg(all(feature = "GKMatch", feature = "block2"))]
        #[method(addPlayersToMatch:matchRequest:completionHandler:)]
        pub unsafe fn addPlayersToMatch_matchRequest_completionHandler(
            &self,
            r#match: &GKMatch,
            match_request: &GKMatchRequest,
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method(cancelPendingInviteToPlayer:)]
        pub unsafe fn cancelPendingInviteToPlayer(&self, player: &GKPlayer);

        #[cfg(feature = "GKMatch")]
        #[method(finishMatchmakingForMatch:)]
        pub unsafe fn finishMatchmakingForMatch(&self, r#match: &GKMatch);

        #[cfg(feature = "block2")]
        #[method(queryPlayerGroupActivity:withCompletionHandler:)]
        pub unsafe fn queryPlayerGroupActivity_withCompletionHandler(
            &self,
            player_group: NSUInteger,
            completion_handler: Option<&Block<dyn Fn(NSInteger, *mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(queryActivityWithCompletionHandler:)]
        pub unsafe fn queryActivityWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(NSInteger, *mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(queryQueueActivity:withCompletionHandler:)]
        pub unsafe fn queryQueueActivity_withCompletionHandler(
            &self,
            queue_name: &NSString,
            completion_handler: Option<&Block<dyn Fn(NSInteger, *mut NSError)>>,
        );

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer", feature = "block2"))]
        #[method(startBrowsingForNearbyPlayersWithHandler:)]
        pub unsafe fn startBrowsingForNearbyPlayersWithHandler(
            &self,
            reachable_handler: Option<&Block<dyn Fn(NonNull<GKPlayer>, Bool)>>,
        );

        #[method(stopBrowsingForNearbyPlayers)]
        pub unsafe fn stopBrowsingForNearbyPlayers(&self);

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer", feature = "block2"))]
        #[method(startGroupActivityWithPlayerHandler:)]
        pub unsafe fn startGroupActivityWithPlayerHandler(
            &self,
            handler: &Block<dyn Fn(NonNull<GKPlayer>)>,
        );

        #[method(stopGroupActivity)]
        pub unsafe fn stopGroupActivity(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKMatchmaker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// GKDeprecated
    unsafe impl GKMatchmaker {
        #[cfg(feature = "block2")]
        #[deprecated = "Use registerListener on GKLocalPlayer to register an object that implements the GKInviteEventListener instead."]
        #[method(inviteHandler)]
        pub unsafe fn inviteHandler(&self) -> *mut Block<dyn Fn(NonNull<GKInvite>, *mut NSArray)>;

        #[cfg(feature = "block2")]
        #[deprecated = "Use registerListener on GKLocalPlayer to register an object that implements the GKInviteEventListener instead."]
        #[method(setInviteHandler:)]
        pub unsafe fn setInviteHandler(
            &self,
            invite_handler: Option<&Block<dyn Fn(NonNull<GKInvite>, *mut NSArray)>>,
        );
    }
);

extern_methods!(
    /// Obsoleted
    unsafe impl GKMatchmaker {
        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(startBrowsingForNearbyPlayersWithReachableHandler:)]
        pub unsafe fn startBrowsingForNearbyPlayersWithReachableHandler(
            &self,
            reachable_handler: Option<&Block<dyn Fn(NonNull<NSString>, Bool)>>,
        );

        #[deprecated]
        #[method(cancelInviteToPlayer:)]
        pub unsafe fn cancelInviteToPlayer(&self, player_id: &NSString);

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(findPlayersForHostedMatchRequest:withCompletionHandler:)]
        pub unsafe fn findPlayersForHostedMatchRequest_withCompletionHandler(
            &self,
            request: &GKMatchRequest,
            completion_handler: Option<&Block<dyn Fn(*mut NSArray<NSString>, *mut NSError)>>,
        );
    }
);
