//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Speech::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum SFSpeechRecognizerAuthorizationStatus {
        SFSpeechRecognizerAuthorizationStatusNotDetermined = 0,
        SFSpeechRecognizerAuthorizationStatusDenied = 1,
        SFSpeechRecognizerAuthorizationStatusRestricted = 2,
        SFSpeechRecognizerAuthorizationStatusAuthorized = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech_SFSpeechRecognizer")]
    pub struct SFSpeechRecognizer;

    #[cfg(feature = "Speech_SFSpeechRecognizer")]
    unsafe impl ClassType for SFSpeechRecognizer {
        type Super = NSObject;
    }
);

#[cfg(feature = "Speech_SFSpeechRecognizer")]
unsafe impl NSObjectProtocol for SFSpeechRecognizer {}

extern_methods!(
    #[cfg(feature = "Speech_SFSpeechRecognizer")]
    unsafe impl SFSpeechRecognizer {
        #[cfg(all(feature = "Foundation_NSLocale", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other supportedLocales)]
        pub unsafe fn supportedLocales() -> Id<NSSet<NSLocale>>;

        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> SFSpeechRecognizerAuthorizationStatus;

        #[method(requestAuthorization:)]
        pub unsafe fn requestAuthorization(
            handler: &Block<(SFSpeechRecognizerAuthorizationStatus,), ()>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Init initWithLocale:)]
        pub unsafe fn initWithLocale(
            this: Option<Allocated<Self>>,
            locale: &NSLocale,
        ) -> Option<Id<Self>>;

        #[method(isAvailable)]
        pub unsafe fn isAvailable(&self) -> bool;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[method(supportsOnDeviceRecognition)]
        pub unsafe fn supportsOnDeviceRecognition(&self) -> bool;

        #[method(setSupportsOnDeviceRecognition:)]
        pub unsafe fn setSupportsOnDeviceRecognition(&self, supports_on_device_recognition: bool);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Id<ProtocolObject<dyn SFSpeechRecognizerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn SFSpeechRecognizerDelegate>>,
        );

        #[method(defaultTaskHint)]
        pub unsafe fn defaultTaskHint(&self) -> SFSpeechRecognitionTaskHint;

        #[method(setDefaultTaskHint:)]
        pub unsafe fn setDefaultTaskHint(&self, default_task_hint: SFSpeechRecognitionTaskHint);

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Speech_SFSpeechRecognitionRequest",
            feature = "Speech_SFSpeechRecognitionResult",
            feature = "Speech_SFSpeechRecognitionTask"
        ))]
        #[method_id(@__retain_semantics Other recognitionTaskWithRequest:resultHandler:)]
        pub unsafe fn recognitionTaskWithRequest_resultHandler(
            &self,
            request: &SFSpeechRecognitionRequest,
            result_handler: &Block<(*mut SFSpeechRecognitionResult, *mut NSError), ()>,
        ) -> Id<SFSpeechRecognitionTask>;

        #[cfg(all(
            feature = "Speech_SFSpeechRecognitionRequest",
            feature = "Speech_SFSpeechRecognitionTask"
        ))]
        #[method_id(@__retain_semantics Other recognitionTaskWithRequest:delegate:)]
        pub unsafe fn recognitionTaskWithRequest_delegate(
            &self,
            request: &SFSpeechRecognitionRequest,
            delegate: &ProtocolObject<dyn SFSpeechRecognitionTaskDelegate>,
        ) -> Id<SFSpeechRecognitionTask>;

        #[cfg(feature = "Foundation_NSOperationQueue")]
        #[method_id(@__retain_semantics Other queue)]
        pub unsafe fn queue(&self) -> Id<NSOperationQueue>;

        #[cfg(feature = "Foundation_NSOperationQueue")]
        #[method(setQueue:)]
        pub unsafe fn setQueue(&self, queue: &NSOperationQueue);
    }
);

extern_protocol!(
    pub unsafe trait SFSpeechRecognizerDelegate: NSObjectProtocol {
        #[cfg(feature = "Speech_SFSpeechRecognizer")]
        #[optional]
        #[method(speechRecognizer:availabilityDidChange:)]
        unsafe fn speechRecognizer_availabilityDidChange(
            &self,
            speech_recognizer: &SFSpeechRecognizer,
            available: bool,
        );
    }

    unsafe impl ProtocolType for dyn SFSpeechRecognizerDelegate {}
);
