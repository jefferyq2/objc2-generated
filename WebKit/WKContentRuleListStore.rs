//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKContentRuleListStore")]
    pub struct WKContentRuleListStore;

    #[cfg(feature = "WebKit_WKContentRuleListStore")]
    unsafe impl ClassType for WKContentRuleListStore {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WKContentRuleListStore")]
unsafe impl NSObjectProtocol for WKContentRuleListStore {}

extern_methods!(
    #[cfg(feature = "WebKit_WKContentRuleListStore")]
    unsafe impl WKContentRuleListStore {
        #[method_id(@__retain_semantics Other defaultStore)]
        pub unsafe fn defaultStore() -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other storeWithURL:)]
        pub unsafe fn storeWithURL(url: Option<&NSURL>) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "WebKit_WKContentRuleList"
        ))]
        #[method(compileContentRuleListForIdentifier:encodedContentRuleList:completionHandler:)]
        pub unsafe fn compileContentRuleListForIdentifier_encodedContentRuleList_completionHandler(
            &self,
            identifier: Option<&NSString>,
            encoded_content_rule_list: Option<&NSString>,
            completion_handler: Option<&Block<(*mut WKContentRuleList, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "WebKit_WKContentRuleList"
        ))]
        #[method(lookUpContentRuleListForIdentifier:completionHandler:)]
        pub unsafe fn lookUpContentRuleListForIdentifier_completionHandler(
            &self,
            identifier: Option<&NSString>,
            completion_handler: Option<&Block<(*mut WKContentRuleList, *mut NSError), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(removeContentRuleListForIdentifier:completionHandler:)]
        pub unsafe fn removeContentRuleListForIdentifier_completionHandler(
            &self,
            identifier: Option<&NSString>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(getAvailableContentRuleListIdentifiers:)]
        pub unsafe fn getAvailableContentRuleListIdentifiers(
            &self,
            completion_handler: Option<&Block<(*mut NSArray<NSString>,), ()>>,
        );
    }
);
