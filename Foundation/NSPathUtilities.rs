//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_methods!(
    /// NSStringPathExtensions
    #[cfg(feature = "NSString")]
    unsafe impl NSString {
        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other pathWithComponents:)]
        pub unsafe fn pathWithComponents(components: &NSArray<NSString>) -> Id<NSString>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other pathComponents)]
        pub unsafe fn pathComponents(&self) -> Id<NSArray<NSString>>;

        #[method(isAbsolutePath)]
        pub unsafe fn isAbsolutePath(&self) -> bool;

        #[method_id(@__retain_semantics Other lastPathComponent)]
        pub unsafe fn lastPathComponent(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other stringByDeletingLastPathComponent)]
        pub unsafe fn stringByDeletingLastPathComponent(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other stringByAppendingPathComponent:)]
        pub fn stringByAppendingPathComponent(&self, str: &NSString) -> Id<NSString>;

        #[method_id(@__retain_semantics Other pathExtension)]
        pub unsafe fn pathExtension(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other stringByDeletingPathExtension)]
        pub unsafe fn stringByDeletingPathExtension(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other stringByAppendingPathExtension:)]
        pub unsafe fn stringByAppendingPathExtension(&self, str: &NSString)
            -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other stringByAbbreviatingWithTildeInPath)]
        pub unsafe fn stringByAbbreviatingWithTildeInPath(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other stringByExpandingTildeInPath)]
        pub unsafe fn stringByExpandingTildeInPath(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other stringByStandardizingPath)]
        pub unsafe fn stringByStandardizingPath(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other stringByResolvingSymlinksInPath)]
        pub unsafe fn stringByResolvingSymlinksInPath(&self) -> Id<NSString>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other stringsByAppendingPaths:)]
        pub unsafe fn stringsByAppendingPaths(
            &self,
            paths: &NSArray<NSString>,
        ) -> Id<NSArray<NSString>>;

        #[cfg(feature = "NSArray")]
        #[method(completePathIntoString:caseSensitive:matchesIntoArray:filterTypes:)]
        pub unsafe fn completePathIntoString_caseSensitive_matchesIntoArray_filterTypes(
            &self,
            output_name: Option<&mut Option<Id<NSString>>>,
            flag: bool,
            output_array: Option<&mut Option<Id<NSArray<NSString>>>>,
            filter_types: Option<&NSArray<NSString>>,
        ) -> NSUInteger;

        #[method(fileSystemRepresentation)]
        pub unsafe fn fileSystemRepresentation(&self) -> NonNull<c_char>;

        #[method(getFileSystemRepresentation:maxLength:)]
        pub unsafe fn getFileSystemRepresentation_maxLength(
            &self,
            cname: NonNull<c_char>,
            max: NSUInteger,
        ) -> bool;
    }
);

extern_methods!(
    /// NSArrayPathExtensions
    #[cfg(feature = "NSArray")]
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other pathsMatchingExtensions:)]
        pub unsafe fn pathsMatchingExtensions(
            &self,
            filter_types: &NSArray<NSString>,
        ) -> Id<NSArray<NSString>>;
    }
);

extern "C" {
    #[cfg(feature = "NSString")]
    pub fn NSUserName() -> NonNull<NSString>;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub fn NSFullUserName() -> NonNull<NSString>;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub fn NSHomeDirectory() -> NonNull<NSString>;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub fn NSHomeDirectoryForUser(user_name: Option<&NSString>) -> *mut NSString;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub fn NSTemporaryDirectory() -> NonNull<NSString>;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub fn NSOpenStepRootDirectory() -> NonNull<NSString>;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSearchPathDirectory(pub NSUInteger);
impl NSSearchPathDirectory {
    pub const NSApplicationDirectory: Self = Self(1);
    pub const NSDemoApplicationDirectory: Self = Self(2);
    pub const NSDeveloperApplicationDirectory: Self = Self(3);
    pub const NSAdminApplicationDirectory: Self = Self(4);
    pub const NSLibraryDirectory: Self = Self(5);
    pub const NSDeveloperDirectory: Self = Self(6);
    pub const NSUserDirectory: Self = Self(7);
    pub const NSDocumentationDirectory: Self = Self(8);
    pub const NSDocumentDirectory: Self = Self(9);
    pub const NSCoreServiceDirectory: Self = Self(10);
    pub const NSAutosavedInformationDirectory: Self = Self(11);
    pub const NSDesktopDirectory: Self = Self(12);
    pub const NSCachesDirectory: Self = Self(13);
    pub const NSApplicationSupportDirectory: Self = Self(14);
    pub const NSDownloadsDirectory: Self = Self(15);
    pub const NSInputMethodsDirectory: Self = Self(16);
    pub const NSMoviesDirectory: Self = Self(17);
    pub const NSMusicDirectory: Self = Self(18);
    pub const NSPicturesDirectory: Self = Self(19);
    pub const NSPrinterDescriptionDirectory: Self = Self(20);
    pub const NSSharedPublicDirectory: Self = Self(21);
    pub const NSPreferencePanesDirectory: Self = Self(22);
    pub const NSApplicationScriptsDirectory: Self = Self(23);
    pub const NSItemReplacementDirectory: Self = Self(99);
    pub const NSAllApplicationsDirectory: Self = Self(100);
    pub const NSAllLibrariesDirectory: Self = Self(101);
    pub const NSTrashDirectory: Self = Self(102);
}

unsafe impl Encode for NSSearchPathDirectory {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSSearchPathDirectory {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSearchPathDomainMask(pub NSUInteger);
impl NSSearchPathDomainMask {
    pub const NSUserDomainMask: Self = Self(1);
    pub const NSLocalDomainMask: Self = Self(2);
    pub const NSNetworkDomainMask: Self = Self(4);
    pub const NSSystemDomainMask: Self = Self(8);
    pub const NSAllDomainsMask: Self = Self(0x0ffff);
}

unsafe impl Encode for NSSearchPathDomainMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSSearchPathDomainMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    #[cfg(all(feature = "NSArray", feature = "NSString"))]
    pub fn NSSearchPathForDirectoriesInDomains(
        directory: NSSearchPathDirectory,
        domain_mask: NSSearchPathDomainMask,
        expand_tilde: Bool,
    ) -> NonNull<NSArray<NSString>>;
}
