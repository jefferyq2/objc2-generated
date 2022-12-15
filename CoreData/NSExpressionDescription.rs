//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSExpressionDescription;

    unsafe impl ClassType for NSExpressionDescription {
        #[inherits(NSObject)]
        type Super = NSPropertyDescription;
    }
);

extern_methods!(
    unsafe impl NSExpressionDescription {
        #[method_id(@__retain_semantics Other expression)]
        pub unsafe fn expression(&self) -> Option<Id<NSExpression, Shared>>;

        #[method(setExpression:)]
        pub unsafe fn setExpression(&self, expression: Option<&NSExpression>);

        #[method(expressionResultType)]
        pub unsafe fn expressionResultType(&self) -> NSAttributeType;

        #[method(setExpressionResultType:)]
        pub unsafe fn setExpressionResultType(&self, expressionResultType: NSAttributeType);
    }
);
