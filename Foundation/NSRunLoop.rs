//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSDefaultRunLoopMode: &'static NSRunLoopMode);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSRunLoopCommonModes: &'static NSRunLoopMode);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSRunLoop")]
    pub struct NSRunLoop;

    #[cfg(feature = "Foundation_NSRunLoop")]
    unsafe impl ClassType for NSRunLoop {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSRunLoop")]
unsafe impl NSObjectProtocol for NSRunLoop {}

extern_methods!(
    #[cfg(feature = "Foundation_NSRunLoop")]
    unsafe impl NSRunLoop {
        #[method_id(@__retain_semantics Other currentRunLoop)]
        pub unsafe fn currentRunLoop() -> Id<NSRunLoop>;

        #[method_id(@__retain_semantics Other mainRunLoop)]
        pub unsafe fn mainRunLoop() -> Id<NSRunLoop>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other currentMode)]
        pub unsafe fn currentMode(&self) -> Option<Id<NSRunLoopMode>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSTimer"))]
        #[method(addTimer:forMode:)]
        pub unsafe fn addTimer_forMode(&self, timer: &NSTimer, mode: &NSRunLoopMode);

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[method(addPort:forMode:)]
        pub unsafe fn addPort_forMode(&self, a_port: &NSPort, mode: &NSRunLoopMode);

        #[cfg(all(feature = "Foundation_NSPort", feature = "Foundation_NSString"))]
        #[method(removePort:forMode:)]
        pub unsafe fn removePort_forMode(&self, a_port: &NSPort, mode: &NSRunLoopMode);

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other limitDateForMode:)]
        pub unsafe fn limitDateForMode(&self, mode: &NSRunLoopMode) -> Option<Id<NSDate>>;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method(acceptInputForMode:beforeDate:)]
        pub unsafe fn acceptInputForMode_beforeDate(
            &self,
            mode: &NSRunLoopMode,
            limit_date: &NSDate,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSRunLoop")]
    unsafe impl NSRunLoop {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSRunLoopConveniences
    #[cfg(feature = "Foundation_NSRunLoop")]
    unsafe impl NSRunLoop {
        #[method(run)]
        pub unsafe fn run(&self);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(runUntilDate:)]
        pub unsafe fn runUntilDate(&self, limit_date: &NSDate);

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method(runMode:beforeDate:)]
        pub unsafe fn runMode_beforeDate(&self, mode: &NSRunLoopMode, limit_date: &NSDate) -> bool;

        #[deprecated = "Not supported"]
        #[method(configureAsServer)]
        pub unsafe fn configureAsServer(&self);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(performInModes:block:)]
        pub unsafe fn performInModes_block(
            &self,
            modes: &NSArray<NSRunLoopMode>,
            block: &Block<dyn Fn()>,
        );

        #[method(performBlock:)]
        pub unsafe fn performBlock(&self, block: &Block<dyn Fn()>);
    }
);

extern_category!(
    /// Category "NSDelayedPerforming" on [`NSObject`].
    #[doc(alias = "NSDelayedPerforming")]
    pub unsafe trait NSObjectNSDelayedPerforming {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(performSelector:withObject:afterDelay:inModes:)]
        unsafe fn performSelector_withObject_afterDelay_inModes(
            &self,
            a_selector: Sel,
            an_argument: Option<&AnyObject>,
            delay: NSTimeInterval,
            modes: &NSArray<NSRunLoopMode>,
        );

        #[method(performSelector:withObject:afterDelay:)]
        unsafe fn performSelector_withObject_afterDelay(
            &self,
            a_selector: Sel,
            an_argument: Option<&AnyObject>,
            delay: NSTimeInterval,
        );

        #[method(cancelPreviousPerformRequestsWithTarget:selector:object:)]
        unsafe fn cancelPreviousPerformRequestsWithTarget_selector_object(
            a_target: &AnyObject,
            a_selector: Sel,
            an_argument: Option<&AnyObject>,
        );

        #[method(cancelPreviousPerformRequestsWithTarget:)]
        unsafe fn cancelPreviousPerformRequestsWithTarget(a_target: &AnyObject);
    }

    unsafe impl NSObjectNSDelayedPerforming for NSObject {}
);

extern_methods!(
    /// NSOrderedPerform
    #[cfg(feature = "Foundation_NSRunLoop")]
    unsafe impl NSRunLoop {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(performSelector:target:argument:order:modes:)]
        pub unsafe fn performSelector_target_argument_order_modes(
            &self,
            a_selector: Sel,
            target: &AnyObject,
            arg: Option<&AnyObject>,
            order: NSUInteger,
            modes: &NSArray<NSRunLoopMode>,
        );

        #[method(cancelPerformSelector:target:argument:)]
        pub unsafe fn cancelPerformSelector_target_argument(
            &self,
            a_selector: Sel,
            target: &AnyObject,
            arg: Option<&AnyObject>,
        );

        #[method(cancelPerformSelectorsWithTarget:)]
        pub unsafe fn cancelPerformSelectorsWithTarget(&self, target: &AnyObject);
    }
);
