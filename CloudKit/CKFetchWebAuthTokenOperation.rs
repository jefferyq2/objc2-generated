//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchWebAuthTokenOperation")]
    pub struct CKFetchWebAuthTokenOperation;

    #[cfg(feature = "CloudKit_CKFetchWebAuthTokenOperation")]
    unsafe impl ClassType for CKFetchWebAuthTokenOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
    }
);

#[cfg(feature = "CloudKit_CKFetchWebAuthTokenOperation")]
unsafe impl NSObjectProtocol for CKFetchWebAuthTokenOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchWebAuthTokenOperation")]
    unsafe impl CKFetchWebAuthTokenOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithAPIToken:)]
        pub unsafe fn initWithAPIToken(
            this: Option<Allocated<Self>>,
            api_token: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other APIToken)]
        pub unsafe fn APIToken(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAPIToken:)]
        pub unsafe fn setAPIToken(&self, api_token: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(fetchWebAuthTokenCompletionBlock)]
        pub unsafe fn fetchWebAuthTokenCompletionBlock(
            &self,
        ) -> *mut Block<(*mut NSString, *mut NSError), ()>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(setFetchWebAuthTokenCompletionBlock:)]
        pub unsafe fn setFetchWebAuthTokenCompletionBlock(
            &self,
            fetch_web_auth_token_completion_block: Option<
                &Block<(*mut NSString, *mut NSError), ()>,
            >,
        );
    }
);
