//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSAnimationCurve {
        NSAnimationEaseInOut = 0,
        NSAnimationEaseIn = 1,
        NSAnimationEaseOut = 2,
        NSAnimationLinear = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSAnimationBlockingMode {
        NSAnimationBlocking = 0,
        NSAnimationNonblocking = 1,
        NSAnimationNonblockingThreaded = 2,
    }
);

pub type NSAnimationProgress = c_float;

extern_static!(NSAnimationProgressMarkNotification: &'static NSNotificationName);

extern_static!(NSAnimationProgressMark: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAnimation;

    unsafe impl ClassType for NSAnimation {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAnimation {
        #[method_id(@__retain_semantics Init initWithDuration:animationCurve:)]
        pub unsafe fn initWithDuration_animationCurve(
            this: Option<Allocated<Self>>,
            duration: NSTimeInterval,
            animationCurve: NSAnimationCurve,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method(startAnimation)]
        pub unsafe fn startAnimation(&self);

        #[method(stopAnimation)]
        pub unsafe fn stopAnimation(&self);

        #[method(isAnimating)]
        pub unsafe fn isAnimating(&self) -> bool;

        #[method(currentProgress)]
        pub unsafe fn currentProgress(&self) -> NSAnimationProgress;

        #[method(setCurrentProgress:)]
        pub unsafe fn setCurrentProgress(&self, currentProgress: NSAnimationProgress);

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method(setDuration:)]
        pub unsafe fn setDuration(&self, duration: NSTimeInterval);

        #[method(animationBlockingMode)]
        pub unsafe fn animationBlockingMode(&self) -> NSAnimationBlockingMode;

        #[method(setAnimationBlockingMode:)]
        pub unsafe fn setAnimationBlockingMode(
            &self,
            animationBlockingMode: NSAnimationBlockingMode,
        );

        #[method(frameRate)]
        pub unsafe fn frameRate(&self) -> c_float;

        #[method(setFrameRate:)]
        pub unsafe fn setFrameRate(&self, frameRate: c_float);

        #[method(animationCurve)]
        pub unsafe fn animationCurve(&self) -> NSAnimationCurve;

        #[method(setAnimationCurve:)]
        pub unsafe fn setAnimationCurve(&self, animationCurve: NSAnimationCurve);

        #[method(currentValue)]
        pub unsafe fn currentValue(&self) -> c_float;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSAnimationDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSAnimationDelegate>);

        #[method_id(@__retain_semantics Other progressMarks)]
        pub unsafe fn progressMarks(&self) -> Id<NSArray<NSNumber>, Shared>;

        #[method(setProgressMarks:)]
        pub unsafe fn setProgressMarks(&self, progressMarks: &NSArray<NSNumber>);

        #[method(addProgressMark:)]
        pub unsafe fn addProgressMark(&self, progressMark: NSAnimationProgress);

        #[method(removeProgressMark:)]
        pub unsafe fn removeProgressMark(&self, progressMark: NSAnimationProgress);

        #[method(startWhenAnimation:reachesProgress:)]
        pub unsafe fn startWhenAnimation_reachesProgress(
            &self,
            animation: &NSAnimation,
            startProgress: NSAnimationProgress,
        );

        #[method(stopWhenAnimation:reachesProgress:)]
        pub unsafe fn stopWhenAnimation_reachesProgress(
            &self,
            animation: &NSAnimation,
            stopProgress: NSAnimationProgress,
        );

        #[method(clearStartAnimation)]
        pub unsafe fn clearStartAnimation(&self);

        #[method(clearStopAnimation)]
        pub unsafe fn clearStopAnimation(&self);

        #[method_id(@__retain_semantics Other runLoopModesForAnimating)]
        pub unsafe fn runLoopModesForAnimating(&self)
            -> Option<Id<NSArray<NSRunLoopMode>, Shared>>;
    }
);

extern_protocol!(
    pub struct NSAnimationDelegate;

    unsafe impl ProtocolType for NSAnimationDelegate {
        #[optional]
        #[method(animationShouldStart:)]
        pub unsafe fn animationShouldStart(&self, animation: &NSAnimation) -> bool;

        #[optional]
        #[method(animationDidStop:)]
        pub unsafe fn animationDidStop(&self, animation: &NSAnimation);

        #[optional]
        #[method(animationDidEnd:)]
        pub unsafe fn animationDidEnd(&self, animation: &NSAnimation);

        #[optional]
        #[method(animation:valueForProgress:)]
        pub unsafe fn animation_valueForProgress(
            &self,
            animation: &NSAnimation,
            progress: NSAnimationProgress,
        ) -> c_float;

        #[optional]
        #[method(animation:didReachProgressMark:)]
        pub unsafe fn animation_didReachProgressMark(
            &self,
            animation: &NSAnimation,
            progress: NSAnimationProgress,
        );
    }
);

pub type NSViewAnimationKey = NSString;

extern_static!(NSViewAnimationTargetKey: &'static NSViewAnimationKey);

extern_static!(NSViewAnimationStartFrameKey: &'static NSViewAnimationKey);

extern_static!(NSViewAnimationEndFrameKey: &'static NSViewAnimationKey);

extern_static!(NSViewAnimationEffectKey: &'static NSViewAnimationKey);

pub type NSViewAnimationEffectName = NSString;

extern_static!(NSViewAnimationFadeInEffect: &'static NSViewAnimationEffectName);

extern_static!(NSViewAnimationFadeOutEffect: &'static NSViewAnimationEffectName);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSViewAnimation;

    unsafe impl ClassType for NSViewAnimation {
        #[inherits(NSObject)]
        type Super = NSAnimation;
    }
);

extern_methods!(
    unsafe impl NSViewAnimation {
        #[method_id(@__retain_semantics Init initWithViewAnimations:)]
        pub unsafe fn initWithViewAnimations(
            this: Option<Allocated<Self>>,
            viewAnimations: &NSArray<NSDictionary<NSViewAnimationKey, Object>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other viewAnimations)]
        pub unsafe fn viewAnimations(
            &self,
        ) -> Id<NSArray<NSDictionary<NSViewAnimationKey, Object>>, Shared>;

        #[method(setViewAnimations:)]
        pub unsafe fn setViewAnimations(
            &self,
            viewAnimations: &NSArray<NSDictionary<NSViewAnimationKey, Object>>,
        );
    }
);

pub type NSAnimatablePropertyKey = NSString;

extern_protocol!(
    pub struct NSAnimatablePropertyContainer;

    unsafe impl ProtocolType for NSAnimatablePropertyContainer {
        #[method_id(@__retain_semantics Other animator)]
        pub unsafe fn animator(&self) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other animations)]
        pub unsafe fn animations(
            &self,
        ) -> Id<NSDictionary<NSAnimatablePropertyKey, Object>, Shared>;

        #[method(setAnimations:)]
        pub unsafe fn setAnimations(
            &self,
            animations: &NSDictionary<NSAnimatablePropertyKey, Object>,
        );

        #[method_id(@__retain_semantics Other animationForKey:)]
        pub unsafe fn animationForKey(
            &self,
            key: &NSAnimatablePropertyKey,
        ) -> Option<Id<Object, Shared>>;
    }
);

extern_static!(NSAnimationTriggerOrderIn: &'static NSAnimatablePropertyKey);

extern_static!(NSAnimationTriggerOrderOut: &'static NSAnimatablePropertyKey);
