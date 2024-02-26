//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSPostingStyle {
        NSPostWhenIdle = 1,
        NSPostASAP = 2,
        NSPostNow = 3,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSNotificationCoalescing {
        NSNotificationNoCoalescing = 0,
        #[doc(alias = "NSNotificationCoalescingOnName")]
        OnName = 1,
        #[doc(alias = "NSNotificationCoalescingOnSender")]
        OnSender = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSNotificationQueue")]
    pub struct NSNotificationQueue;

    #[cfg(feature = "Foundation_NSNotificationQueue")]
    unsafe impl ClassType for NSNotificationQueue {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSNotificationQueue")]
unsafe impl NSObjectProtocol for NSNotificationQueue {}

extern_methods!(
    #[cfg(feature = "Foundation_NSNotificationQueue")]
    unsafe impl NSNotificationQueue {
        #[method_id(@__retain_semantics Other defaultQueue)]
        pub unsafe fn defaultQueue() -> Id<NSNotificationQueue>;

        #[cfg(feature = "Foundation_NSNotificationCenter")]
        #[method_id(@__retain_semantics Init initWithNotificationCenter:)]
        pub unsafe fn initWithNotificationCenter(
            this: Allocated<Self>,
            notification_center: &NSNotificationCenter,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(enqueueNotification:postingStyle:)]
        pub unsafe fn enqueueNotification_postingStyle(
            &self,
            notification: &NSNotification,
            posting_style: NSPostingStyle,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSNotification",
            feature = "Foundation_NSString"
        ))]
        #[method(enqueueNotification:postingStyle:coalesceMask:forModes:)]
        pub unsafe fn enqueueNotification_postingStyle_coalesceMask_forModes(
            &self,
            notification: &NSNotification,
            posting_style: NSPostingStyle,
            coalesce_mask: NSNotificationCoalescing,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(dequeueNotificationsMatching:coalesceMask:)]
        pub unsafe fn dequeueNotificationsMatching_coalesceMask(
            &self,
            notification: &NSNotification,
            coalesce_mask: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSNotificationQueue")]
    unsafe impl NSNotificationQueue {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
