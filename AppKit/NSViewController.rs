//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSViewControllerTransitionOptions {
        NSViewControllerTransitionNone = 0x0,
        NSViewControllerTransitionCrossfade = 0x1,
        NSViewControllerTransitionSlideUp = 0x10,
        NSViewControllerTransitionSlideDown = 0x20,
        NSViewControllerTransitionSlideLeft = 0x40,
        NSViewControllerTransitionSlideRight = 0x80,
        NSViewControllerTransitionSlideForward = 0x140,
        NSViewControllerTransitionSlideBackward = 0x180,
        NSViewControllerTransitionAllowUserInteraction = 0x1000,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSViewController")]
    pub struct NSViewController;

    #[cfg(feature = "AppKit_NSViewController")]
    unsafe impl ClassType for NSViewController {
        #[inherits(NSObject)]
        type Super = NSResponder;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSViewController")]
unsafe impl NSCoding for NSViewController {}

#[cfg(feature = "AppKit_NSViewController")]
unsafe impl NSEditor for NSViewController {}

#[cfg(feature = "AppKit_NSViewController")]
unsafe impl NSObjectProtocol for NSViewController {}

#[cfg(feature = "AppKit_NSViewController")]
unsafe impl NSSeguePerforming for NSViewController {}

#[cfg(feature = "AppKit_NSViewController")]
unsafe impl NSUserInterfaceItemIdentification for NSViewController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSViewController")]
    unsafe impl NSViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other nibName)]
        pub unsafe fn nibName(&self) -> Option<Id<NSNibName>>;

        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Other nibBundle)]
        pub unsafe fn nibBundle(&self) -> Option<Id<NSBundle>>;

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<AnyObject>>;

        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, represented_object: Option<&AnyObject>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Id<NSView>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: &NSView);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other viewIfLoaded)]
        pub unsafe fn viewIfLoaded(&self) -> Option<Id<NSView>>;

        #[method(loadView)]
        pub unsafe fn loadView(&self);

        #[method(loadViewIfNeeded)]
        pub unsafe fn loadViewIfNeeded(&self);

        #[method(commitEditingWithDelegate:didCommitSelector:contextInfo:)]
        pub unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo(
            &self,
            delegate: Option<&AnyObject>,
            did_commit_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(commitEditing)]
        pub unsafe fn commitEditing(&self) -> bool;

        #[method(discardEditing)]
        pub unsafe fn discardEditing(&self);

        #[method(viewDidLoad)]
        pub unsafe fn viewDidLoad(&self);

        #[method(isViewLoaded)]
        pub unsafe fn isViewLoaded(&self) -> bool;

        #[method(viewWillAppear)]
        pub unsafe fn viewWillAppear(&self);

        #[method(viewDidAppear)]
        pub unsafe fn viewDidAppear(&self);

        #[method(viewWillDisappear)]
        pub unsafe fn viewWillDisappear(&self);

        #[method(viewDidDisappear)]
        pub unsafe fn viewDidDisappear(&self);

        #[method(preferredContentSize)]
        pub unsafe fn preferredContentSize(&self) -> NSSize;

        #[method(setPreferredContentSize:)]
        pub unsafe fn setPreferredContentSize(&self, preferred_content_size: NSSize);

        #[method(updateViewConstraints)]
        pub unsafe fn updateViewConstraints(&self);

        #[method(viewWillLayout)]
        pub unsafe fn viewWillLayout(&self);

        #[method(viewDidLayout)]
        pub unsafe fn viewDidLayout(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSViewController")]
    unsafe impl NSViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSViewController")]
    unsafe impl NSViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// NSViewControllerPresentation
    #[cfg(feature = "AppKit_NSViewController")]
    unsafe impl NSViewController {
        #[method(presentViewController:animator:)]
        pub unsafe fn presentViewController_animator(
            &self,
            view_controller: &NSViewController,
            animator: &ProtocolObject<dyn NSViewControllerPresentationAnimator>,
        );

        #[method(dismissViewController:)]
        pub unsafe fn dismissViewController(&self, view_controller: &NSViewController);

        #[method(dismissController:)]
        pub unsafe fn dismissController(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other presentedViewControllers)]
        pub unsafe fn presentedViewControllers(&self) -> Option<Id<NSArray<NSViewController>>>;

        #[method_id(@__retain_semantics Other presentingViewController)]
        pub unsafe fn presentingViewController(&self) -> Option<Id<NSViewController>>;
    }
);

