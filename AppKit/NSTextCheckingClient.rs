//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextInputTraitType(pub NSInteger);
impl NSTextInputTraitType {
    #[doc(alias = "NSTextInputTraitTypeDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSTextInputTraitTypeNo")]
    pub const No: Self = Self(1);
    #[doc(alias = "NSTextInputTraitTypeYes")]
    pub const Yes: Self = Self(2);
}

unsafe impl Encode for NSTextInputTraitType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSTextInputTraitType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait NSTextInputTraits {
        #[optional]
        #[method(autocorrectionType)]
        unsafe fn autocorrectionType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setAutocorrectionType:)]
        unsafe fn setAutocorrectionType(&self, autocorrection_type: NSTextInputTraitType);

        #[optional]
        #[method(spellCheckingType)]
        unsafe fn spellCheckingType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setSpellCheckingType:)]
        unsafe fn setSpellCheckingType(&self, spell_checking_type: NSTextInputTraitType);

        #[optional]
        #[method(grammarCheckingType)]
        unsafe fn grammarCheckingType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setGrammarCheckingType:)]
        unsafe fn setGrammarCheckingType(&self, grammar_checking_type: NSTextInputTraitType);

        #[optional]
        #[method(smartQuotesType)]
        unsafe fn smartQuotesType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setSmartQuotesType:)]
        unsafe fn setSmartQuotesType(&self, smart_quotes_type: NSTextInputTraitType);

        #[optional]
        #[method(smartDashesType)]
        unsafe fn smartDashesType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setSmartDashesType:)]
        unsafe fn setSmartDashesType(&self, smart_dashes_type: NSTextInputTraitType);

        #[optional]
        #[method(smartInsertDeleteType)]
        unsafe fn smartInsertDeleteType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setSmartInsertDeleteType:)]
        unsafe fn setSmartInsertDeleteType(&self, smart_insert_delete_type: NSTextInputTraitType);

        #[optional]
        #[method(textReplacementType)]
        unsafe fn textReplacementType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setTextReplacementType:)]
        unsafe fn setTextReplacementType(&self, text_replacement_type: NSTextInputTraitType);

        #[optional]
        #[method(dataDetectionType)]
        unsafe fn dataDetectionType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setDataDetectionType:)]
        unsafe fn setDataDetectionType(&self, data_detection_type: NSTextInputTraitType);

        #[optional]
        #[method(linkDetectionType)]
        unsafe fn linkDetectionType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setLinkDetectionType:)]
        unsafe fn setLinkDetectionType(&self, link_detection_type: NSTextInputTraitType);

        #[optional]
        #[method(textCompletionType)]
        unsafe fn textCompletionType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setTextCompletionType:)]
        unsafe fn setTextCompletionType(&self, text_completion_type: NSTextInputTraitType);

        #[optional]
        #[method(inlinePredictionType)]
        unsafe fn inlinePredictionType(&self) -> NSTextInputTraitType;

        #[optional]
        #[method(setInlinePredictionType:)]
        unsafe fn setInlinePredictionType(&self, inline_prediction_type: NSTextInputTraitType);
    }

    unsafe impl ProtocolType for dyn NSTextInputTraits {}
);

extern_protocol!(
    #[cfg(feature = "NSTextInputClient")]
    pub unsafe trait NSTextCheckingClient: NSTextInputClient + NSTextInputTraits {
        #[method_id(@__retain_semantics Other annotatedSubstringForProposedRange:actualRange:)]
        unsafe fn annotatedSubstringForProposedRange_actualRange(
            &self,
            range: NSRange,
            actual_range: NSRangePointer,
        ) -> Option<Id<NSAttributedString>>;

        #[method(setAnnotations:range:)]
        unsafe fn setAnnotations_range(
            &self,
            annotations: &NSDictionary<NSAttributedStringKey, NSString>,
            range: NSRange,
        );

        #[method(addAnnotations:range:)]
        unsafe fn addAnnotations_range(
            &self,
            annotations: &NSDictionary<NSAttributedStringKey, NSString>,
            range: NSRange,
        );

        #[method(removeAnnotation:range:)]
        unsafe fn removeAnnotation_range(
            &self,
            annotation_name: &NSAttributedStringKey,
            range: NSRange,
        );

        #[method(replaceCharactersInRange:withAnnotatedString:)]
        unsafe fn replaceCharactersInRange_withAnnotatedString(
            &self,
            range: NSRange,
            annotated_string: &NSAttributedString,
        );

        #[method(selectAndShowRange:)]
        unsafe fn selectAndShowRange(&self, range: NSRange);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other viewForRange:firstRect:actualRange:)]
        unsafe fn viewForRange_firstRect_actualRange(
            &self,
            range: NSRange,
            first_rect: NSRectPointer,
            actual_range: NSRangePointer,
            mtm: MainThreadMarker,
        ) -> Option<Id<NSView>>;

        #[cfg(all(feature = "NSCandidateListTouchBarItem", feature = "NSTouchBarItem"))]
        #[method_id(@__retain_semantics Other candidateListTouchBarItem)]
        unsafe fn candidateListTouchBarItem(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Id<NSCandidateListTouchBarItem>>;
    }

    #[cfg(feature = "NSTextInputClient")]
    unsafe impl ProtocolType for dyn NSTextCheckingClient {}
);
