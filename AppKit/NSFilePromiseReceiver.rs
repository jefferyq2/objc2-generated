//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSFilePromiseReceiver")]
    pub struct NSFilePromiseReceiver;

    #[cfg(feature = "AppKit_NSFilePromiseReceiver")]
    unsafe impl ClassType for NSFilePromiseReceiver {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSFilePromiseReceiver")]
unsafe impl NSObjectProtocol for NSFilePromiseReceiver {}

#[cfg(feature = "AppKit_NSFilePromiseReceiver")]
unsafe impl NSPasteboardReading for NSFilePromiseReceiver {}

extern_methods!(
    #[cfg(feature = "AppKit_NSFilePromiseReceiver")]
    unsafe impl NSFilePromiseReceiver {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other readableDraggedTypes)]
        pub unsafe fn readableDraggedTypes() -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other fileTypes)]
        pub unsafe fn fileTypes(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other fileNames)]
        pub unsafe fn fileNames(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSOperationQueue",
            feature = "Foundation_NSURL"
        ))]
        #[method(receivePromisedFilesAtDestination:options:operationQueue:reader:)]
        pub unsafe fn receivePromisedFilesAtDestination_options_operationQueue_reader(
            &self,
            destination_dir: &NSURL,
            options: &NSDictionary,
            operation_queue: &NSOperationQueue,
            reader: &Block<(NonNull<NSURL>, *mut NSError), ()>,
        );
    }
);
