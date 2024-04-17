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
pub struct SFSpeechRecognizerAuthorizationStatus(pub NSInteger);
impl SFSpeechRecognizerAuthorizationStatus {
    #[doc(alias = "SFSpeechRecognizerAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "SFSpeechRecognizerAuthorizationStatusDenied")]
    pub const Denied: Self = Self(1);
    #[doc(alias = "SFSpeechRecognizerAuthorizationStatusRestricted")]
    pub const Restricted: Self = Self(2);
    #[doc(alias = "SFSpeechRecognizerAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(3);
}

unsafe impl Encode for SFSpeechRecognizerAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SFSpeechRecognizerAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFSpeechRecognizer;

    unsafe impl ClassType for SFSpeechRecognizer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for SFSpeechRecognizer {}

extern_methods!(
    unsafe impl SFSpeechRecognizer {
        #[method_id(@__retain_semantics Other supportedLocales)]
        pub unsafe fn supportedLocales() -> Id<NSSet<NSLocale>>;

        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> SFSpeechRecognizerAuthorizationStatus;

        #[cfg(feature = "block2")]
        #[method(requestAuthorization:)]
        pub unsafe fn requestAuthorization(
            handler: &Block<dyn Fn(SFSpeechRecognizerAuthorizationStatus)>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithLocale:)]
        pub unsafe fn initWithLocale(this: Allocated<Self>, locale: &NSLocale) -> Option<Id<Self>>;

        #[method(isAvailable)]
        pub unsafe fn isAvailable(&self) -> bool;

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

        #[cfg(feature = "SFSpeechRecognitionTaskHint")]
        #[method(defaultTaskHint)]
        pub unsafe fn defaultTaskHint(&self) -> SFSpeechRecognitionTaskHint;

        #[cfg(feature = "SFSpeechRecognitionTaskHint")]
        #[method(setDefaultTaskHint:)]
        pub unsafe fn setDefaultTaskHint(&self, default_task_hint: SFSpeechRecognitionTaskHint);

        #[cfg(all(
            feature = "SFSpeechRecognitionRequest",
            feature = "SFSpeechRecognitionResult",
            feature = "SFSpeechRecognitionTask",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other recognitionTaskWithRequest:resultHandler:)]
        pub unsafe fn recognitionTaskWithRequest_resultHandler(
            &self,
            request: &SFSpeechRecognitionRequest,
            result_handler: &Block<dyn Fn(*mut SFSpeechRecognitionResult, *mut NSError)>,
        ) -> Id<SFSpeechRecognitionTask>;

        #[cfg(all(
            feature = "SFSpeechRecognitionRequest",
            feature = "SFSpeechRecognitionTask"
        ))]
        #[method_id(@__retain_semantics Other recognitionTaskWithRequest:delegate:)]
        pub unsafe fn recognitionTaskWithRequest_delegate(
            &self,
            request: &SFSpeechRecognitionRequest,
            delegate: &ProtocolObject<dyn SFSpeechRecognitionTaskDelegate>,
        ) -> Id<SFSpeechRecognitionTask>;

        #[method_id(@__retain_semantics Other queue)]
        pub unsafe fn queue(&self) -> Id<NSOperationQueue>;

        #[method(setQueue:)]
        pub unsafe fn setQueue(&self, queue: &NSOperationQueue);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SFSpeechRecognizer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait SFSpeechRecognizerDelegate: NSObjectProtocol {
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
