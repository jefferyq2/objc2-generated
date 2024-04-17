//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWorkspaceIconCreationOptions(pub NSUInteger);
impl NSWorkspaceIconCreationOptions {
    pub const NSExcludeQuickDrawElementsIconCreationOption: Self = Self(1 << 1);
    pub const NSExclude10_4ElementsIconCreationOption: Self = Self(1 << 2);
}

unsafe impl Encode for NSWorkspaceIconCreationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSWorkspaceIconCreationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSWorkspace;

    unsafe impl ClassType for NSWorkspace {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSWorkspace {}

extern_methods!(
    unsafe impl NSWorkspace {
        #[method_id(@__retain_semantics Other sharedWorkspace)]
        pub unsafe fn sharedWorkspace() -> Id<NSWorkspace>;

        #[method_id(@__retain_semantics Other notificationCenter)]
        pub unsafe fn notificationCenter(&self) -> Id<NSNotificationCenter>;

        #[method(openURL:)]
        pub unsafe fn openURL(&self, url: &NSURL) -> bool;

        #[cfg(all(feature = "NSRunningApplication", feature = "block2"))]
        #[method(openURL:configuration:completionHandler:)]
        pub unsafe fn openURL_configuration_completionHandler(
            &self,
            url: &NSURL,
            configuration: &NSWorkspaceOpenConfiguration,
            completion_handler: Option<&Block<dyn Fn(*mut NSRunningApplication, *mut NSError)>>,
        );

        #[cfg(all(feature = "NSRunningApplication", feature = "block2"))]
        #[method(openURLs:withApplicationAtURL:configuration:completionHandler:)]
        pub unsafe fn openURLs_withApplicationAtURL_configuration_completionHandler(
            &self,
            urls: &NSArray<NSURL>,
            application_url: &NSURL,
            configuration: &NSWorkspaceOpenConfiguration,
            completion_handler: Option<&Block<dyn Fn(*mut NSRunningApplication, *mut NSError)>>,
        );

        #[cfg(all(feature = "NSRunningApplication", feature = "block2"))]
        #[method(openApplicationAtURL:configuration:completionHandler:)]
        pub unsafe fn openApplicationAtURL_configuration_completionHandler(
            &self,
            application_url: &NSURL,
            configuration: &NSWorkspaceOpenConfiguration,
            completion_handler: Option<&Block<dyn Fn(*mut NSRunningApplication, *mut NSError)>>,
        );

        #[method(selectFile:inFileViewerRootedAtPath:)]
        pub unsafe fn selectFile_inFileViewerRootedAtPath(
            &self,
            full_path: Option<&NSString>,
            root_full_path: &NSString,
        ) -> bool;

        #[method(activateFileViewerSelectingURLs:)]
        pub unsafe fn activateFileViewerSelectingURLs(&self, file_ur_ls: &NSArray<NSURL>);

        #[method(showSearchResultsForQueryString:)]
        pub unsafe fn showSearchResultsForQueryString(&self, query_string: &NSString) -> bool;

        #[method(noteFileSystemChanged:)]
        pub unsafe fn noteFileSystemChanged_(&self, path: &NSString);

        #[method(isFilePackageAtPath:)]
        pub unsafe fn isFilePackageAtPath(&self, full_path: &NSString) -> bool;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other iconForFile:)]
        pub unsafe fn iconForFile(&self, full_path: &NSString) -> Id<NSImage>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other iconForFiles:)]
        pub unsafe fn iconForFiles(&self, full_paths: &NSArray<NSString>) -> Option<Id<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setIcon:forFile:options:)]
        pub unsafe fn setIcon_forFile_options(
            &self,
            image: Option<&NSImage>,
            full_path: &NSString,
            options: NSWorkspaceIconCreationOptions,
        ) -> bool;

        #[method_id(@__retain_semantics Other fileLabels)]
        pub unsafe fn fileLabels(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other fileLabelColors)]
        pub unsafe fn fileLabelColors(&self) -> Id<NSArray<NSColor>>;

        #[cfg(feature = "block2")]
        #[method(recycleURLs:completionHandler:)]
        pub unsafe fn recycleURLs_completionHandler(
            &self,
            ur_ls: &NSArray<NSURL>,
            handler: Option<&Block<dyn Fn(NonNull<NSDictionary<NSURL, NSURL>>, *mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(duplicateURLs:completionHandler:)]
        pub unsafe fn duplicateURLs_completionHandler(
            &self,
            ur_ls: &NSArray<NSURL>,
            handler: Option<&Block<dyn Fn(NonNull<NSDictionary<NSURL, NSURL>>, *mut NSError)>>,
        );

        #[method(getFileSystemInfoForPath:isRemovable:isWritable:isUnmountable:description:type:)]
        pub unsafe fn getFileSystemInfoForPath_isRemovable_isWritable_isUnmountable_description_type(
            &self,
            full_path: &NSString,
            removable_flag: *mut Bool,
            writable_flag: *mut Bool,
            unmountable_flag: *mut Bool,
            description: Option<&mut Option<Id<NSString>>>,
            file_system_type: Option<&mut Option<Id<NSString>>>,
        ) -> bool;

        #[method(unmountAndEjectDeviceAtPath:)]
        pub unsafe fn unmountAndEjectDeviceAtPath(&self, path: &NSString) -> bool;

        #[method(unmountAndEjectDeviceAtURL:error:_)]
        pub unsafe fn unmountAndEjectDeviceAtURL_error(
            &self,
            url: &NSURL,
        ) -> Result<(), Id<NSError>>;

        #[method(extendPowerOffBy:)]
        pub unsafe fn extendPowerOffBy(&self, requested: NSInteger) -> NSInteger;

        #[method(hideOtherApplications)]
        pub unsafe fn hideOtherApplications(&self);

        #[method_id(@__retain_semantics Other URLForApplicationWithBundleIdentifier:)]
        pub unsafe fn URLForApplicationWithBundleIdentifier(
            &self,
            bundle_identifier: &NSString,
        ) -> Option<Id<NSURL>>;

        #[method_id(@__retain_semantics Other URLsForApplicationsWithBundleIdentifier:)]
        pub unsafe fn URLsForApplicationsWithBundleIdentifier(
            &self,
            bundle_identifier: &NSString,
        ) -> Id<NSArray<NSURL>>;

        #[method_id(@__retain_semantics Other URLForApplicationToOpenURL:)]
        pub unsafe fn URLForApplicationToOpenURL(&self, url: &NSURL) -> Option<Id<NSURL>>;

        #[method_id(@__retain_semantics Other URLsForApplicationsToOpenURL:)]
        pub unsafe fn URLsForApplicationsToOpenURL(&self, url: &NSURL) -> Id<NSArray<NSURL>>;

        #[cfg(feature = "block2")]
        #[method(setDefaultApplicationAtURL:toOpenContentTypeOfFileAtURL:completionHandler:)]
        pub unsafe fn setDefaultApplicationAtURL_toOpenContentTypeOfFileAtURL_completionHandler(
            &self,
            application_url: &NSURL,
            url: &NSURL,
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(setDefaultApplicationAtURL:toOpenURLsWithScheme:completionHandler:)]
        pub unsafe fn setDefaultApplicationAtURL_toOpenURLsWithScheme_completionHandler(
            &self,
            application_url: &NSURL,
            url_scheme: &NSString,
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(setDefaultApplicationAtURL:toOpenFileAtURL:completionHandler:)]
        pub unsafe fn setDefaultApplicationAtURL_toOpenFileAtURL_completionHandler(
            &self,
            application_url: &NSURL,
            url: &NSURL,
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "NSRunningApplication")]
        #[method_id(@__retain_semantics Other frontmostApplication)]
        pub unsafe fn frontmostApplication(&self) -> Option<Id<NSRunningApplication>>;

        #[cfg(feature = "NSRunningApplication")]
        #[method_id(@__retain_semantics Other menuBarOwningApplication)]
        pub unsafe fn menuBarOwningApplication(&self) -> Option<Id<NSRunningApplication>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSWorkspace {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSWorkspaceOpenConfiguration;

    unsafe impl ClassType for NSWorkspaceOpenConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for NSWorkspaceOpenConfiguration {}

unsafe impl NSObjectProtocol for NSWorkspaceOpenConfiguration {}

extern_methods!(
    unsafe impl NSWorkspaceOpenConfiguration {
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration() -> Id<Self>;

        #[method(promptsUserIfNeeded)]
        pub unsafe fn promptsUserIfNeeded(&self) -> bool;

        #[method(setPromptsUserIfNeeded:)]
        pub unsafe fn setPromptsUserIfNeeded(&self, prompts_user_if_needed: bool);

        #[method(addsToRecentItems)]
        pub unsafe fn addsToRecentItems(&self) -> bool;

        #[method(setAddsToRecentItems:)]
        pub unsafe fn setAddsToRecentItems(&self, adds_to_recent_items: bool);

        #[method(activates)]
        pub unsafe fn activates(&self) -> bool;

        #[method(setActivates:)]
        pub unsafe fn setActivates(&self, activates: bool);

        #[method(hides)]
        pub unsafe fn hides(&self) -> bool;

        #[method(setHides:)]
        pub unsafe fn setHides(&self, hides: bool);

        #[method(hidesOthers)]
        pub unsafe fn hidesOthers(&self) -> bool;

        #[method(setHidesOthers:)]
        pub unsafe fn setHidesOthers(&self, hides_others: bool);

        #[method(isForPrinting)]
        pub unsafe fn isForPrinting(&self) -> bool;

        #[method(setForPrinting:)]
        pub unsafe fn setForPrinting(&self, for_printing: bool);

        #[method(createsNewApplicationInstance)]
        pub unsafe fn createsNewApplicationInstance(&self) -> bool;

        #[method(setCreatesNewApplicationInstance:)]
        pub unsafe fn setCreatesNewApplicationInstance(
            &self,
            creates_new_application_instance: bool,
        );

        #[method(allowsRunningApplicationSubstitution)]
        pub unsafe fn allowsRunningApplicationSubstitution(&self) -> bool;

        #[method(setAllowsRunningApplicationSubstitution:)]
        pub unsafe fn setAllowsRunningApplicationSubstitution(
            &self,
            allows_running_application_substitution: bool,
        );

        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Id<NSArray<NSString>>;

        #[method(setArguments:)]
        pub unsafe fn setArguments(&self, arguments: &NSArray<NSString>);

        #[method_id(@__retain_semantics Other environment)]
        pub unsafe fn environment(&self) -> Id<NSDictionary<NSString, NSString>>;

        #[method(setEnvironment:)]
        pub unsafe fn setEnvironment(&self, environment: &NSDictionary<NSString, NSString>);

        #[method_id(@__retain_semantics Other appleEvent)]
        pub unsafe fn appleEvent(&self) -> Option<Id<NSAppleEventDescriptor>>;

        #[method(setAppleEvent:)]
        pub unsafe fn setAppleEvent(&self, apple_event: Option<&NSAppleEventDescriptor>);

        #[method(requiresUniversalLinks)]
        pub unsafe fn requiresUniversalLinks(&self) -> bool;

        #[method(setRequiresUniversalLinks:)]
        pub unsafe fn setRequiresUniversalLinks(&self, requires_universal_links: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSWorkspaceOpenConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

// NS_TYPED_ENUM
pub type NSWorkspaceDesktopImageOptionKey = NSString;

extern "C" {
    pub static NSWorkspaceDesktopImageScalingKey: &'static NSWorkspaceDesktopImageOptionKey;
}

extern "C" {
    pub static NSWorkspaceDesktopImageAllowClippingKey: &'static NSWorkspaceDesktopImageOptionKey;
}

extern "C" {
    pub static NSWorkspaceDesktopImageFillColorKey: &'static NSWorkspaceDesktopImageOptionKey;
}

extern_methods!(
    /// NSDesktopImages
    unsafe impl NSWorkspace {
        #[cfg(feature = "NSScreen")]
        #[method(setDesktopImageURL:forScreen:options:error:_)]
        pub unsafe fn setDesktopImageURL_forScreen_options_error(
            &self,
            url: &NSURL,
            screen: &NSScreen,
            options: &NSDictionary<NSWorkspaceDesktopImageOptionKey, AnyObject>,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "NSScreen")]
        #[method_id(@__retain_semantics Other desktopImageURLForScreen:)]
        pub unsafe fn desktopImageURLForScreen(&self, screen: &NSScreen) -> Option<Id<NSURL>>;

        #[cfg(feature = "NSScreen")]
        #[method_id(@__retain_semantics Other desktopImageOptionsForScreen:)]
        pub unsafe fn desktopImageOptionsForScreen(
            &self,
            screen: &NSScreen,
        ) -> Option<Id<NSDictionary<NSWorkspaceDesktopImageOptionKey, AnyObject>>>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWorkspaceAuthorizationType(pub NSInteger);
impl NSWorkspaceAuthorizationType {
    #[doc(alias = "NSWorkspaceAuthorizationTypeCreateSymbolicLink")]
    pub const CreateSymbolicLink: Self = Self(0);
    #[doc(alias = "NSWorkspaceAuthorizationTypeSetAttributes")]
    pub const SetAttributes: Self = Self(1);
    #[doc(alias = "NSWorkspaceAuthorizationTypeReplaceFile")]
    pub const ReplaceFile: Self = Self(2);
}

unsafe impl Encode for NSWorkspaceAuthorizationType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSWorkspaceAuthorizationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSWorkspaceAuthorization;

    unsafe impl ClassType for NSWorkspaceAuthorization {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSWorkspaceAuthorization {}

extern_methods!(
    unsafe impl NSWorkspaceAuthorization {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSWorkspaceAuthorization {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSWorkspaceAuthorization
    unsafe impl NSWorkspace {
        #[cfg(feature = "block2")]
        #[method(requestAuthorizationOfType:completionHandler:)]
        pub unsafe fn requestAuthorizationOfType_completionHandler(
            &self,
            r#type: NSWorkspaceAuthorizationType,
            completion_handler: &Block<dyn Fn(*mut NSWorkspaceAuthorization, *mut NSError)>,
        );
    }
);

extern_category!(
    /// Category "NSWorkspaceAuthorization" on [`NSFileManager`].
    #[doc(alias = "NSWorkspaceAuthorization")]
    pub unsafe trait NSFileManagerNSWorkspaceAuthorization {
        #[method_id(@__retain_semantics Other fileManagerWithAuthorization:)]
        unsafe fn fileManagerWithAuthorization(
            authorization: &NSWorkspaceAuthorization,
        ) -> Id<Self>;
    }

    unsafe impl NSFileManagerNSWorkspaceAuthorization for NSFileManager {}
);

extern "C" {
    pub static NSWorkspaceApplicationKey: &'static NSString;
}

extern "C" {
    pub static NSWorkspaceWillLaunchApplicationNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceDidLaunchApplicationNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceDidTerminateApplicationNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceDidHideApplicationNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceDidUnhideApplicationNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceDidActivateApplicationNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceDidDeactivateApplicationNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceVolumeLocalizedNameKey: &'static NSString;
}

extern "C" {
    pub static NSWorkspaceVolumeURLKey: &'static NSString;
}

extern "C" {
    pub static NSWorkspaceVolumeOldLocalizedNameKey: &'static NSString;
}

extern "C" {
    pub static NSWorkspaceVolumeOldURLKey: &'static NSString;
}

extern "C" {
    pub static NSWorkspaceDidMountNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceDidUnmountNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceWillUnmountNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceDidRenameVolumeNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceWillPowerOffNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceWillSleepNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceDidWakeNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceScreensDidSleepNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceScreensDidWakeNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceSessionDidBecomeActiveNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceSessionDidResignActiveNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceDidChangeFileLabelsNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSWorkspaceActiveSpaceDidChangeNotification: &'static NSNotificationName;
}

// NS_TYPED_ENUM
pub type NSWorkspaceFileOperationName = NSString;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWorkspaceLaunchOptions(pub NSUInteger);
impl NSWorkspaceLaunchOptions {
    #[deprecated = "Use -[NSWorkspaceOpenConfiguration setForPrinting:YES] instead."]
    pub const NSWorkspaceLaunchAndPrint: Self = Self(0x00000002);
    #[deprecated = "Use -[NSWorkspaceOpenConfiguration setPromptsUserIfNeeded:YES] instead."]
    pub const NSWorkspaceLaunchWithErrorPresentation: Self = Self(0x00000040);
    #[deprecated = "This option does nothing."]
    pub const NSWorkspaceLaunchInhibitingBackgroundOnly: Self = Self(0x00000080);
    #[deprecated = "Use -[NSWorkspaceOpenConfiguration setAddsToRecentItems:YES] instead."]
    pub const NSWorkspaceLaunchWithoutAddingToRecents: Self = Self(0x00000100);
    #[deprecated = "Use -[NSWorkspaceOpenConfiguration setActivates:NO] instead."]
    pub const NSWorkspaceLaunchWithoutActivation: Self = Self(0x00000200);
    #[deprecated = "When using NSWorkspaceOpenConfiguration, all launches are asynchronous."]
    pub const NSWorkspaceLaunchAsync: Self = Self(0x00010000);
    #[deprecated = "Use -[NSWorkspaceOpenConfiguration setCreatesNewApplicationInstance:YES] instead."]
    pub const NSWorkspaceLaunchNewInstance: Self = Self(0x00080000);
    #[deprecated = "Use -[NSWorkspaceOpenConfiguration setHides:YES] instead."]
    pub const NSWorkspaceLaunchAndHide: Self = Self(0x00100000);
    #[deprecated = "Use -[NSWorkspaceOpenConfiguration setHidesOthers:YES] instead."]
    pub const NSWorkspaceLaunchAndHideOthers: Self = Self(0x00200000);
    #[deprecated = "Use NSWorkspaceOpenConfiguration instead."]
    pub const NSWorkspaceLaunchDefault: Self =
        Self(NSWorkspaceLaunchOptions::NSWorkspaceLaunchAsync.0);
    #[deprecated = "The Classic environment is no longer supported."]
    pub const NSWorkspaceLaunchAllowingClassicStartup: Self = Self(0x00020000);
    #[deprecated = "The Classic environment is no longer supported."]
    pub const NSWorkspaceLaunchPreferringClassic: Self = Self(0x00040000);
}

unsafe impl Encode for NSWorkspaceLaunchOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSWorkspaceLaunchOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_ENUM
pub type NSWorkspaceLaunchConfigurationKey = NSString;

extern "C" {
    pub static NSWorkspaceLaunchConfigurationAppleEvent: &'static NSWorkspaceLaunchConfigurationKey;
}

extern "C" {
    pub static NSWorkspaceLaunchConfigurationArguments: &'static NSWorkspaceLaunchConfigurationKey;
}

extern "C" {
    pub static NSWorkspaceLaunchConfigurationEnvironment:
        &'static NSWorkspaceLaunchConfigurationKey;
}

extern "C" {
    pub static NSWorkspaceLaunchConfigurationArchitecture:
        &'static NSWorkspaceLaunchConfigurationKey;
}

extern_methods!(
    /// NSDeprecated
    unsafe impl NSWorkspace {
        #[deprecated = "Use -[NSWorkspace openURL:] instead."]
        #[method(openFile:)]
        pub unsafe fn openFile(&self, full_path: &NSString) -> bool;

        #[deprecated = "Use -[NSWorkspace openURLs:withApplicationAtURL:configuration:completionHandler:] instead."]
        #[method(openFile:withApplication:)]
        pub unsafe fn openFile_withApplication(
            &self,
            full_path: &NSString,
            app_name: Option<&NSString>,
        ) -> bool;

        #[deprecated = "Use -[NSWorkspace openURLs:withApplicationAtURL:configuration:completionHandler:] instead."]
        #[method(openFile:withApplication:andDeactivate:)]
        pub unsafe fn openFile_withApplication_andDeactivate(
            &self,
            full_path: &NSString,
            app_name: Option<&NSString>,
            flag: bool,
        ) -> bool;

        #[deprecated = "Use -[NSWorkspace openApplicationAtURL:configuration:completionHandler:] instead."]
        #[method(launchApplication:)]
        pub unsafe fn launchApplication(&self, app_name: &NSString) -> bool;

        #[cfg(feature = "NSRunningApplication")]
        #[deprecated = "Use -[NSWorkspace openApplicationAtURL:configuration:completionHandler:] instead."]
        #[method_id(@__retain_semantics Other launchApplicationAtURL:options:configuration:error:_)]
        pub unsafe fn launchApplicationAtURL_options_configuration_error(
            &self,
            url: &NSURL,
            options: NSWorkspaceLaunchOptions,
            configuration: &NSDictionary<NSWorkspaceLaunchConfigurationKey, AnyObject>,
        ) -> Result<Id<NSRunningApplication>, Id<NSError>>;

        #[cfg(feature = "NSRunningApplication")]
        #[deprecated = "Use -[NSWorkspace openURL:configuration:completionHandler:] instead."]
        #[method_id(@__retain_semantics Other openURL:options:configuration:error:_)]
        pub unsafe fn openURL_options_configuration_error(
            &self,
            url: &NSURL,
            options: NSWorkspaceLaunchOptions,
            configuration: &NSDictionary<NSWorkspaceLaunchConfigurationKey, AnyObject>,
        ) -> Result<Id<NSRunningApplication>, Id<NSError>>;

        #[cfg(feature = "NSRunningApplication")]
        #[deprecated = "Use -[NSWorkspace openURLs:withApplicationAtURL:configuration:completionHandler:] instead."]
        #[method_id(@__retain_semantics Other openURLs:withApplicationAtURL:options:configuration:error:_)]
        pub unsafe fn openURLs_withApplicationAtURL_options_configuration_error(
            &self,
            urls: &NSArray<NSURL>,
            application_url: &NSURL,
            options: NSWorkspaceLaunchOptions,
            configuration: &NSDictionary<NSWorkspaceLaunchConfigurationKey, AnyObject>,
        ) -> Result<Id<NSRunningApplication>, Id<NSError>>;

        #[deprecated = "Use -[NSWorkspace openApplicationAtURL:configuration:completionHandler:] instead."]
        #[method(launchApplication:showIcon:autolaunch:)]
        pub unsafe fn launchApplication_showIcon_autolaunch(
            &self,
            app_name: &NSString,
            show_icon: bool,
            autolaunch: bool,
        ) -> bool;

        #[deprecated = "Use -[NSWorkspace URLForApplicationWithBundleIdentifier:] instead."]
        #[method_id(@__retain_semantics Other fullPathForApplication:)]
        pub unsafe fn fullPathForApplication(&self, app_name: &NSString) -> Option<Id<NSString>>;

        #[deprecated = "Use -[NSWorkspace URLForApplicationWithBundleIdentifier:] instead."]
        #[method_id(@__retain_semantics Other absolutePathForAppBundleWithIdentifier:)]
        pub unsafe fn absolutePathForAppBundleWithIdentifier(
            &self,
            bundle_identifier: &NSString,
        ) -> Option<Id<NSString>>;

        #[deprecated = "Use -[NSWorkspace openApplicationAtURL:configuration:completionHandler:] instead."]
        #[method(launchAppWithBundleIdentifier:options:additionalEventParamDescriptor:launchIdentifier:)]
        pub unsafe fn launchAppWithBundleIdentifier_options_additionalEventParamDescriptor_launchIdentifier(
            &self,
            bundle_identifier: &NSString,
            options: NSWorkspaceLaunchOptions,
            descriptor: Option<&NSAppleEventDescriptor>,
            identifier: Option<&mut Option<Id<NSNumber>>>,
        ) -> bool;

        #[deprecated = "Use -[NSWorkspace openURLs:withApplicationAtURL:configuration:completionHandler:] instead."]
        #[method(openURLs:withAppBundleIdentifier:options:additionalEventParamDescriptor:launchIdentifiers:)]
        pub unsafe fn openURLs_withAppBundleIdentifier_options_additionalEventParamDescriptor_launchIdentifiers(
            &self,
            urls: &NSArray<NSURL>,
            bundle_identifier: Option<&NSString>,
            options: NSWorkspaceLaunchOptions,
            descriptor: Option<&NSAppleEventDescriptor>,
            identifiers: Option<&mut Option<Id<NSArray<NSNumber>>>>,
        ) -> bool;

        #[deprecated]
        #[method(openTempFile:)]
        pub unsafe fn openTempFile(&self, full_path: &NSString) -> bool;

        #[deprecated]
        #[method(findApplications)]
        pub unsafe fn findApplications(&self);

        #[deprecated]
        #[method(noteUserDefaultsChanged)]
        pub unsafe fn noteUserDefaultsChanged(&self);

        #[cfg(feature = "NSImage")]
        #[deprecated]
        #[method(slideImage:from:to:)]
        pub unsafe fn slideImage_from_to(
            &self,
            image: &NSImage,
            from_point: NSPoint,
            to_point: NSPoint,
        );

        #[deprecated]
        #[method(checkForRemovableMedia)]
        pub unsafe fn checkForRemovableMedia(&self);

        #[deprecated]
        #[method(noteFileSystemChanged)]
        pub unsafe fn noteFileSystemChanged(&self);

        #[deprecated]
        #[method(fileSystemChanged)]
        pub unsafe fn fileSystemChanged(&self) -> bool;

        #[deprecated]
        #[method(userDefaultsChanged)]
        pub unsafe fn userDefaultsChanged(&self) -> bool;

        #[deprecated]
        #[method_id(@__retain_semantics Other mountNewRemovableMedia)]
        pub unsafe fn mountNewRemovableMedia(&self) -> Option<Id<NSArray>>;

        #[deprecated = "Use -[NSWorkspace frontmostApplication] instead."]
        #[method_id(@__retain_semantics Other activeApplication)]
        pub unsafe fn activeApplication(&self) -> Option<Id<NSDictionary>>;

        #[deprecated = "Use -[NSFileManager mountedVolumeURLsIncludingResourceValuesForKeys:options:] instead."]
        #[method_id(@__retain_semantics Other mountedLocalVolumePaths)]
        pub unsafe fn mountedLocalVolumePaths(&self) -> Option<Id<NSArray>>;

        #[deprecated = "Use -[NSFileManager mountedVolumeURLsIncludingResourceValuesForKeys:options:] instead."]
        #[method_id(@__retain_semantics Other mountedRemovableMedia)]
        pub unsafe fn mountedRemovableMedia(&self) -> Option<Id<NSArray>>;

        #[deprecated = "Use -[NSWorkspace runningApplications] instead."]
        #[method_id(@__retain_semantics Other launchedApplications)]
        pub unsafe fn launchedApplications(&self) -> Option<Id<NSArray>>;

        #[cfg(all(feature = "NSImage", feature = "NSResponder", feature = "NSView"))]
        #[deprecated = "Use -[NSWorkspace openURL:] instead."]
        #[method(openFile:fromImage:at:inView:)]
        pub unsafe fn openFile_fromImage_at_inView(
            &self,
            full_path: &NSString,
            image: Option<&NSImage>,
            point: NSPoint,
            view: Option<&NSView>,
        ) -> bool;

        #[deprecated]
        #[method(performFileOperation:source:destination:files:tag:)]
        pub unsafe fn performFileOperation_source_destination_files_tag(
            &self,
            operation: &NSWorkspaceFileOperationName,
            source: &NSString,
            destination: &NSString,
            files: &NSArray,
            tag: *mut NSInteger,
        ) -> bool;

        #[deprecated = "Use -[NSWorkspace URLForApplicationToOpenURL:] to get the URL of an application that will open a given item, or -[NSURL getResourceValue:forKey:error:] with NSURLContentTypeKey to get the type of the given item."]
        #[method(getInfoForFile:application:type:)]
        pub unsafe fn getInfoForFile_application_type(
            &self,
            full_path: &NSString,
            app_name: Option<&mut Option<Id<NSString>>>,
            r#type: Option<&mut Option<Id<NSString>>>,
        ) -> bool;

        #[cfg(feature = "NSImage")]
        #[deprecated = "Use -[NSWorkspace iconForContentType:] instead."]
        #[method_id(@__retain_semantics Other iconForFileType:)]
        pub unsafe fn iconForFileType(&self, file_type: &NSString) -> Id<NSImage>;

        #[deprecated = "Use -[NSURL getResourceValue:forKey:error:] with NSURLContentTypeKey instead."]
        #[method_id(@__retain_semantics Other typeOfFile:error:_)]
        pub unsafe fn typeOfFile_error(
            &self,
            absolute_file_path: &NSString,
        ) -> Result<Id<NSString>, Id<NSError>>;

        #[deprecated = "Use UTType.localizedDescription instead."]
        #[method_id(@__retain_semantics Other localizedDescriptionForType:)]
        pub unsafe fn localizedDescriptionForType(
            &self,
            type_name: &NSString,
        ) -> Option<Id<NSString>>;

        #[deprecated = "Use UTType.preferredFilenameExtension instead."]
        #[method_id(@__retain_semantics Other preferredFilenameExtensionForType:)]
        pub unsafe fn preferredFilenameExtensionForType(
            &self,
            type_name: &NSString,
        ) -> Option<Id<NSString>>;

        #[deprecated = "Use +[UTType typesWithTag:tagClass:conformingToType:] to get a list of candidate types, then check if the input type conforms to any of them."]
        #[method(filenameExtension:isValidForType:)]
        pub unsafe fn filenameExtension_isValidForType(
            &self,
            filename_extension: &NSString,
            type_name: &NSString,
        ) -> bool;

        #[deprecated = "Use -[UTType conformsToType:] instead."]
        #[method(type:conformsToType:)]
        pub unsafe fn type_conformsToType(
            &self,
            first_type_name: &NSString,
            second_type_name: &NSString,
        ) -> bool;
    }
);

extern "C" {
    pub static NSWorkspaceMoveOperation: &'static NSWorkspaceFileOperationName;
}

extern "C" {
    pub static NSWorkspaceCopyOperation: &'static NSWorkspaceFileOperationName;
}

extern "C" {
    pub static NSWorkspaceLinkOperation: &'static NSWorkspaceFileOperationName;
}

extern "C" {
    pub static NSWorkspaceCompressOperation: &'static NSWorkspaceFileOperationName;
}

extern "C" {
    pub static NSWorkspaceDecompressOperation: &'static NSWorkspaceFileOperationName;
}

extern "C" {
    pub static NSWorkspaceEncryptOperation: &'static NSWorkspaceFileOperationName;
}

extern "C" {
    pub static NSWorkspaceDecryptOperation: &'static NSWorkspaceFileOperationName;
}

extern "C" {
    pub static NSWorkspaceDestroyOperation: &'static NSWorkspaceFileOperationName;
}

extern "C" {
    pub static NSWorkspaceRecycleOperation: &'static NSWorkspaceFileOperationName;
}

extern "C" {
    pub static NSWorkspaceDuplicateOperation: &'static NSWorkspaceFileOperationName;
}

extern "C" {
    pub static NSWorkspaceDidPerformFileOperationNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSPlainFileType: &'static NSString;
}

extern "C" {
    pub static NSDirectoryFileType: &'static NSString;
}

extern "C" {
    pub static NSApplicationFileType: &'static NSString;
}

extern "C" {
    pub static NSFilesystemFileType: &'static NSString;
}

extern "C" {
    pub static NSShellCommandFileType: &'static NSString;
}
