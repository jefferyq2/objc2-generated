//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSSaveOptions {
        NSSaveOptionsYes = 0,
        NSSaveOptionsNo = 1,
        NSSaveOptionsAsk = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCloneCommand;

    unsafe impl ClassType for NSCloneCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
    }
);

extern_methods!(
    unsafe impl NSCloneCommand {
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receiversRef: Option<&NSScriptObjectSpecifier>);

        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCloseCommand;

    unsafe impl ClassType for NSCloseCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
    }
);

extern_methods!(
    unsafe impl NSCloseCommand {
        #[method(saveOptions)]
        pub unsafe fn saveOptions(&self) -> NSSaveOptions;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCountCommand;

    unsafe impl ClassType for NSCountCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
    }
);

extern_methods!(
    unsafe impl NSCountCommand {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCreateCommand;

    unsafe impl ClassType for NSCreateCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
    }
);

extern_methods!(
    unsafe impl NSCreateCommand {
        #[method_id(@__retain_semantics Other createClassDescription)]
        pub unsafe fn createClassDescription(&self) -> Id<NSScriptClassDescription, Shared>;

        #[method_id(@__retain_semantics Other resolvedKeyDictionary)]
        pub unsafe fn resolvedKeyDictionary(&self) -> Id<NSDictionary<NSString, Object>, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDeleteCommand;

    unsafe impl ClassType for NSDeleteCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
    }
);

extern_methods!(
    unsafe impl NSDeleteCommand {
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receiversRef: Option<&NSScriptObjectSpecifier>);

        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSExistsCommand;

    unsafe impl ClassType for NSExistsCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
    }
);

extern_methods!(
    unsafe impl NSExistsCommand {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSGetCommand;

    unsafe impl ClassType for NSGetCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
    }
);

extern_methods!(
    unsafe impl NSGetCommand {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMoveCommand;

    unsafe impl ClassType for NSMoveCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
    }
);

extern_methods!(
    unsafe impl NSMoveCommand {
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receiversRef: Option<&NSScriptObjectSpecifier>);

        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSQuitCommand;

    unsafe impl ClassType for NSQuitCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
    }
);

extern_methods!(
    unsafe impl NSQuitCommand {
        #[method(saveOptions)]
        pub unsafe fn saveOptions(&self) -> NSSaveOptions;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSetCommand;

    unsafe impl ClassType for NSSetCommand {
        #[inherits(NSObject)]
        type Super = NSScriptCommand;
    }
);

extern_methods!(
    unsafe impl NSSetCommand {
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(&self, receiversRef: Option<&NSScriptObjectSpecifier>);

        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared>;
    }
);
