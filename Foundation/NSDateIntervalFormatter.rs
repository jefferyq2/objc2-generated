//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDateIntervalFormatterStyle(pub NSUInteger);
impl NSDateIntervalFormatterStyle {
    pub const NSDateIntervalFormatterNoStyle: Self = Self(0);
    pub const NSDateIntervalFormatterShortStyle: Self = Self(1);
    pub const NSDateIntervalFormatterMediumStyle: Self = Self(2);
    pub const NSDateIntervalFormatterLongStyle: Self = Self(3);
    pub const NSDateIntervalFormatterFullStyle: Self = Self(4);
}

unsafe impl Encode for NSDateIntervalFormatterStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDateIntervalFormatterStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSFormatter")]
    pub struct NSDateIntervalFormatter;

    #[cfg(feature = "NSFormatter")]
    unsafe impl ClassType for NSDateIntervalFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSFormatter")]
unsafe impl Send for NSDateIntervalFormatter {}

#[cfg(feature = "NSFormatter")]
unsafe impl Sync for NSDateIntervalFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCoding for NSDateIntervalFormatter {}

#[cfg(all(feature = "NSFormatter", feature = "NSObject"))]
unsafe impl NSCopying for NSDateIntervalFormatter {}

#[cfg(feature = "NSFormatter")]
unsafe impl NSObjectProtocol for NSDateIntervalFormatter {}

extern_methods!(
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSDateIntervalFormatter {
        #[cfg(feature = "NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[cfg(feature = "NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(feature = "NSCalendar")]
        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Id<NSCalendar>;

        #[cfg(feature = "NSCalendar")]
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[cfg(feature = "NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone>;

        #[cfg(feature = "NSTimeZone")]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other dateTemplate)]
        pub unsafe fn dateTemplate(&self) -> Id<NSString>;

        #[cfg(feature = "NSString")]
        #[method(setDateTemplate:)]
        pub unsafe fn setDateTemplate(&self, date_template: Option<&NSString>);

        #[method(dateStyle)]
        pub unsafe fn dateStyle(&self) -> NSDateIntervalFormatterStyle;

        #[method(setDateStyle:)]
        pub unsafe fn setDateStyle(&self, date_style: NSDateIntervalFormatterStyle);

        #[method(timeStyle)]
        pub unsafe fn timeStyle(&self) -> NSDateIntervalFormatterStyle;

        #[method(setTimeStyle:)]
        pub unsafe fn setTimeStyle(&self, time_style: NSDateIntervalFormatterStyle);

        #[cfg(all(feature = "NSDate", feature = "NSString"))]
        #[method_id(@__retain_semantics Other stringFromDate:toDate:)]
        pub unsafe fn stringFromDate_toDate(
            &self,
            from_date: &NSDate,
            to_date: &NSDate,
        ) -> Id<NSString>;

        #[cfg(all(feature = "NSDateInterval", feature = "NSString"))]
        #[method_id(@__retain_semantics Other stringFromDateInterval:)]
        pub unsafe fn stringFromDateInterval(
            &self,
            date_interval: &NSDateInterval,
        ) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSFormatter")]
    unsafe impl NSDateIntervalFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
