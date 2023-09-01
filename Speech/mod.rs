// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `Speech` framework

#[cfg_attr(feature = "apple", link(name = "Speech", kind = "framework"))]
extern "C" {}

#[path = "SFSpeechRecognitionMetadata.rs"]
mod __SFSpeechRecognitionMetadata;
#[path = "SFSpeechRecognitionRequest.rs"]
mod __SFSpeechRecognitionRequest;
#[path = "SFSpeechRecognitionResult.rs"]
mod __SFSpeechRecognitionResult;
#[path = "SFSpeechRecognitionTask.rs"]
mod __SFSpeechRecognitionTask;
#[path = "SFSpeechRecognitionTaskHint.rs"]
mod __SFSpeechRecognitionTaskHint;
#[path = "SFSpeechRecognizer.rs"]
mod __SFSpeechRecognizer;
#[path = "SFTranscription.rs"]
mod __SFTranscription;
#[path = "SFTranscriptionSegment.rs"]
mod __SFTranscriptionSegment;
#[path = "SFVoiceAnalytics.rs"]
mod __SFVoiceAnalytics;

#[cfg(feature = "Speech_SFSpeechRecognitionMetadata")]
pub use self::__SFSpeechRecognitionMetadata::SFSpeechRecognitionMetadata;
#[cfg(feature = "Speech_SFSpeechAudioBufferRecognitionRequest")]
pub use self::__SFSpeechRecognitionRequest::SFSpeechAudioBufferRecognitionRequest;
#[cfg(feature = "Speech_SFSpeechRecognitionRequest")]
pub use self::__SFSpeechRecognitionRequest::SFSpeechRecognitionRequest;
#[cfg(feature = "Speech_SFSpeechURLRecognitionRequest")]
pub use self::__SFSpeechRecognitionRequest::SFSpeechURLRecognitionRequest;
#[cfg(feature = "Speech_SFSpeechRecognitionResult")]
pub use self::__SFSpeechRecognitionResult::SFSpeechRecognitionResult;
#[cfg(feature = "Speech_SFSpeechRecognitionTask")]
pub use self::__SFSpeechRecognitionTask::SFSpeechRecognitionTask;
pub use self::__SFSpeechRecognitionTask::SFSpeechRecognitionTaskDelegate;
pub use self::__SFSpeechRecognitionTask::{
    SFSpeechRecognitionTaskState, SFSpeechRecognitionTaskStateCanceling,
    SFSpeechRecognitionTaskStateCompleted, SFSpeechRecognitionTaskStateFinishing,
    SFSpeechRecognitionTaskStateRunning, SFSpeechRecognitionTaskStateStarting,
};
pub use self::__SFSpeechRecognitionTaskHint::{
    SFSpeechRecognitionTaskHint, SFSpeechRecognitionTaskHintConfirmation,
    SFSpeechRecognitionTaskHintDictation, SFSpeechRecognitionTaskHintSearch,
    SFSpeechRecognitionTaskHintUnspecified,
};
#[cfg(feature = "Speech_SFSpeechRecognizer")]
pub use self::__SFSpeechRecognizer::SFSpeechRecognizer;
pub use self::__SFSpeechRecognizer::SFSpeechRecognizerDelegate;
pub use self::__SFSpeechRecognizer::{
    SFSpeechRecognizerAuthorizationStatus, SFSpeechRecognizerAuthorizationStatusAuthorized,
    SFSpeechRecognizerAuthorizationStatusDenied,
    SFSpeechRecognizerAuthorizationStatusNotDetermined,
    SFSpeechRecognizerAuthorizationStatusRestricted,
};
#[cfg(feature = "Speech_SFTranscription")]
pub use self::__SFTranscription::SFTranscription;
#[cfg(feature = "Speech_SFTranscriptionSegment")]
pub use self::__SFTranscriptionSegment::SFTranscriptionSegment;
#[cfg(feature = "Speech_SFAcousticFeature")]
pub use self::__SFVoiceAnalytics::SFAcousticFeature;
#[cfg(feature = "Speech_SFVoiceAnalytics")]
pub use self::__SFVoiceAnalytics::SFVoiceAnalytics;
