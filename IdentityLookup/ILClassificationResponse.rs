//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ILClassificationResponse;

    unsafe impl ClassType for ILClassificationResponse {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for ILClassificationResponse {}

unsafe impl NSObjectProtocol for ILClassificationResponse {}

unsafe impl NSSecureCoding for ILClassificationResponse {}

extern_methods!(
    unsafe impl ILClassificationResponse {
        #[cfg(feature = "ILClassificationActions")]
        #[method(action)]
        pub unsafe fn action(&self) -> ILClassificationAction;

        #[method_id(@__retain_semantics Other userString)]
        pub unsafe fn userString(&self) -> Option<Id<NSString>>;

        #[method(setUserString:)]
        pub unsafe fn setUserString(&self, user_string: Option<&NSString>);

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary<NSString, AnyObject>>);

        #[cfg(feature = "ILClassificationActions")]
        #[method_id(@__retain_semantics Init initWithClassificationAction:)]
        pub unsafe fn initWithClassificationAction(
            this: Allocated<Self>,
            action: ILClassificationAction,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ILClassificationResponse {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
