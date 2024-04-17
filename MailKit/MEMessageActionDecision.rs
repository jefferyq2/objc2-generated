//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEMessageActionDecision;

    unsafe impl ClassType for MEMessageActionDecision {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for MEMessageActionDecision {}

unsafe impl NSObjectProtocol for MEMessageActionDecision {}

unsafe impl NSSecureCoding for MEMessageActionDecision {}

extern_methods!(
    unsafe impl MEMessageActionDecision {
        #[method_id(@__retain_semantics Other invokeAgainWithBody)]
        pub unsafe fn invokeAgainWithBody() -> Id<MEMessageActionDecision>;

        #[cfg(feature = "MEMessageAction")]
        #[method_id(@__retain_semantics Other decisionApplyingAction:)]
        pub unsafe fn decisionApplyingAction(action: &MEMessageAction) -> Id<Self>;

        #[cfg(feature = "MEMessageAction")]
        #[method_id(@__retain_semantics Other decisionApplyingActions:)]
        pub unsafe fn decisionApplyingActions(actions: &NSArray<MEMessageAction>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
