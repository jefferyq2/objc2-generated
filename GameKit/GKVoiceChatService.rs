//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKVoiceChatService;

    unsafe impl ClassType for GKVoiceChatService {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for GKVoiceChatService {}

extern_methods!(
    unsafe impl GKVoiceChatService {
        #[method_id(@__retain_semantics Other defaultVoiceChatService)]
        pub unsafe fn defaultVoiceChatService() -> Option<Id<GKVoiceChatService>>;

        #[method(isVoIPAllowed)]
        pub unsafe fn isVoIPAllowed() -> bool;

        #[cfg(feature = "GKPublicProtocols")]
        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Option<Id<ProtocolObject<dyn GKVoiceChatClient>>>;

        #[cfg(feature = "GKPublicProtocols")]
        #[method(setClient:)]
        pub unsafe fn setClient(&self, client: Option<&ProtocolObject<dyn GKVoiceChatClient>>);

        #[method(stopVoiceChatWithParticipantID:)]
        pub unsafe fn stopVoiceChatWithParticipantID(&self, participant_id: Option<&NSString>);

        #[method(denyCallID:)]
        pub unsafe fn denyCallID(&self, call_id: NSInteger);

        #[method(receivedRealTimeData:fromParticipantID:)]
        pub unsafe fn receivedRealTimeData_fromParticipantID(
            &self,
            audio: Option<&NSData>,
            participant_id: Option<&NSString>,
        );

        #[method(receivedData:fromParticipantID:)]
        pub unsafe fn receivedData_fromParticipantID(
            &self,
            arbitrary_data: Option<&NSData>,
            participant_id: Option<&NSString>,
        );

        #[method(isMicrophoneMuted)]
        pub unsafe fn isMicrophoneMuted(&self) -> bool;

        #[method(setMicrophoneMuted:)]
        pub unsafe fn setMicrophoneMuted(&self, microphone_muted: bool);

        #[method(remoteParticipantVolume)]
        pub unsafe fn remoteParticipantVolume(&self) -> c_float;

        #[method(setRemoteParticipantVolume:)]
        pub unsafe fn setRemoteParticipantVolume(&self, remote_participant_volume: c_float);

        #[method(isOutputMeteringEnabled)]
        pub unsafe fn isOutputMeteringEnabled(&self) -> bool;

        #[method(setOutputMeteringEnabled:)]
        pub unsafe fn setOutputMeteringEnabled(&self, output_metering_enabled: bool);

        #[method(isInputMeteringEnabled)]
        pub unsafe fn isInputMeteringEnabled(&self) -> bool;

        #[method(setInputMeteringEnabled:)]
        pub unsafe fn setInputMeteringEnabled(&self, input_metering_enabled: bool);

        #[method(outputMeterLevel)]
        pub unsafe fn outputMeterLevel(&self) -> c_float;

        #[method(inputMeterLevel)]
        pub unsafe fn inputMeterLevel(&self) -> c_float;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKVoiceChatService {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
