//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSUserNotificationActivationType(pub NSInteger);
impl NSUserNotificationActivationType {
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    #[doc(alias = "NSUserNotificationActivationTypeNone")]
    pub const None: Self = Self(0);
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    #[doc(alias = "NSUserNotificationActivationTypeContentsClicked")]
    pub const ContentsClicked: Self = Self(1);
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    #[doc(alias = "NSUserNotificationActivationTypeActionButtonClicked")]
    pub const ActionButtonClicked: Self = Self(2);
    #[doc(alias = "NSUserNotificationActivationTypeReplied")]
    pub const Replied: Self = Self(3);
    #[doc(alias = "NSUserNotificationActivationTypeAdditionalActionClicked")]
    pub const AdditionalActionClicked: Self = Self(4);
}

unsafe impl Encode for NSUserNotificationActivationType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSUserNotificationActivationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    pub struct NSUserNotification;

    unsafe impl ClassType for NSUserNotification {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSUserNotification {}

unsafe impl NSObjectProtocol for NSUserNotification {}

extern_methods!(
    unsafe impl NSUserNotification {
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other informativeText)]
        pub unsafe fn informativeText(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setInformativeText:)]
        pub unsafe fn setInformativeText(&self, informative_text: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other actionButtonTitle)]
        pub unsafe fn actionButtonTitle(&self) -> Id<NSString>;

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setActionButtonTitle:)]
        pub unsafe fn setActionButtonTitle(&self, action_button_title: &NSString);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary<NSString, AnyObject>>);

        #[cfg(feature = "NSDate")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other deliveryDate)]
        pub unsafe fn deliveryDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "NSDate")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setDeliveryDate:)]
        pub unsafe fn setDeliveryDate(&self, delivery_date: Option<&NSDate>);

        #[cfg(feature = "NSTimeZone")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other deliveryTimeZone)]
        pub unsafe fn deliveryTimeZone(&self) -> Option<Id<NSTimeZone>>;

        #[cfg(feature = "NSTimeZone")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setDeliveryTimeZone:)]
        pub unsafe fn setDeliveryTimeZone(&self, delivery_time_zone: Option<&NSTimeZone>);

        #[cfg(feature = "NSCalendar")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other deliveryRepeatInterval)]
        pub unsafe fn deliveryRepeatInterval(&self) -> Option<Id<NSDateComponents>>;

        #[cfg(feature = "NSCalendar")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setDeliveryRepeatInterval:)]
        pub unsafe fn setDeliveryRepeatInterval(
            &self,
            delivery_repeat_interval: Option<&NSDateComponents>,
        );

        #[cfg(feature = "NSDate")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other actualDeliveryDate)]
        pub unsafe fn actualDeliveryDate(&self) -> Option<Id<NSDate>>;

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(isPresented)]
        pub unsafe fn isPresented(&self) -> bool;

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(isRemote)]
        pub unsafe fn isRemote(&self) -> bool;

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other soundName)]
        pub unsafe fn soundName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setSoundName:)]
        pub unsafe fn setSoundName(&self, sound_name: Option<&NSString>);

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(hasActionButton)]
        pub unsafe fn hasActionButton(&self) -> bool;

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setHasActionButton:)]
        pub unsafe fn setHasActionButton(&self, has_action_button: bool);

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(activationType)]
        pub unsafe fn activationType(&self) -> NSUserNotificationActivationType;

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other otherButtonTitle)]
        pub unsafe fn otherButtonTitle(&self) -> Id<NSString>;

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setOtherButtonTitle:)]
        pub unsafe fn setOtherButtonTitle(&self, other_button_title: &NSString);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[method(hasReplyButton)]
        pub unsafe fn hasReplyButton(&self) -> bool;

        #[method(setHasReplyButton:)]
        pub unsafe fn setHasReplyButton(&self, has_reply_button: bool);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other responsePlaceholder)]
        pub unsafe fn responsePlaceholder(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setResponsePlaceholder:)]
        pub unsafe fn setResponsePlaceholder(&self, response_placeholder: Option<&NSString>);

        #[cfg(feature = "NSAttributedString")]
        #[method_id(@__retain_semantics Other response)]
        pub unsafe fn response(&self) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other additionalActions)]
        pub unsafe fn additionalActions(&self) -> Option<Id<NSArray<NSUserNotificationAction>>>;

        #[cfg(feature = "NSArray")]
        #[method(setAdditionalActions:)]
        pub unsafe fn setAdditionalActions(
            &self,
            additional_actions: Option<&NSArray<NSUserNotificationAction>>,
        );

        #[method_id(@__retain_semantics Other additionalActivationAction)]
        pub unsafe fn additionalActivationAction(&self) -> Option<Id<NSUserNotificationAction>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUserNotification {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    pub struct NSUserNotificationAction;

    unsafe impl ClassType for NSUserNotificationAction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSUserNotificationAction {}

unsafe impl NSObjectProtocol for NSUserNotificationAction {}

extern_methods!(
    unsafe impl NSUserNotificationAction {
        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:)]
        pub unsafe fn actionWithIdentifier_title(
            identifier: Option<&NSString>,
            title: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUserNotificationAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSUserNotificationDefaultSoundName: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
    pub struct NSUserNotificationCenter;

    unsafe impl ClassType for NSUserNotificationCenter {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSUserNotificationCenter {}

extern_methods!(
    unsafe impl NSUserNotificationCenter {
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other defaultUserNotificationCenter)]
        pub unsafe fn defaultUserNotificationCenter() -> Id<NSUserNotificationCenter>;

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSUserNotificationCenterDelegate>>>;

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSUserNotificationCenterDelegate>>,
        );

        #[cfg(feature = "NSArray")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other scheduledNotifications)]
        pub unsafe fn scheduledNotifications(&self) -> Id<NSArray<NSUserNotification>>;

        #[cfg(feature = "NSArray")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(setScheduledNotifications:)]
        pub unsafe fn setScheduledNotifications(
            &self,
            scheduled_notifications: &NSArray<NSUserNotification>,
        );

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(scheduleNotification:)]
        pub unsafe fn scheduleNotification(&self, notification: &NSUserNotification);

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(removeScheduledNotification:)]
        pub unsafe fn removeScheduledNotification(&self, notification: &NSUserNotification);

        #[cfg(feature = "NSArray")]
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method_id(@__retain_semantics Other deliveredNotifications)]
        pub unsafe fn deliveredNotifications(&self) -> Id<NSArray<NSUserNotification>>;

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(deliverNotification:)]
        pub unsafe fn deliverNotification(&self, notification: &NSUserNotification);

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(removeDeliveredNotification:)]
        pub unsafe fn removeDeliveredNotification(&self, notification: &NSUserNotification);

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[method(removeAllDeliveredNotifications)]
        pub unsafe fn removeAllDeliveredNotifications(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSUserNotificationCenter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSUserNotificationCenterDelegate: NSObjectProtocol {
        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[optional]
        #[method(userNotificationCenter:didDeliverNotification:)]
        unsafe fn userNotificationCenter_didDeliverNotification(
            &self,
            center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        );

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[optional]
        #[method(userNotificationCenter:didActivateNotification:)]
        unsafe fn userNotificationCenter_didActivateNotification(
            &self,
            center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        );

        #[deprecated = "All NSUserNotifications API should be replaced with UserNotifications.frameworks API"]
        #[optional]
        #[method(userNotificationCenter:shouldPresentNotification:)]
        unsafe fn userNotificationCenter_shouldPresentNotification(
            &self,
            center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSUserNotificationCenterDelegate {}
);