extern_methods!(
    /// NSViewControllerPresentationAndTransitionStyles
    #[cfg(feature = "AppKit_NSViewController")]
    unsafe impl NSViewController {
        #[method(presentViewControllerAsSheet:)]
        pub unsafe fn presentViewControllerAsSheet(&self, view_controller: &NSViewController);

        #[method(presentViewControllerAsModalWindow:)]
        pub unsafe fn presentViewControllerAsModalWindow(&self, view_controller: &NSViewController);

        #[cfg(feature = "AppKit_NSView")]
        #[method(presentViewController:asPopoverRelativeToRect:ofView:preferredEdge:behavior:)]
        pub unsafe fn presentViewController_asPopoverRelativeToRect_ofView_preferredEdge_behavior(
            &self,
            view_controller: &NSViewController,
            positioning_rect: NSRect,
            positioning_view: &NSView,
            preferred_edge: NSRectEdge,
            behavior: NSPopoverBehavior,
        );

        #[cfg(feature = "AppKit_NSView")]
        #[method(presentViewController:asPopoverRelativeToRect:ofView:preferredEdge:behavior:hasFullSizeContent:)]
        pub unsafe fn presentViewController_asPopoverRelativeToRect_ofView_preferredEdge_behavior_hasFullSizeContent(
            &self,
            view_controller: &NSViewController,
            positioning_rect: NSRect,
            positioning_view: &NSView,
            preferred_edge: NSRectEdge,
            behavior: NSPopoverBehavior,
            has_full_size_content: bool,
        );

        #[method(transitionFromViewController:toViewController:options:completionHandler:)]
        pub unsafe fn transitionFromViewController_toViewController_options_completionHandler(
            &self,
            from_view_controller: &NSViewController,
            to_view_controller: &NSViewController,
            options: NSViewControllerTransitionOptions,
            completion: Option<&Block<dyn Fn()>>,
        );
    }
);

extern_methods!(
    /// NSViewControllerContainer
    #[cfg(feature = "AppKit_NSViewController")]
    unsafe impl NSViewController {
        #[method_id(@__retain_semantics Other parentViewController)]
        pub unsafe fn parentViewController(&self) -> Option<Id<NSViewController>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other childViewControllers)]
        pub unsafe fn childViewControllers(&self) -> Id<NSArray<NSViewController>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setChildViewControllers:)]
        pub unsafe fn setChildViewControllers(
            &self,
            child_view_controllers: &NSArray<NSViewController>,
        );

        #[method(addChildViewController:)]
        pub unsafe fn addChildViewController(&self, child_view_controller: &NSViewController);

        #[method(removeFromParentViewController)]
        pub unsafe fn removeFromParentViewController(&self);

        #[method(insertChildViewController:atIndex:)]
        pub unsafe fn insertChildViewController_atIndex(
            &self,
            child_view_controller: &NSViewController,
            index: NSInteger,
        );

        #[method(removeChildViewControllerAtIndex:)]
        pub unsafe fn removeChildViewControllerAtIndex(&self, index: NSInteger);

        #[method(preferredContentSizeDidChangeForViewController:)]
        pub unsafe fn preferredContentSizeDidChangeForViewController(
            &self,
            view_controller: &NSViewController,
        );

        #[method(viewWillTransitionToSize:)]
        pub unsafe fn viewWillTransitionToSize(&self, new_size: NSSize);
    }
);

extern_protocol!(
    pub unsafe trait NSViewControllerPresentationAnimator:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[cfg(feature = "AppKit_NSViewController")]
        #[method(animatePresentationOfViewController:fromViewController:)]
        unsafe fn animatePresentationOfViewController_fromViewController(
            &self,
            view_controller: &NSViewController,
            from_view_controller: &NSViewController,
        );

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(animateDismissalOfViewController:fromViewController:)]
        unsafe fn animateDismissalOfViewController_fromViewController(
            &self,
            view_controller: &NSViewController,
            from_view_controller: &NSViewController,
        );
    }

    unsafe impl ProtocolType for dyn NSViewControllerPresentationAnimator {}
);

extern_methods!(
    /// NSViewControllerStoryboardingMethods
    #[cfg(feature = "AppKit_NSViewController")]
    unsafe impl NSViewController {
        #[cfg(feature = "AppKit_NSStoryboard")]
        #[method_id(@__retain_semantics Other storyboard)]
        pub unsafe fn storyboard(&self) -> Option<Id<NSStoryboard>>;
    }
);

extern_methods!(
    /// NSExtensionAdditions
    #[cfg(feature = "AppKit_NSViewController")]
    unsafe impl NSViewController {
        #[cfg(feature = "Foundation_NSExtensionContext")]
        #[method_id(@__retain_semantics Other extensionContext)]
        pub unsafe fn extensionContext(&self) -> Option<Id<NSExtensionContext>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other sourceItemView)]
        pub unsafe fn sourceItemView(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setSourceItemView:)]
        pub unsafe fn setSourceItemView(&self, source_item_view: Option<&NSView>);

        #[method(preferredScreenOrigin)]
        pub unsafe fn preferredScreenOrigin(&self) -> NSPoint;

        #[method(setPreferredScreenOrigin:)]
        pub unsafe fn setPreferredScreenOrigin(&self, preferred_screen_origin: NSPoint);

        #[method(preferredMinimumSize)]
        pub unsafe fn preferredMinimumSize(&self) -> NSSize;

        #[method(preferredMaximumSize)]
        pub unsafe fn preferredMaximumSize(&self) -> NSSize;
    }
);

#[cfg(feature = "AppKit_NSViewController")]
unsafe impl NSExtensionRequestHandling for NSViewController {}
