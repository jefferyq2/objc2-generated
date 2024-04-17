//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub type CKOperationID = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKOperation;

    unsafe impl ClassType for CKOperation {
        #[inherits(NSObject)]
        type Super = NSOperation;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CKOperation {}

extern_methods!(
    unsafe impl CKOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Id<CKOperationConfiguration>;

        #[method(setConfiguration:)]
        pub unsafe fn setConfiguration(&self, configuration: Option<&CKOperationConfiguration>);

        #[cfg(feature = "CKOperationGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Option<Id<CKOperationGroup>>;

        #[cfg(feature = "CKOperationGroup")]
        #[method(setGroup:)]
        pub unsafe fn setGroup(&self, group: Option<&CKOperationGroup>);

        #[method_id(@__retain_semantics Other operationID)]
        pub unsafe fn operationID(&self) -> Id<CKOperationID>;

        #[cfg(feature = "block2")]
        #[method(longLivedOperationWasPersistedBlock)]
        pub unsafe fn longLivedOperationWasPersistedBlock(&self) -> *mut Block<dyn Fn()>;

        #[cfg(feature = "block2")]
        #[method(setLongLivedOperationWasPersistedBlock:)]
        pub unsafe fn setLongLivedOperationWasPersistedBlock(
            &self,
            long_lived_operation_was_persisted_block: Option<&Block<dyn Fn()>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CKOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKOperationConfiguration;

    unsafe impl ClassType for CKOperationConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CKOperationConfiguration {}

extern_methods!(
    unsafe impl CKOperationConfiguration {
        #[cfg(feature = "CKContainer")]
        #[method_id(@__retain_semantics Other container)]
        pub unsafe fn container(&self) -> Option<Id<CKContainer>>;

        #[cfg(feature = "CKContainer")]
        #[method(setContainer:)]
        pub unsafe fn setContainer(&self, container: Option<&CKContainer>);

        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;

        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, quality_of_service: NSQualityOfService);

        #[method(allowsCellularAccess)]
        pub unsafe fn allowsCellularAccess(&self) -> bool;

        #[method(setAllowsCellularAccess:)]
        pub unsafe fn setAllowsCellularAccess(&self, allows_cellular_access: bool);

        #[method(isLongLived)]
        pub unsafe fn isLongLived(&self) -> bool;

        #[method(setLongLived:)]
        pub unsafe fn setLongLived(&self, long_lived: bool);

        #[method(timeoutIntervalForRequest)]
        pub unsafe fn timeoutIntervalForRequest(&self) -> NSTimeInterval;

        #[method(setTimeoutIntervalForRequest:)]
        pub unsafe fn setTimeoutIntervalForRequest(
            &self,
            timeout_interval_for_request: NSTimeInterval,
        );

        #[method(timeoutIntervalForResource)]
        pub unsafe fn timeoutIntervalForResource(&self) -> NSTimeInterval;

        #[method(setTimeoutIntervalForResource:)]
        pub unsafe fn setTimeoutIntervalForResource(
            &self,
            timeout_interval_for_resource: NSTimeInterval,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CKOperationConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// CKOperationDeprecated
    unsafe impl CKOperation {
        #[cfg(feature = "CKContainer")]
        #[deprecated = "Use CKOperationConfiguration"]
        #[method_id(@__retain_semantics Other container)]
        pub unsafe fn container(&self) -> Option<Id<CKContainer>>;

        #[cfg(feature = "CKContainer")]
        #[deprecated = "Use CKOperationConfiguration"]
        #[method(setContainer:)]
        pub unsafe fn setContainer(&self, container: Option<&CKContainer>);

        #[deprecated = "Use CKOperationConfiguration"]
        #[method(allowsCellularAccess)]
        pub unsafe fn allowsCellularAccess(&self) -> bool;

        #[deprecated = "Use CKOperationConfiguration"]
        #[method(setAllowsCellularAccess:)]
        pub unsafe fn setAllowsCellularAccess(&self, allows_cellular_access: bool);

        #[deprecated = "Use CKOperationConfiguration"]
        #[method(isLongLived)]
        pub unsafe fn isLongLived(&self) -> bool;

        #[deprecated = "Use CKOperationConfiguration"]
        #[method(setLongLived:)]
        pub unsafe fn setLongLived(&self, long_lived: bool);

        #[deprecated = "Use CKOperationConfiguration"]
        #[method(timeoutIntervalForRequest)]
        pub unsafe fn timeoutIntervalForRequest(&self) -> NSTimeInterval;

        #[deprecated = "Use CKOperationConfiguration"]
        #[method(setTimeoutIntervalForRequest:)]
        pub unsafe fn setTimeoutIntervalForRequest(
            &self,
            timeout_interval_for_request: NSTimeInterval,
        );

        #[deprecated = "Use CKOperationConfiguration"]
        #[method(timeoutIntervalForResource)]
        pub unsafe fn timeoutIntervalForResource(&self) -> NSTimeInterval;

        #[deprecated = "Use CKOperationConfiguration"]
        #[method(setTimeoutIntervalForResource:)]
        pub unsafe fn setTimeoutIntervalForResource(
            &self,
            timeout_interval_for_resource: NSTimeInterval,
        );
    }
);
