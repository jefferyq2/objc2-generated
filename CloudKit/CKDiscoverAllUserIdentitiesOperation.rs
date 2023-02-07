//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKDiscoverAllUserIdentitiesOperation")]
    pub struct CKDiscoverAllUserIdentitiesOperation;

    #[cfg(feature = "CloudKit_CKDiscoverAllUserIdentitiesOperation")]
    unsafe impl ClassType for CKDiscoverAllUserIdentitiesOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
    }
);

#[cfg(feature = "CloudKit_CKDiscoverAllUserIdentitiesOperation")]
unsafe impl NSObjectProtocol for CKDiscoverAllUserIdentitiesOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKDiscoverAllUserIdentitiesOperation")]
    unsafe impl CKDiscoverAllUserIdentitiesOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKUserIdentity")]
        #[method(userIdentityDiscoveredBlock)]
        pub unsafe fn userIdentityDiscoveredBlock(
            &self,
        ) -> *mut Block<(NonNull<CKUserIdentity>,), ()>;

        #[cfg(feature = "CloudKit_CKUserIdentity")]
        #[method(setUserIdentityDiscoveredBlock:)]
        pub unsafe fn setUserIdentityDiscoveredBlock(
            &self,
            user_identity_discovered_block: Option<&Block<(NonNull<CKUserIdentity>,), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(discoverAllUserIdentitiesCompletionBlock)]
        pub unsafe fn discoverAllUserIdentitiesCompletionBlock(
            &self,
        ) -> *mut Block<(*mut NSError,), ()>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(setDiscoverAllUserIdentitiesCompletionBlock:)]
        pub unsafe fn setDiscoverAllUserIdentitiesCompletionBlock(
            &self,
            discover_all_user_identities_completion_block: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);
