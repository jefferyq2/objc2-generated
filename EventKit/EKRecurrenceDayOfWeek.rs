//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct EKRecurrenceDayOfWeek;

    unsafe impl ClassType for EKRecurrenceDayOfWeek {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for EKRecurrenceDayOfWeek {}

unsafe impl NSCopying for EKRecurrenceDayOfWeek {}

unsafe impl NSObjectProtocol for EKRecurrenceDayOfWeek {}

unsafe impl NSSecureCoding for EKRecurrenceDayOfWeek {}

extern_methods!(
    unsafe impl EKRecurrenceDayOfWeek {
        #[cfg(feature = "EKTypes")]
        #[method_id(@__retain_semantics Other dayOfWeek:)]
        pub unsafe fn dayOfWeek(day_of_the_week: EKWeekday) -> Id<Self>;

        #[cfg(feature = "EKTypes")]
        #[method_id(@__retain_semantics Other dayOfWeek:weekNumber:)]
        pub unsafe fn dayOfWeek_weekNumber(
            day_of_the_week: EKWeekday,
            week_number: NSInteger,
        ) -> Id<Self>;

        #[cfg(feature = "EKTypes")]
        #[method_id(@__retain_semantics Init initWithDayOfTheWeek:weekNumber:)]
        pub unsafe fn initWithDayOfTheWeek_weekNumber(
            this: Allocated<Self>,
            day_of_the_week: EKWeekday,
            week_number: NSInteger,
        ) -> Id<Self>;

        #[cfg(feature = "EKTypes")]
        #[method(dayOfTheWeek)]
        pub unsafe fn dayOfTheWeek(&self) -> EKWeekday;

        #[method(weekNumber)]
        pub unsafe fn weekNumber(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl EKRecurrenceDayOfWeek {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
