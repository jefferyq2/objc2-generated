//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ClassKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ClassKit_CLSActivity")]
    pub struct CLSActivity;

    #[cfg(feature = "ClassKit_CLSActivity")]
    unsafe impl ClassType for CLSActivity {
        #[inherits(NSObject)]
        type Super = CLSObject;
    }
);

extern_methods!(
    #[cfg(feature = "ClassKit_CLSActivity")]
    unsafe impl CLSActivity {
        #[method(progress)]
        pub unsafe fn progress(&self) -> c_double;

        #[method(setProgress:)]
        pub unsafe fn setProgress(&self, progress: c_double);

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[cfg(feature = "ClassKit_CLSActivityItem")]
        #[method_id(@__retain_semantics Other primaryActivityItem)]
        pub unsafe fn primaryActivityItem(&self) -> Option<Id<CLSActivityItem, Shared>>;

        #[cfg(feature = "ClassKit_CLSActivityItem")]
        #[method(setPrimaryActivityItem:)]
        pub unsafe fn setPrimaryActivityItem(
            &self,
            primary_activity_item: Option<&CLSActivityItem>,
        );

        #[method(addProgressRangeFromStart:toEnd:)]
        pub unsafe fn addProgressRangeFromStart_toEnd(&self, start: c_double, end: c_double);

        #[cfg(feature = "ClassKit_CLSActivityItem")]
        #[method(addAdditionalActivityItem:)]
        pub unsafe fn addAdditionalActivityItem(&self, activity_item: &CLSActivityItem);

        #[cfg(all(feature = "ClassKit_CLSActivityItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other additionalActivityItems)]
        pub unsafe fn additionalActivityItems(&self) -> Id<NSArray<CLSActivityItem>, Shared>;
    }
);

extern_methods!(
    /// Activation
    #[cfg(feature = "ClassKit_CLSActivity")]
    unsafe impl CLSActivity {
        #[method(isStarted)]
        pub unsafe fn isStarted(&self) -> bool;

        #[method(start)]
        pub unsafe fn start(&self);

        #[method(stop)]
        pub unsafe fn stop(&self);

        #[method(removeAllActivityItems)]
        pub unsafe fn removeAllActivityItems(&self);
    }
);

extern_methods!(
    /// Activity
    #[cfg(feature = "ClassKit_CLSContext")]
    unsafe impl CLSContext {
        #[cfg(feature = "ClassKit_CLSActivity")]
        #[method_id(@__retain_semantics Other currentActivity)]
        pub unsafe fn currentActivity(&self) -> Option<Id<CLSActivity, Shared>>;

        #[cfg(feature = "ClassKit_CLSActivity")]
        #[method_id(@__retain_semantics Other createNewActivity)]
        pub unsafe fn createNewActivity(&self) -> Id<CLSActivity, Shared>;
    }
);
