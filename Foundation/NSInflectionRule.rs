//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSInflectionRule;

    unsafe impl ClassType for NSInflectionRule {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSInflectionRule {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other automaticRule)]
        pub unsafe fn automaticRule() -> Id<NSInflectionRule, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSInflectionRuleExplicit;

    unsafe impl ClassType for NSInflectionRuleExplicit {
        #[inherits(NSObject)]
        type Super = NSInflectionRule;
    }
);

extern_methods!(
    unsafe impl NSInflectionRuleExplicit {
        #[method_id(@__retain_semantics Init initWithMorphology:)]
        pub unsafe fn initWithMorphology(
            this: Option<Allocated<Self>>,
            morphology: &NSMorphology,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other morphology)]
        pub unsafe fn morphology(&self) -> Id<NSMorphology, Shared>;
    }
);

extern_methods!(
    /// NSInflectionAvailability
    unsafe impl NSInflectionRule {
        #[method(canInflectLanguage:)]
        pub unsafe fn canInflectLanguage(language: &NSString) -> bool;

        #[method(canInflectPreferredLocalization)]
        pub unsafe fn canInflectPreferredLocalization() -> bool;
    }
);
