//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSFileAttributeKey = NSString;

pub type NSFileAttributeType = NSString;

pub type NSFileProtectionType = NSString;

pub type NSFileProviderServiceName = NSString;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSVolumeEnumerationOptions {
        NSVolumeEnumerationSkipHiddenVolumes = 1 << 1,
        NSVolumeEnumerationProduceFileReferenceURLs = 1 << 2,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDirectoryEnumerationOptions {
        NSDirectoryEnumerationSkipsSubdirectoryDescendants = 1 << 0,
        NSDirectoryEnumerationSkipsPackageDescendants = 1 << 1,
        NSDirectoryEnumerationSkipsHiddenFiles = 1 << 2,
        NSDirectoryEnumerationIncludesDirectoriesPostOrder = 1 << 3,
        NSDirectoryEnumerationProducesRelativePathURLs = 1 << 4,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileManagerItemReplacementOptions {
        NSFileManagerItemReplacementUsingNewMetadataOnly = 1 << 0,
        NSFileManagerItemReplacementWithoutDeletingBackupItem = 1 << 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSURLRelationship {
        NSURLRelationshipContains = 0,
        NSURLRelationshipSame = 1,
        NSURLRelationshipOther = 2,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileManagerUnmountOptions {
        NSFileManagerUnmountAllPartitionsAndEjectDisk = 1 << 0,
        NSFileManagerUnmountWithoutUI = 1 << 1,
    }
);

extern_static!(NSFileManagerUnmountDissentingProcessIdentifierErrorKey: &'static NSString);

extern_static!(NSUbiquityIdentityDidChangeNotification: &'static NSNotificationName);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFileManager;

    unsafe impl ClassType for NSFileManager {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSFileManager {
        #[method_id(@__retain_semantics Other defaultManager)]
        pub unsafe fn defaultManager() -> Id<NSFileManager, Shared>;

        #[method_id(@__retain_semantics Other mountedVolumeURLsIncludingResourceValuesForKeys:options:)]
        pub unsafe fn mountedVolumeURLsIncludingResourceValuesForKeys_options(
            &self,
            propertyKeys: Option<&NSArray<NSURLResourceKey>>,
            options: NSVolumeEnumerationOptions,
        ) -> Option<Id<NSArray<NSURL>, Shared>>;

        #[method(unmountVolumeAtURL:options:completionHandler:)]
        pub unsafe fn unmountVolumeAtURL_options_completionHandler(
            &self,
            url: &NSURL,
            mask: NSFileManagerUnmountOptions,
            completionHandler: &Block<(*mut NSError,), ()>,
        );

        #[method_id(@__retain_semantics Other contentsOfDirectoryAtURL:includingPropertiesForKeys:options:error:)]
        pub unsafe fn contentsOfDirectoryAtURL_includingPropertiesForKeys_options_error(
            &self,
            url: &NSURL,
            keys: Option<&NSArray<NSURLResourceKey>>,
            mask: NSDirectoryEnumerationOptions,
        ) -> Result<Id<NSArray<NSURL>, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other URLsForDirectory:inDomains:)]
        pub unsafe fn URLsForDirectory_inDomains(
            &self,
            directory: NSSearchPathDirectory,
            domainMask: NSSearchPathDomainMask,
        ) -> Id<NSArray<NSURL>, Shared>;

        #[method_id(@__retain_semantics Other URLForDirectory:inDomain:appropriateForURL:create:error:)]
        pub unsafe fn URLForDirectory_inDomain_appropriateForURL_create_error(
            &self,
            directory: NSSearchPathDirectory,
            domain: NSSearchPathDomainMask,
            url: Option<&NSURL>,
            shouldCreate: bool,
        ) -> Result<Id<NSURL, Shared>, Id<NSError, Shared>>;

        #[method(getRelationship:ofDirectoryAtURL:toItemAtURL:error:)]
        pub unsafe fn getRelationship_ofDirectoryAtURL_toItemAtURL_error(
            &self,
            outRelationship: NonNull<NSURLRelationship>,
            directoryURL: &NSURL,
            otherURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(getRelationship:ofDirectory:inDomain:toItemAtURL:error:)]
        pub unsafe fn getRelationship_ofDirectory_inDomain_toItemAtURL_error(
            &self,
            outRelationship: NonNull<NSURLRelationship>,
            directory: NSSearchPathDirectory,
            domainMask: NSSearchPathDomainMask,
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(createDirectoryAtURL:withIntermediateDirectories:attributes:error:)]
        pub unsafe fn createDirectoryAtURL_withIntermediateDirectories_attributes_error(
            &self,
            url: &NSURL,
            createIntermediates: bool,
            attributes: Option<&NSDictionary<NSFileAttributeKey, Object>>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(createSymbolicLinkAtURL:withDestinationURL:error:)]
        pub unsafe fn createSymbolicLinkAtURL_withDestinationURL_error(
            &self,
            url: &NSURL,
            destURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSFileManagerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSFileManagerDelegate>);

        #[method(setAttributes:ofItemAtPath:error:)]
        pub unsafe fn setAttributes_ofItemAtPath_error(
            &self,
            attributes: &NSDictionary<NSFileAttributeKey, Object>,
            path: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(createDirectoryAtPath:withIntermediateDirectories:attributes:error:)]
        pub unsafe fn createDirectoryAtPath_withIntermediateDirectories_attributes_error(
            &self,
            path: &NSString,
            createIntermediates: bool,
            attributes: Option<&NSDictionary<NSFileAttributeKey, Object>>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other contentsOfDirectoryAtPath:error:)]
        pub unsafe fn contentsOfDirectoryAtPath_error(
            &self,
            path: &NSString,
        ) -> Result<Id<NSArray<NSString>, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other subpathsOfDirectoryAtPath:error:)]
        pub unsafe fn subpathsOfDirectoryAtPath_error(
            &self,
            path: &NSString,
        ) -> Result<Id<NSArray<NSString>, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other attributesOfItemAtPath:error:)]
        pub unsafe fn attributesOfItemAtPath_error(
            &self,
            path: &NSString,
        ) -> Result<Id<NSDictionary<NSFileAttributeKey, Object>, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other attributesOfFileSystemForPath:error:)]
        pub unsafe fn attributesOfFileSystemForPath_error(
            &self,
            path: &NSString,
        ) -> Result<Id<NSDictionary<NSFileAttributeKey, Object>, Shared>, Id<NSError, Shared>>;

        #[method(createSymbolicLinkAtPath:withDestinationPath:error:)]
        pub unsafe fn createSymbolicLinkAtPath_withDestinationPath_error(
            &self,
            path: &NSString,
            destPath: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other destinationOfSymbolicLinkAtPath:error:)]
        pub unsafe fn destinationOfSymbolicLinkAtPath_error(
            &self,
            path: &NSString,
        ) -> Result<Id<NSString, Shared>, Id<NSError, Shared>>;

        #[method(copyItemAtPath:toPath:error:)]
        pub unsafe fn copyItemAtPath_toPath_error(
            &self,
            srcPath: &NSString,
            dstPath: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(moveItemAtPath:toPath:error:)]
        pub unsafe fn moveItemAtPath_toPath_error(
            &self,
            srcPath: &NSString,
            dstPath: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(linkItemAtPath:toPath:error:)]
        pub unsafe fn linkItemAtPath_toPath_error(
            &self,
            srcPath: &NSString,
            dstPath: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(removeItemAtPath:error:)]
        pub unsafe fn removeItemAtPath_error(
            &self,
            path: &NSString,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(copyItemAtURL:toURL:error:)]
        pub unsafe fn copyItemAtURL_toURL_error(
            &self,
            srcURL: &NSURL,
            dstURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(moveItemAtURL:toURL:error:)]
        pub unsafe fn moveItemAtURL_toURL_error(
            &self,
            srcURL: &NSURL,
            dstURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(linkItemAtURL:toURL:error:)]
        pub unsafe fn linkItemAtURL_toURL_error(
            &self,
            srcURL: &NSURL,
            dstURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(removeItemAtURL:error:)]
        pub unsafe fn removeItemAtURL_error(&self, URL: &NSURL) -> Result<(), Id<NSError, Shared>>;

        #[method(trashItemAtURL:resultingItemURL:error:)]
        pub unsafe fn trashItemAtURL_resultingItemURL_error(
            &self,
            url: &NSURL,
            outResultingURL: *mut *mut NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other fileAttributesAtPath:traverseLink:)]
        pub unsafe fn fileAttributesAtPath_traverseLink(
            &self,
            path: &NSString,
            yorn: bool,
        ) -> Option<Id<NSDictionary, Shared>>;

        #[method(changeFileAttributes:atPath:)]
        pub unsafe fn changeFileAttributes_atPath(
            &self,
            attributes: &NSDictionary,
            path: &NSString,
        ) -> bool;

        #[method_id(@__retain_semantics Other directoryContentsAtPath:)]
        pub unsafe fn directoryContentsAtPath(
            &self,
            path: &NSString,
        ) -> Option<Id<NSArray, Shared>>;

        #[method_id(@__retain_semantics Other fileSystemAttributesAtPath:)]
        pub unsafe fn fileSystemAttributesAtPath(
            &self,
            path: &NSString,
        ) -> Option<Id<NSDictionary, Shared>>;

        #[method_id(@__retain_semantics Other pathContentOfSymbolicLinkAtPath:)]
        pub unsafe fn pathContentOfSymbolicLinkAtPath(
            &self,
            path: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method(createSymbolicLinkAtPath:pathContent:)]
        pub unsafe fn createSymbolicLinkAtPath_pathContent(
            &self,
            path: &NSString,
            otherpath: &NSString,
        ) -> bool;

        #[method(createDirectoryAtPath:attributes:)]
        pub unsafe fn createDirectoryAtPath_attributes(
            &self,
            path: &NSString,
            attributes: &NSDictionary,
        ) -> bool;

        #[method(linkPath:toPath:handler:)]
        pub unsafe fn linkPath_toPath_handler(
            &self,
            src: &NSString,
            dest: &NSString,
            handler: Option<&Object>,
        ) -> bool;

        #[method(copyPath:toPath:handler:)]
        pub unsafe fn copyPath_toPath_handler(
            &self,
            src: &NSString,
            dest: &NSString,
            handler: Option<&Object>,
        ) -> bool;

        #[method(movePath:toPath:handler:)]
        pub unsafe fn movePath_toPath_handler(
            &self,
            src: &NSString,
            dest: &NSString,
            handler: Option<&Object>,
        ) -> bool;

        #[method(removeFileAtPath:handler:)]
        pub unsafe fn removeFileAtPath_handler(
            &self,
            path: &NSString,
            handler: Option<&Object>,
        ) -> bool;

        #[method_id(@__retain_semantics Other currentDirectoryPath)]
        pub unsafe fn currentDirectoryPath(&self) -> Id<NSString, Shared>;

        #[method(changeCurrentDirectoryPath:)]
        pub unsafe fn changeCurrentDirectoryPath(&self, path: &NSString) -> bool;

        #[method(fileExistsAtPath:)]
        pub unsafe fn fileExistsAtPath(&self, path: &NSString) -> bool;

        #[method(fileExistsAtPath:isDirectory:)]
        pub unsafe fn fileExistsAtPath_isDirectory(
            &self,
            path: &NSString,
            isDirectory: *mut Bool,
        ) -> bool;

        #[method(isReadableFileAtPath:)]
        pub unsafe fn isReadableFileAtPath(&self, path: &NSString) -> bool;

        #[method(isWritableFileAtPath:)]
        pub unsafe fn isWritableFileAtPath(&self, path: &NSString) -> bool;

        #[method(isExecutableFileAtPath:)]
        pub unsafe fn isExecutableFileAtPath(&self, path: &NSString) -> bool;

        #[method(isDeletableFileAtPath:)]
        pub unsafe fn isDeletableFileAtPath(&self, path: &NSString) -> bool;

        #[method(contentsEqualAtPath:andPath:)]
        pub unsafe fn contentsEqualAtPath_andPath(
            &self,
            path1: &NSString,
            path2: &NSString,
        ) -> bool;

        #[method_id(@__retain_semantics Other displayNameAtPath:)]
        pub unsafe fn displayNameAtPath(&self, path: &NSString) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other componentsToDisplayForPath:)]
        pub unsafe fn componentsToDisplayForPath(
            &self,
            path: &NSString,
        ) -> Option<Id<NSArray<NSString>, Shared>>;

        #[method_id(@__retain_semantics Other enumeratorAtPath:)]
        pub unsafe fn enumeratorAtPath(
            &self,
            path: &NSString,
        ) -> Option<Id<NSDirectoryEnumerator<NSString>, Shared>>;

        #[method_id(@__retain_semantics Other enumeratorAtURL:includingPropertiesForKeys:options:errorHandler:)]
        pub unsafe fn enumeratorAtURL_includingPropertiesForKeys_options_errorHandler(
            &self,
            url: &NSURL,
            keys: Option<&NSArray<NSURLResourceKey>>,
            mask: NSDirectoryEnumerationOptions,
            handler: Option<&Block<(NonNull<NSURL>, NonNull<NSError>), Bool>>,
        ) -> Option<Id<NSDirectoryEnumerator<NSURL>, Shared>>;

        #[method_id(@__retain_semantics Other subpathsAtPath:)]
        pub unsafe fn subpathsAtPath(
            &self,
            path: &NSString,
        ) -> Option<Id<NSArray<NSString>, Shared>>;

        #[method_id(@__retain_semantics Other contentsAtPath:)]
        pub unsafe fn contentsAtPath(&self, path: &NSString) -> Option<Id<NSData, Shared>>;

        #[method(createFileAtPath:contents:attributes:)]
        pub unsafe fn createFileAtPath_contents_attributes(
            &self,
            path: &NSString,
            data: Option<&NSData>,
            attr: Option<&NSDictionary<NSFileAttributeKey, Object>>,
        ) -> bool;

        #[method(fileSystemRepresentationWithPath:)]
        pub unsafe fn fileSystemRepresentationWithPath(&self, path: &NSString) -> NonNull<c_char>;

        #[method_id(@__retain_semantics Other stringWithFileSystemRepresentation:length:)]
        pub unsafe fn stringWithFileSystemRepresentation_length(
            &self,
            str: NonNull<c_char>,
            len: NSUInteger,
        ) -> Id<NSString, Shared>;

        #[method(replaceItemAtURL:withItemAtURL:backupItemName:options:resultingItemURL:error:)]
        pub unsafe fn replaceItemAtURL_withItemAtURL_backupItemName_options_resultingItemURL_error(
            &self,
            originalItemURL: &NSURL,
            newItemURL: &NSURL,
            backupItemName: Option<&NSString>,
            options: NSFileManagerItemReplacementOptions,
            resultingURL: *mut *mut NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(setUbiquitous:itemAtURL:destinationURL:error:)]
        pub unsafe fn setUbiquitous_itemAtURL_destinationURL_error(
            &self,
            flag: bool,
            url: &NSURL,
            destinationURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(isUbiquitousItemAtURL:)]
        pub unsafe fn isUbiquitousItemAtURL(&self, url: &NSURL) -> bool;

        #[method(startDownloadingUbiquitousItemAtURL:error:)]
        pub unsafe fn startDownloadingUbiquitousItemAtURL_error(
            &self,
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(evictUbiquitousItemAtURL:error:)]
        pub unsafe fn evictUbiquitousItemAtURL_error(
            &self,
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other URLForUbiquityContainerIdentifier:)]
        pub unsafe fn URLForUbiquityContainerIdentifier(
            &self,
            containerIdentifier: Option<&NSString>,
        ) -> Option<Id<NSURL, Shared>>;

        #[method_id(@__retain_semantics Other URLForPublishingUbiquitousItemAtURL:expirationDate:error:)]
        pub unsafe fn URLForPublishingUbiquitousItemAtURL_expirationDate_error(
            &self,
            url: &NSURL,
            outDate: *mut *mut NSDate,
        ) -> Result<Id<NSURL, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other ubiquityIdentityToken)]
        pub unsafe fn ubiquityIdentityToken(&self) -> Option<Id<TodoProtocols, Shared>>;

        #[method(getFileProviderServicesForItemAtURL:completionHandler:)]
        pub unsafe fn getFileProviderServicesForItemAtURL_completionHandler(
            &self,
            url: &NSURL,
            completionHandler: &Block<
                (
                    *mut NSDictionary<NSFileProviderServiceName, NSFileProviderService>,
                    *mut NSError,
                ),
                (),
            >,
        );

        #[method_id(@__retain_semantics Other containerURLForSecurityApplicationGroupIdentifier:)]
        pub unsafe fn containerURLForSecurityApplicationGroupIdentifier(
            &self,
            groupIdentifier: &NSString,
        ) -> Option<Id<NSURL, Shared>>;
    }
);

