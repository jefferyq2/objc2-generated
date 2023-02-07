//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSSplitViewItemBehavior {
        NSSplitViewItemBehaviorDefault = 0,
        NSSplitViewItemBehaviorSidebar = 1,
        NSSplitViewItemBehaviorContentList = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSSplitViewItemCollapseBehavior {
        NSSplitViewItemCollapseBehaviorDefault = 0,
        NSSplitViewItemCollapseBehaviorPreferResizingSplitViewWithFixedSiblings = 1,
        NSSplitViewItemCollapseBehaviorPreferResizingSiblingsWithFixedSplitView = 2,
        NSSplitViewItemCollapseBehaviorUseConstraints = 3,
    }
);

extern_static!(NSSplitViewItemUnspecifiedDimension: CGFloat);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSplitViewItem")]
    pub struct NSSplitViewItem;

    #[cfg(feature = "AppKit_NSSplitViewItem")]
    unsafe impl ClassType for NSSplitViewItem {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSSplitViewItem")]
unsafe impl NSAnimatablePropertyContainer for NSSplitViewItem {}

#[cfg(feature = "AppKit_NSSplitViewItem")]
unsafe impl NSCoding for NSSplitViewItem {}

#[cfg(feature = "AppKit_NSSplitViewItem")]
unsafe impl NSObjectProtocol for NSSplitViewItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSplitViewItem")]
    unsafe impl NSSplitViewItem {
        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other splitViewItemWithViewController:)]
        pub unsafe fn splitViewItemWithViewController(
            view_controller: &NSViewController,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other sidebarWithViewController:)]
        pub unsafe fn sidebarWithViewController(view_controller: &NSViewController) -> Id<Self>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other contentListWithViewController:)]
        pub unsafe fn contentListWithViewController(view_controller: &NSViewController)
            -> Id<Self>;

        #[method(behavior)]
        pub unsafe fn behavior(&self) -> NSSplitViewItemBehavior;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other viewController)]
        pub unsafe fn viewController(&self) -> Id<NSViewController>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(setViewController:)]
        pub unsafe fn setViewController(&self, view_controller: &NSViewController);

        #[method(isCollapsed)]
        pub unsafe fn isCollapsed(&self) -> bool;

        #[method(setCollapsed:)]
        pub unsafe fn setCollapsed(&self, collapsed: bool);

        #[method(canCollapse)]
        pub unsafe fn canCollapse(&self) -> bool;

        #[method(setCanCollapse:)]
        pub unsafe fn setCanCollapse(&self, can_collapse: bool);

        #[method(collapseBehavior)]
        pub unsafe fn collapseBehavior(&self) -> NSSplitViewItemCollapseBehavior;

        #[method(setCollapseBehavior:)]
        pub unsafe fn setCollapseBehavior(
            &self,
            collapse_behavior: NSSplitViewItemCollapseBehavior,
        );

        #[method(minimumThickness)]
        pub unsafe fn minimumThickness(&self) -> CGFloat;

        #[method(setMinimumThickness:)]
        pub unsafe fn setMinimumThickness(&self, minimum_thickness: CGFloat);

        #[method(maximumThickness)]
        pub unsafe fn maximumThickness(&self) -> CGFloat;

        #[method(setMaximumThickness:)]
        pub unsafe fn setMaximumThickness(&self, maximum_thickness: CGFloat);

        #[method(preferredThicknessFraction)]
        pub unsafe fn preferredThicknessFraction(&self) -> CGFloat;

        #[method(setPreferredThicknessFraction:)]
        pub unsafe fn setPreferredThicknessFraction(&self, preferred_thickness_fraction: CGFloat);

        #[method(holdingPriority)]
        pub unsafe fn holdingPriority(&self) -> NSLayoutPriority;

        #[method(setHoldingPriority:)]
        pub unsafe fn setHoldingPriority(&self, holding_priority: NSLayoutPriority);

        #[method(automaticMaximumThickness)]
        pub unsafe fn automaticMaximumThickness(&self) -> CGFloat;

        #[method(setAutomaticMaximumThickness:)]
        pub unsafe fn setAutomaticMaximumThickness(&self, automatic_maximum_thickness: CGFloat);

        #[method(isSpringLoaded)]
        pub unsafe fn isSpringLoaded(&self) -> bool;

        #[method(setSpringLoaded:)]
        pub unsafe fn setSpringLoaded(&self, spring_loaded: bool);

        #[method(allowsFullHeightLayout)]
        pub unsafe fn allowsFullHeightLayout(&self) -> bool;

        #[method(setAllowsFullHeightLayout:)]
        pub unsafe fn setAllowsFullHeightLayout(&self, allows_full_height_layout: bool);

        #[method(titlebarSeparatorStyle)]
        pub unsafe fn titlebarSeparatorStyle(&self) -> NSTitlebarSeparatorStyle;

        #[method(setTitlebarSeparatorStyle:)]
        pub unsafe fn setTitlebarSeparatorStyle(
            &self,
            titlebar_separator_style: NSTitlebarSeparatorStyle,
        );
    }
);
