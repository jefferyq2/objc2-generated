//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

typed_enum!(
    pub type SKCloudServiceSetupOptionsKey = NSString;
);

typed_enum!(
    pub type SKCloudServiceSetupAction = NSString;
);

typed_enum!(
    pub type SKCloudServiceSetupMessageIdentifier = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
    pub struct SKCloudServiceSetupViewController;

    #[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
    unsafe impl ClassType for SKCloudServiceSetupViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

#[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
unsafe impl NSCoding for SKCloudServiceSetupViewController {}

#[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
unsafe impl NSEditor for SKCloudServiceSetupViewController {}

#[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
unsafe impl NSObjectProtocol for SKCloudServiceSetupViewController {}

#[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
unsafe impl NSSeguePerforming for SKCloudServiceSetupViewController {}

#[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
unsafe impl NSUserInterfaceItemIdentification for SKCloudServiceSetupViewController {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
    unsafe impl SKCloudServiceSetupViewController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn SKCloudServiceSetupViewControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn SKCloudServiceSetupViewControllerDelegate>>,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSError"))]
        #[method(loadWithOptions:completionHandler:)]
        pub unsafe fn loadWithOptions_completionHandler(
            &self,
            options: &NSDictionary<SKCloudServiceSetupOptionsKey, Object>,
            completion_handler: Option<&Block<(Bool, *mut NSError), ()>>,
        );
    }
);

extern_protocol!(
    pub unsafe trait SKCloudServiceSetupViewControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
        #[optional]
        #[method(cloudServiceSetupViewControllerDidDismiss:)]
        unsafe fn cloudServiceSetupViewControllerDidDismiss(
            &self,
            cloud_service_setup_view_controller: &SKCloudServiceSetupViewController,
        );
    }

    unsafe impl ProtocolType for dyn SKCloudServiceSetupViewControllerDelegate {}
);

extern_static!(SKCloudServiceSetupOptionsActionKey: &'static SKCloudServiceSetupOptionsKey);

extern_static!(
    SKCloudServiceSetupOptionsITunesItemIdentifierKey: &'static SKCloudServiceSetupOptionsKey
);

extern_static!(SKCloudServiceSetupOptionsAffiliateTokenKey: &'static SKCloudServiceSetupOptionsKey);

extern_static!(SKCloudServiceSetupOptionsCampaignTokenKey: &'static SKCloudServiceSetupOptionsKey);

extern_static!(
    SKCloudServiceSetupOptionsMessageIdentifierKey: &'static SKCloudServiceSetupOptionsKey
);

extern_static!(SKCloudServiceSetupActionSubscribe: &'static SKCloudServiceSetupAction);

extern_static!(
    SKCloudServiceSetupMessageIdentifierJoin: &'static SKCloudServiceSetupMessageIdentifier
);

extern_static!(
    SKCloudServiceSetupMessageIdentifierConnect: &'static SKCloudServiceSetupMessageIdentifier
);

extern_static!(
    SKCloudServiceSetupMessageIdentifierAddMusic: &'static SKCloudServiceSetupMessageIdentifier
);

extern_static!(
    SKCloudServiceSetupMessageIdentifierPlayMusic: &'static SKCloudServiceSetupMessageIdentifier
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "StoreKit_SKCloudServiceSetupViewController")]
    unsafe impl SKCloudServiceSetupViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;
    }
);