extern_methods!(
    /// NSUserInformation
    unsafe impl NSFileManager {
        #[method_id(@__retain_semantics Other homeDirectoryForCurrentUser)]
        pub unsafe fn homeDirectoryForCurrentUser(&self) -> Id<NSURL, Shared>;

        #[method_id(@__retain_semantics Other temporaryDirectory)]
        pub unsafe fn temporaryDirectory(&self) -> Id<NSURL, Shared>;

        #[method_id(@__retain_semantics Other homeDirectoryForUser:)]
        pub unsafe fn homeDirectoryForUser(&self, userName: &NSString)
            -> Option<Id<NSURL, Shared>>;
    }
);

extern_protocol!(
    pub struct NSFileManagerDelegate;

    unsafe impl ProtocolType for NSFileManagerDelegate {
        #[optional]
        #[method(fileManager:shouldCopyItemAtPath:toPath:)]
        pub unsafe fn fileManager_shouldCopyItemAtPath_toPath(
            &self,
            fileManager: &NSFileManager,
            srcPath: &NSString,
            dstPath: &NSString,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldCopyItemAtURL:toURL:)]
        pub unsafe fn fileManager_shouldCopyItemAtURL_toURL(
            &self,
            fileManager: &NSFileManager,
            srcURL: &NSURL,
            dstURL: &NSURL,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldProceedAfterError:copyingItemAtPath:toPath:)]
        pub unsafe fn fileManager_shouldProceedAfterError_copyingItemAtPath_toPath(
            &self,
            fileManager: &NSFileManager,
            error: &NSError,
            srcPath: &NSString,
            dstPath: &NSString,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldProceedAfterError:copyingItemAtURL:toURL:)]
        pub unsafe fn fileManager_shouldProceedAfterError_copyingItemAtURL_toURL(
            &self,
            fileManager: &NSFileManager,
            error: &NSError,
            srcURL: &NSURL,
            dstURL: &NSURL,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldMoveItemAtPath:toPath:)]
        pub unsafe fn fileManager_shouldMoveItemAtPath_toPath(
            &self,
            fileManager: &NSFileManager,
            srcPath: &NSString,
            dstPath: &NSString,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldMoveItemAtURL:toURL:)]
        pub unsafe fn fileManager_shouldMoveItemAtURL_toURL(
            &self,
            fileManager: &NSFileManager,
            srcURL: &NSURL,
            dstURL: &NSURL,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldProceedAfterError:movingItemAtPath:toPath:)]
        pub unsafe fn fileManager_shouldProceedAfterError_movingItemAtPath_toPath(
            &self,
            fileManager: &NSFileManager,
            error: &NSError,
            srcPath: &NSString,
            dstPath: &NSString,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldProceedAfterError:movingItemAtURL:toURL:)]
        pub unsafe fn fileManager_shouldProceedAfterError_movingItemAtURL_toURL(
            &self,
            fileManager: &NSFileManager,
            error: &NSError,
            srcURL: &NSURL,
            dstURL: &NSURL,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldLinkItemAtPath:toPath:)]
        pub unsafe fn fileManager_shouldLinkItemAtPath_toPath(
            &self,
            fileManager: &NSFileManager,
            srcPath: &NSString,
            dstPath: &NSString,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldLinkItemAtURL:toURL:)]
        pub unsafe fn fileManager_shouldLinkItemAtURL_toURL(
            &self,
            fileManager: &NSFileManager,
            srcURL: &NSURL,
            dstURL: &NSURL,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldProceedAfterError:linkingItemAtPath:toPath:)]
        pub unsafe fn fileManager_shouldProceedAfterError_linkingItemAtPath_toPath(
            &self,
            fileManager: &NSFileManager,
            error: &NSError,
            srcPath: &NSString,
            dstPath: &NSString,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldProceedAfterError:linkingItemAtURL:toURL:)]
        pub unsafe fn fileManager_shouldProceedAfterError_linkingItemAtURL_toURL(
            &self,
            fileManager: &NSFileManager,
            error: &NSError,
            srcURL: &NSURL,
            dstURL: &NSURL,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldRemoveItemAtPath:)]
        pub unsafe fn fileManager_shouldRemoveItemAtPath(
            &self,
            fileManager: &NSFileManager,
            path: &NSString,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldRemoveItemAtURL:)]
        pub unsafe fn fileManager_shouldRemoveItemAtURL(
            &self,
            fileManager: &NSFileManager,
            URL: &NSURL,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldProceedAfterError:removingItemAtPath:)]
        pub unsafe fn fileManager_shouldProceedAfterError_removingItemAtPath(
            &self,
            fileManager: &NSFileManager,
            error: &NSError,
            path: &NSString,
        ) -> bool;

        #[optional]
        #[method(fileManager:shouldProceedAfterError:removingItemAtURL:)]
        pub unsafe fn fileManager_shouldProceedAfterError_removingItemAtURL(
            &self,
            fileManager: &NSFileManager,
            error: &NSError,
            URL: &NSURL,
        ) -> bool;
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDirectoryEnumerator<
        ObjectType: Message = Object,
        ObjectTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for NSDirectoryEnumerator<ObjectType, ObjectTypeOwnership>
    {
        #[inherits(NSObject)]
        type Super = NSEnumerator<ObjectType, ObjectTypeOwnership>;
    }
);

