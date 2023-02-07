//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNNotificationActionIcon")]
    pub struct UNNotificationActionIcon;

    #[cfg(feature = "UserNotifications_UNNotificationActionIcon")]
    unsafe impl ClassType for UNNotificationActionIcon {
        type Super = NSObject;
    }
);

#[cfg(feature = "UserNotifications_UNNotificationActionIcon")]
unsafe impl NSCoding for UNNotificationActionIcon {}

#[cfg(feature = "UserNotifications_UNNotificationActionIcon")]
unsafe impl NSObjectProtocol for UNNotificationActionIcon {}

#[cfg(feature = "UserNotifications_UNNotificationActionIcon")]
unsafe impl NSSecureCoding for UNNotificationActionIcon {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNNotificationActionIcon")]
    unsafe impl UNNotificationActionIcon {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other iconWithTemplateImageName:)]
        pub unsafe fn iconWithTemplateImageName(template_image_name: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other iconWithSystemImageName:)]
        pub unsafe fn iconWithSystemImageName(system_image_name: &NSString) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
