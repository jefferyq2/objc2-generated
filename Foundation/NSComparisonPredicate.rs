//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSComparisonPredicateOptions {
        NSCaseInsensitivePredicateOption = 0x01,
        NSDiacriticInsensitivePredicateOption = 0x02,
        NSNormalizedPredicateOption = 0x04,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSComparisonPredicateModifier {
        NSDirectPredicateModifier = 0,
        NSAllPredicateModifier = 1,
        NSAnyPredicateModifier = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSPredicateOperatorType {
        NSLessThanPredicateOperatorType = 0,
        NSLessThanOrEqualToPredicateOperatorType = 1,
        NSGreaterThanPredicateOperatorType = 2,
        NSGreaterThanOrEqualToPredicateOperatorType = 3,
        NSEqualToPredicateOperatorType = 4,
        NSNotEqualToPredicateOperatorType = 5,
        NSMatchesPredicateOperatorType = 6,
        NSLikePredicateOperatorType = 7,
        NSBeginsWithPredicateOperatorType = 8,
        NSEndsWithPredicateOperatorType = 9,
        NSInPredicateOperatorType = 10,
        NSCustomSelectorPredicateOperatorType = 11,
        NSContainsPredicateOperatorType = 99,
        NSBetweenPredicateOperatorType = 100,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSComparisonPredicate")]
    pub struct NSComparisonPredicate;

    #[cfg(feature = "Foundation_NSComparisonPredicate")]
    unsafe impl ClassType for NSComparisonPredicate {
        #[inherits(NSObject)]
        type Super = NSPredicate;
    }
);

#[cfg(feature = "Foundation_NSComparisonPredicate")]
unsafe impl NSCoding for NSComparisonPredicate {}

#[cfg(feature = "Foundation_NSComparisonPredicate")]
unsafe impl NSObjectProtocol for NSComparisonPredicate {}

#[cfg(feature = "Foundation_NSComparisonPredicate")]
unsafe impl NSSecureCoding for NSComparisonPredicate {}

extern_methods!(
    #[cfg(feature = "Foundation_NSComparisonPredicate")]
    unsafe impl NSComparisonPredicate {
        #[cfg(feature = "Foundation_NSExpression")]
        #[method_id(@__retain_semantics Other predicateWithLeftExpression:rightExpression:modifier:type:options:)]
        pub unsafe fn predicateWithLeftExpression_rightExpression_modifier_type_options(
            lhs: &NSExpression,
            rhs: &NSExpression,
            modifier: NSComparisonPredicateModifier,
            r#type: NSPredicateOperatorType,
            options: NSComparisonPredicateOptions,
        ) -> Id<NSComparisonPredicate>;

        #[cfg(feature = "Foundation_NSExpression")]
        #[method_id(@__retain_semantics Other predicateWithLeftExpression:rightExpression:customSelector:)]
        pub unsafe fn predicateWithLeftExpression_rightExpression_customSelector(
            lhs: &NSExpression,
            rhs: &NSExpression,
            selector: Sel,
        ) -> Id<NSComparisonPredicate>;

        #[cfg(feature = "Foundation_NSExpression")]
        #[method_id(@__retain_semantics Init initWithLeftExpression:rightExpression:modifier:type:options:)]
        pub unsafe fn initWithLeftExpression_rightExpression_modifier_type_options(
            this: Option<Allocated<Self>>,
            lhs: &NSExpression,
            rhs: &NSExpression,
            modifier: NSComparisonPredicateModifier,
            r#type: NSPredicateOperatorType,
            options: NSComparisonPredicateOptions,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSExpression")]
        #[method_id(@__retain_semantics Init initWithLeftExpression:rightExpression:customSelector:)]
        pub unsafe fn initWithLeftExpression_rightExpression_customSelector(
            this: Option<Allocated<Self>>,
            lhs: &NSExpression,
            rhs: &NSExpression,
            selector: Sel,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method(predicateOperatorType)]
        pub unsafe fn predicateOperatorType(&self) -> NSPredicateOperatorType;

        #[method(comparisonPredicateModifier)]
        pub unsafe fn comparisonPredicateModifier(&self) -> NSComparisonPredicateModifier;

        #[cfg(feature = "Foundation_NSExpression")]
        #[method_id(@__retain_semantics Other leftExpression)]
        pub unsafe fn leftExpression(&self) -> Id<NSExpression>;

        #[cfg(feature = "Foundation_NSExpression")]
        #[method_id(@__retain_semantics Other rightExpression)]
        pub unsafe fn rightExpression(&self) -> Id<NSExpression>;

        #[method(customSelector)]
        pub unsafe fn customSelector(&self) -> Option<Sel>;

        #[method(options)]
        pub unsafe fn options(&self) -> NSComparisonPredicateOptions;
    }
);
