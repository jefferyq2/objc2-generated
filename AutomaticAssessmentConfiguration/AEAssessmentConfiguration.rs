//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AutomaticAssessmentConfiguration::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum AEAutocorrectMode {
        AEAutocorrectModeNone = 0,
        AEAutocorrectModeSpelling = 1 << 0,
        AEAutocorrectModePunctuation = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentConfiguration")]
    pub struct AEAssessmentConfiguration;

    #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentConfiguration")]
    unsafe impl ClassType for AEAssessmentConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentConfiguration")]
unsafe impl NSObjectProtocol for AEAssessmentConfiguration {}

extern_methods!(
    #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentConfiguration")]
    unsafe impl AEAssessmentConfiguration {
        #[method(autocorrectMode)]
        pub unsafe fn autocorrectMode(&self) -> AEAutocorrectMode;

        #[method(setAutocorrectMode:)]
        pub unsafe fn setAutocorrectMode(&self, autocorrect_mode: AEAutocorrectMode);

        #[method(allowsSpellCheck)]
        pub unsafe fn allowsSpellCheck(&self) -> bool;

        #[method(setAllowsSpellCheck:)]
        pub unsafe fn setAllowsSpellCheck(&self, allows_spell_check: bool);

        #[method(allowsPredictiveKeyboard)]
        pub unsafe fn allowsPredictiveKeyboard(&self) -> bool;

        #[method(setAllowsPredictiveKeyboard:)]
        pub unsafe fn setAllowsPredictiveKeyboard(&self, allows_predictive_keyboard: bool);

        #[method(allowsKeyboardShortcuts)]
        pub unsafe fn allowsKeyboardShortcuts(&self) -> bool;

        #[method(setAllowsKeyboardShortcuts:)]
        pub unsafe fn setAllowsKeyboardShortcuts(&self, allows_keyboard_shortcuts: bool);

        #[method(allowsActivityContinuation)]
        pub unsafe fn allowsActivityContinuation(&self) -> bool;

        #[method(setAllowsActivityContinuation:)]
        pub unsafe fn setAllowsActivityContinuation(&self, allows_activity_continuation: bool);

        #[method(allowsDictation)]
        pub unsafe fn allowsDictation(&self) -> bool;

        #[method(setAllowsDictation:)]
        pub unsafe fn setAllowsDictation(&self, allows_dictation: bool);

        #[method(allowsAccessibilitySpeech)]
        pub unsafe fn allowsAccessibilitySpeech(&self) -> bool;

        #[method(setAllowsAccessibilitySpeech:)]
        pub unsafe fn setAllowsAccessibilitySpeech(&self, allows_accessibility_speech: bool);

        #[method(allowsPasswordAutoFill)]
        pub unsafe fn allowsPasswordAutoFill(&self) -> bool;

        #[method(setAllowsPasswordAutoFill:)]
        pub unsafe fn setAllowsPasswordAutoFill(&self, allows_password_auto_fill: bool);

        #[method(allowsContinuousPathKeyboard)]
        pub unsafe fn allowsContinuousPathKeyboard(&self) -> bool;

        #[method(setAllowsContinuousPathKeyboard:)]
        pub unsafe fn setAllowsContinuousPathKeyboard(&self, allows_continuous_path_keyboard: bool);

        #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentParticipantConfiguration")]
        #[method_id(@__retain_semantics Other mainParticipantConfiguration)]
        pub unsafe fn mainParticipantConfiguration(
            &self,
        ) -> Id<AEAssessmentParticipantConfiguration>;

        #[cfg(all(
            feature = "AutomaticAssessmentConfiguration_AEAssessmentApplication",
            feature = "AutomaticAssessmentConfiguration_AEAssessmentParticipantConfiguration",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other configurationsByApplication)]
        pub unsafe fn configurationsByApplication(
            &self,
        ) -> Id<NSDictionary<AEAssessmentApplication, AEAssessmentParticipantConfiguration>>;

        #[cfg(all(
            feature = "AutomaticAssessmentConfiguration_AEAssessmentApplication",
            feature = "AutomaticAssessmentConfiguration_AEAssessmentParticipantConfiguration"
        ))]
        #[method(setConfiguration:forApplication:)]
        pub unsafe fn setConfiguration_forApplication(
            &self,
            configuration: &AEAssessmentParticipantConfiguration,
            application: &AEAssessmentApplication,
        );

        #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentApplication")]
        #[method(removeApplication:)]
        pub unsafe fn removeApplication(&self, application: &AEAssessmentApplication);
    }
);
