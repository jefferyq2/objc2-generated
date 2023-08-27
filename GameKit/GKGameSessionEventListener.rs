//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_protocol!(
    #[deprecated = "Use GKLocalPlayerListener for multiplayer event notifications."]
    pub unsafe trait GKGameSessionEventListener: NSObjectProtocol {
        #[cfg(all(feature = "GameKit_GKCloudPlayer", feature = "GameKit_GKGameSession"))]
        #[deprecated = "Use GKLocalPlayerListener for multiplayer event notifications."]
        #[optional]
        #[method(session:didAddPlayer:)]
        unsafe fn session_didAddPlayer(&self, session: &GKGameSession, player: &GKCloudPlayer);

        #[cfg(all(feature = "GameKit_GKCloudPlayer", feature = "GameKit_GKGameSession"))]
        #[deprecated = "Use GKLocalPlayerListener for multiplayer event notifications."]
        #[optional]
        #[method(session:didRemovePlayer:)]
        unsafe fn session_didRemovePlayer(&self, session: &GKGameSession, player: &GKCloudPlayer);

        #[cfg(all(feature = "GameKit_GKCloudPlayer", feature = "GameKit_GKGameSession"))]
        #[deprecated = "Use GKLocalPlayerListener for multiplayer event notifications."]
        #[optional]
        #[method(session:player:didChangeConnectionState:)]
        unsafe fn session_player_didChangeConnectionState(
            &self,
            session: &GKGameSession,
            player: &GKCloudPlayer,
            new_state: GKConnectionState,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "GameKit_GKCloudPlayer",
            feature = "GameKit_GKGameSession"
        ))]
        #[deprecated = "Use GKLocalPlayerListener for multiplayer event notifications."]
        #[optional]
        #[method(session:player:didSaveData:)]
        unsafe fn session_player_didSaveData(
            &self,
            session: &GKGameSession,
            player: &GKCloudPlayer,
            data: &NSData,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "GameKit_GKCloudPlayer",
            feature = "GameKit_GKGameSession"
        ))]
        #[deprecated = "Use GKLocalPlayerListener for multiplayer event notifications."]
        #[optional]
        #[method(session:didReceiveData:fromPlayer:)]
        unsafe fn session_didReceiveData_fromPlayer(
            &self,
            session: &GKGameSession,
            data: &NSData,
            player: &GKCloudPlayer,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSString",
            feature = "GameKit_GKCloudPlayer",
            feature = "GameKit_GKGameSession"
        ))]
        #[deprecated = "Use GKLocalPlayerListener for multiplayer event notifications."]
        #[optional]
        #[method(session:didReceiveMessage:withData:fromPlayer:)]
        unsafe fn session_didReceiveMessage_withData_fromPlayer(
            &self,
            session: &GKGameSession,
            message: &NSString,
            data: &NSData,
            player: &GKCloudPlayer,
        );
    }

    unsafe impl ProtocolType for dyn GKGameSessionEventListener {}
);

extern_methods!(
    /// GKGameSessionEventListener
    #[cfg(feature = "GameKit_GKGameSession")]
    unsafe impl GKGameSession {
        #[deprecated = "Use GKLocalPlayer's registerListener: to register for GKLocalPlayerListener event notifications."]
        #[method(addEventListener:)]
        pub unsafe fn addEventListener(listener: &NSObject);

        #[deprecated = "Use GKLocalPlayer's unregisterListener: or unregisterAllListeners to unregister from GKLocalPlayerListener event notifications."]
        #[method(removeEventListener:)]
        pub unsafe fn removeEventListener(listener: &NSObject);
    }
);
