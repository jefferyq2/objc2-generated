//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSFormattingContext {
        NSFormattingContextUnknown = 0,
        NSFormattingContextDynamic = 1,
        NSFormattingContextStandalone = 2,
        NSFormattingContextListItem = 3,
        NSFormattingContextBeginningOfSentence = 4,
        NSFormattingContextMiddleOfSentence = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSFormattingUnitStyle {
        NSFormattingUnitStyleShort = 1,
        NSFormattingUnitStyleMedium = 2,
        NSFormattingUnitStyleLong = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSFormatter")]
    pub struct NSFormatter;

    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl ClassType for NSFormatter {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSFormatter")]
unsafe impl NSCoding for NSFormatter {}

#[cfg(feature = "Foundation_NSFormatter")]
unsafe impl NSObjectProtocol for NSFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSFormatter")]
    unsafe impl NSFormatter {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(&self, obj: Option<&Object>) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other attributedStringForObjectValue:withDefaultAttributes:)]
        pub unsafe fn attributedStringForObjectValue_withDefaultAttributes(
            &self,
            obj: &Object,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        ) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other editingStringForObjectValue:)]
        pub unsafe fn editingStringForObjectValue(&self, obj: &Object) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<Object>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString>>>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(isPartialStringValid:newEditingString:errorDescription:)]
        pub unsafe fn isPartialStringValid_newEditingString_errorDescription(
            &self,
            partial_string: &NSString,
            new_string: Option<&mut Option<Id<NSString>>>,
            error: Option<&mut Option<Id<NSString>>>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(isPartialStringValid:proposedSelectedRange:originalString:originalSelectedRange:errorDescription:)]
        pub unsafe fn isPartialStringValid_proposedSelectedRange_originalString_originalSelectedRange_errorDescription(
            &self,
            partial_string_ptr: &mut Id<NSString>,
            proposed_sel_range_ptr: NSRangePointer,
            orig_string: &NSString,
            orig_sel_range: NSRange,
            error: Option<&mut Option<Id<NSString>>>,
        ) -> bool;
    }
);