extern_methods!(
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSDirectoryEnumerator<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other fileAttributes)]
        pub unsafe fn fileAttributes(
            &self,
        ) -> Option<Id<NSDictionary<NSFileAttributeKey, Object>, Shared>>;

        #[method_id(@__retain_semantics Other directoryAttributes)]
        pub unsafe fn directoryAttributes(
            &self,
        ) -> Option<Id<NSDictionary<NSFileAttributeKey, Object>, Shared>>;

        #[method(isEnumeratingDirectoryPostOrder)]
        pub unsafe fn isEnumeratingDirectoryPostOrder(&self) -> bool;

        #[method(skipDescendents)]
        pub unsafe fn skipDescendents(&self);

        #[method(level)]
        pub unsafe fn level(&self) -> NSUInteger;

        #[method(skipDescendants)]
        pub unsafe fn skipDescendants(&self);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFileProviderService;

    unsafe impl ClassType for NSFileProviderService {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSFileProviderService {
        #[method(getFileProviderConnectionWithCompletionHandler:)]
        pub unsafe fn getFileProviderConnectionWithCompletionHandler(
            &self,
            completionHandler: &Block<(*mut NSXPCConnection, *mut NSError), ()>,
        );

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSFileProviderServiceName, Shared>;
    }
);

extern_static!(NSFileType: &'static NSFileAttributeKey);

extern_static!(NSFileTypeDirectory: &'static NSFileAttributeType);

extern_static!(NSFileTypeRegular: &'static NSFileAttributeType);

extern_static!(NSFileTypeSymbolicLink: &'static NSFileAttributeType);

extern_static!(NSFileTypeSocket: &'static NSFileAttributeType);

extern_static!(NSFileTypeCharacterSpecial: &'static NSFileAttributeType);

extern_static!(NSFileTypeBlockSpecial: &'static NSFileAttributeType);

extern_static!(NSFileTypeUnknown: &'static NSFileAttributeType);

extern_static!(NSFileSize: &'static NSFileAttributeKey);

extern_static!(NSFileModificationDate: &'static NSFileAttributeKey);

extern_static!(NSFileReferenceCount: &'static NSFileAttributeKey);

extern_static!(NSFileDeviceIdentifier: &'static NSFileAttributeKey);

extern_static!(NSFileOwnerAccountName: &'static NSFileAttributeKey);

extern_static!(NSFileGroupOwnerAccountName: &'static NSFileAttributeKey);

extern_static!(NSFilePosixPermissions: &'static NSFileAttributeKey);

extern_static!(NSFileSystemNumber: &'static NSFileAttributeKey);

extern_static!(NSFileSystemFileNumber: &'static NSFileAttributeKey);

extern_static!(NSFileExtensionHidden: &'static NSFileAttributeKey);

extern_static!(NSFileHFSCreatorCode: &'static NSFileAttributeKey);

extern_static!(NSFileHFSTypeCode: &'static NSFileAttributeKey);

extern_static!(NSFileImmutable: &'static NSFileAttributeKey);

extern_static!(NSFileAppendOnly: &'static NSFileAttributeKey);

extern_static!(NSFileCreationDate: &'static NSFileAttributeKey);

extern_static!(NSFileOwnerAccountID: &'static NSFileAttributeKey);

extern_static!(NSFileGroupOwnerAccountID: &'static NSFileAttributeKey);

extern_static!(NSFileBusy: &'static NSFileAttributeKey);

extern_static!(NSFileProtectionKey: &'static NSFileAttributeKey);

extern_static!(NSFileProtectionNone: &'static NSFileProtectionType);

extern_static!(NSFileProtectionComplete: &'static NSFileProtectionType);

extern_static!(NSFileProtectionCompleteUnlessOpen: &'static NSFileProtectionType);

extern_static!(NSFileProtectionCompleteUntilFirstUserAuthentication: &'static NSFileProtectionType);

extern_static!(NSFileSystemSize: &'static NSFileAttributeKey);

extern_static!(NSFileSystemFreeSize: &'static NSFileAttributeKey);

extern_static!(NSFileSystemNodes: &'static NSFileAttributeKey);

extern_static!(NSFileSystemFreeNodes: &'static NSFileAttributeKey);

extern_methods!(
    /// NSFileAttributes
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method(fileSize)]
        pub unsafe fn fileSize(&self) -> c_ulonglong;

        #[method_id(@__retain_semantics Other fileModificationDate)]
        pub unsafe fn fileModificationDate(&self) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other fileType)]
        pub unsafe fn fileType(&self) -> Option<Id<NSString, Shared>>;

        #[method(filePosixPermissions)]
        pub unsafe fn filePosixPermissions(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other fileOwnerAccountName)]
        pub unsafe fn fileOwnerAccountName(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other fileGroupOwnerAccountName)]
        pub unsafe fn fileGroupOwnerAccountName(&self) -> Option<Id<NSString, Shared>>;

        #[method(fileSystemNumber)]
        pub unsafe fn fileSystemNumber(&self) -> NSInteger;

        #[method(fileSystemFileNumber)]
        pub unsafe fn fileSystemFileNumber(&self) -> NSUInteger;

        #[method(fileExtensionHidden)]
        pub unsafe fn fileExtensionHidden(&self) -> bool;

        #[method(fileHFSCreatorCode)]
        pub unsafe fn fileHFSCreatorCode(&self) -> OSType;

        #[method(fileHFSTypeCode)]
        pub unsafe fn fileHFSTypeCode(&self) -> OSType;

        #[method(fileIsImmutable)]
        pub unsafe fn fileIsImmutable(&self) -> bool;

        #[method(fileIsAppendOnly)]
        pub unsafe fn fileIsAppendOnly(&self) -> bool;

        #[method_id(@__retain_semantics Other fileCreationDate)]
        pub unsafe fn fileCreationDate(&self) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other fileOwnerAccountID)]
        pub unsafe fn fileOwnerAccountID(&self) -> Option<Id<NSNumber, Shared>>;

        #[method_id(@__retain_semantics Other fileGroupOwnerAccountID)]
        pub unsafe fn fileGroupOwnerAccountID(&self) -> Option<Id<NSNumber, Shared>>;
    }
);
