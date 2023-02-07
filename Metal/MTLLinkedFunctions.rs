//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLLinkedFunctions")]
    pub struct MTLLinkedFunctions;

    #[cfg(feature = "Metal_MTLLinkedFunctions")]
    unsafe impl ClassType for MTLLinkedFunctions {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLLinkedFunctions")]
unsafe impl NSObjectProtocol for MTLLinkedFunctions {}

extern_methods!(
    #[cfg(feature = "Metal_MTLLinkedFunctions")]
    unsafe impl MTLLinkedFunctions {
        #[method_id(@__retain_semantics Other linkedFunctions)]
        pub fn linkedFunctions() -> Id<MTLLinkedFunctions>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other functions)]
        pub fn functions(&self) -> Option<Id<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setFunctions:)]
        pub fn setFunctions(&self, functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other binaryFunctions)]
        pub fn binaryFunctions(&self) -> Option<Id<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setBinaryFunctions:)]
        pub fn setBinaryFunctions(
            &self,
            binary_functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other groups)]
        pub fn groups(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSArray<ProtocolObject<dyn MTLFunction>>>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(setGroups:)]
        pub fn setGroups(
            &self,
            groups: Option<&NSDictionary<NSString, NSArray<ProtocolObject<dyn MTLFunction>>>>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other privateFunctions)]
        pub fn privateFunctions(&self) -> Option<Id<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setPrivateFunctions:)]
        pub fn setPrivateFunctions(
            &self,
            private_functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>,
        );
    }
);
