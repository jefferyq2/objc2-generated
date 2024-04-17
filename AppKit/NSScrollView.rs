//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSScrollElasticity(pub NSInteger);
impl NSScrollElasticity {
    #[doc(alias = "NSScrollElasticityAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "NSScrollElasticityNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "NSScrollElasticityAllowed")]
    pub const Allowed: Self = Self(2);
}

unsafe impl Encode for NSScrollElasticity {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSScrollElasticity {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    pub struct NSScrollView;

    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl ClassType for NSScrollView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSScrollView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSScrollView {}

#[cfg(all(feature = "NSAnimation", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAnimatablePropertyContainer for NSScrollView {}

#[cfg(all(feature = "NSAppearance", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSAppearanceCustomization for NSScrollView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSScrollView {}

#[cfg(all(feature = "NSDragging", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSDraggingDestination for NSScrollView {}

#[cfg(all(feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSScrollView {}

#[cfg(all(feature = "NSResponder", feature = "NSTextFinder", feature = "NSView"))]
unsafe impl NSTextFinderBarContainer for NSScrollView {}

#[cfg(all(
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSScrollView {}

extern_methods!(
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScrollView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(all(feature = "NSCell", feature = "NSScroller"))]
        #[method(frameSizeForContentSize:horizontalScrollerClass:verticalScrollerClass:borderType:controlSize:scrollerStyle:)]
        pub unsafe fn frameSizeForContentSize_horizontalScrollerClass_verticalScrollerClass_borderType_controlSize_scrollerStyle(
            c_size: NSSize,
            horizontal_scroller_class: Option<&AnyClass>,
            vertical_scroller_class: Option<&AnyClass>,
            r#type: NSBorderType,
            control_size: NSControlSize,
            scroller_style: NSScrollerStyle,
            mtm: MainThreadMarker,
        ) -> NSSize;

        #[cfg(all(feature = "NSCell", feature = "NSScroller"))]
        #[method(contentSizeForFrameSize:horizontalScrollerClass:verticalScrollerClass:borderType:controlSize:scrollerStyle:)]
        pub unsafe fn contentSizeForFrameSize_horizontalScrollerClass_verticalScrollerClass_borderType_controlSize_scrollerStyle(
            f_size: NSSize,
            horizontal_scroller_class: Option<&AnyClass>,
            vertical_scroller_class: Option<&AnyClass>,
            r#type: NSBorderType,
            control_size: NSControlSize,
            scroller_style: NSScrollerStyle,
            mtm: MainThreadMarker,
        ) -> NSSize;

        #[deprecated = "Use +frameSizeForContentSize:horizontalScrollerClass:verticalScrollerClass:borderType:controlSize:scrollerStyle: instead"]
        #[method(frameSizeForContentSize:hasHorizontalScroller:hasVerticalScroller:borderType:)]
        pub unsafe fn frameSizeForContentSize_hasHorizontalScroller_hasVerticalScroller_borderType(
            c_size: NSSize,
            h_flag: bool,
            v_flag: bool,
            r#type: NSBorderType,
            mtm: MainThreadMarker,
        ) -> NSSize;

        #[deprecated = "+contentSizeForFrameSize:horizontalScrollerClass:verticalScrollerClass:borderType:controlSize:scrollerStyle: instead"]
        #[method(contentSizeForFrameSize:hasHorizontalScroller:hasVerticalScroller:borderType:)]
        pub unsafe fn contentSizeForFrameSize_hasHorizontalScroller_hasVerticalScroller_borderType(
            f_size: NSSize,
            h_flag: bool,
            v_flag: bool,
            r#type: NSBorderType,
            mtm: MainThreadMarker,
        ) -> NSSize;

        #[method(documentVisibleRect)]
        pub unsafe fn documentVisibleRect(&self) -> NSRect;

        #[method(contentSize)]
        pub unsafe fn contentSize(&self) -> NSSize;

        #[method_id(@__retain_semantics Other documentView)]
        pub unsafe fn documentView(&self) -> Option<Id<NSView>>;

        #[method(setDocumentView:)]
        pub unsafe fn setDocumentView(&self, document_view: Option<&NSView>);

        #[cfg(feature = "NSClipView")]
        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Id<NSClipView>;

        #[cfg(feature = "NSClipView")]
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, content_view: &NSClipView);

        #[cfg(feature = "NSCursor")]
        #[method_id(@__retain_semantics Other documentCursor)]
        pub unsafe fn documentCursor(&self) -> Option<Id<NSCursor>>;

        #[cfg(feature = "NSCursor")]
        #[method(setDocumentCursor:)]
        pub unsafe fn setDocumentCursor(&self, document_cursor: Option<&NSCursor>);

        #[method(borderType)]
        pub unsafe fn borderType(&self) -> NSBorderType;

        #[method(setBorderType:)]
        pub unsafe fn setBorderType(&self, border_type: NSBorderType);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

        #[cfg(feature = "NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[method(hasVerticalScroller)]
        pub unsafe fn hasVerticalScroller(&self) -> bool;

        #[method(setHasVerticalScroller:)]
        pub unsafe fn setHasVerticalScroller(&self, has_vertical_scroller: bool);

        #[method(hasHorizontalScroller)]
        pub unsafe fn hasHorizontalScroller(&self) -> bool;

        #[method(setHasHorizontalScroller:)]
        pub unsafe fn setHasHorizontalScroller(&self, has_horizontal_scroller: bool);

        #[cfg(all(feature = "NSControl", feature = "NSScroller"))]
        #[method_id(@__retain_semantics Other verticalScroller)]
        pub unsafe fn verticalScroller(&self) -> Option<Id<NSScroller>>;

        #[cfg(all(feature = "NSControl", feature = "NSScroller"))]
        #[method(setVerticalScroller:)]
        pub unsafe fn setVerticalScroller(&self, vertical_scroller: Option<&NSScroller>);

        #[cfg(all(feature = "NSControl", feature = "NSScroller"))]
        #[method_id(@__retain_semantics Other horizontalScroller)]
        pub unsafe fn horizontalScroller(&self) -> Option<Id<NSScroller>>;

        #[cfg(all(feature = "NSControl", feature = "NSScroller"))]
        #[method(setHorizontalScroller:)]
        pub unsafe fn setHorizontalScroller(&self, horizontal_scroller: Option<&NSScroller>);

        #[method(autohidesScrollers)]
        pub unsafe fn autohidesScrollers(&self) -> bool;

        #[method(setAutohidesScrollers:)]
        pub unsafe fn setAutohidesScrollers(&self, autohides_scrollers: bool);

        #[method(horizontalLineScroll)]
        pub unsafe fn horizontalLineScroll(&self) -> CGFloat;

        #[method(setHorizontalLineScroll:)]
        pub unsafe fn setHorizontalLineScroll(&self, horizontal_line_scroll: CGFloat);

        #[method(verticalLineScroll)]
        pub unsafe fn verticalLineScroll(&self) -> CGFloat;

        #[method(setVerticalLineScroll:)]
        pub unsafe fn setVerticalLineScroll(&self, vertical_line_scroll: CGFloat);

        #[method(lineScroll)]
        pub unsafe fn lineScroll(&self) -> CGFloat;

        #[method(setLineScroll:)]
        pub unsafe fn setLineScroll(&self, line_scroll: CGFloat);

        #[method(horizontalPageScroll)]
        pub unsafe fn horizontalPageScroll(&self) -> CGFloat;

        #[method(setHorizontalPageScroll:)]
        pub unsafe fn setHorizontalPageScroll(&self, horizontal_page_scroll: CGFloat);

        #[method(verticalPageScroll)]
        pub unsafe fn verticalPageScroll(&self) -> CGFloat;

        #[method(setVerticalPageScroll:)]
        pub unsafe fn setVerticalPageScroll(&self, vertical_page_scroll: CGFloat);

        #[method(pageScroll)]
        pub unsafe fn pageScroll(&self) -> CGFloat;

        #[method(setPageScroll:)]
        pub unsafe fn setPageScroll(&self, page_scroll: CGFloat);

        #[method(scrollsDynamically)]
        pub unsafe fn scrollsDynamically(&self) -> bool;

        #[method(setScrollsDynamically:)]
        pub unsafe fn setScrollsDynamically(&self, scrolls_dynamically: bool);

        #[method(tile)]
        pub unsafe fn tile(&self);

        #[cfg(feature = "NSClipView")]
        #[method(reflectScrolledClipView:)]
        pub unsafe fn reflectScrolledClipView(&self, c_view: &NSClipView);

        #[cfg(feature = "NSEvent")]
        #[method(scrollWheel:)]
        pub unsafe fn scrollWheel(&self, event: &NSEvent);

        #[cfg(feature = "NSScroller")]
        #[method(scrollerStyle)]
        pub unsafe fn scrollerStyle(&self) -> NSScrollerStyle;

        #[cfg(feature = "NSScroller")]
        #[method(setScrollerStyle:)]
        pub unsafe fn setScrollerStyle(&self, scroller_style: NSScrollerStyle);

        #[cfg(feature = "NSScroller")]
        #[method(scrollerKnobStyle)]
        pub unsafe fn scrollerKnobStyle(&self) -> NSScrollerKnobStyle;

        #[cfg(feature = "NSScroller")]
        #[method(setScrollerKnobStyle:)]
        pub unsafe fn setScrollerKnobStyle(&self, scroller_knob_style: NSScrollerKnobStyle);

        #[method(flashScrollers)]
        pub unsafe fn flashScrollers(&self);

        #[method(horizontalScrollElasticity)]
        pub unsafe fn horizontalScrollElasticity(&self) -> NSScrollElasticity;

        #[method(setHorizontalScrollElasticity:)]
        pub unsafe fn setHorizontalScrollElasticity(
            &self,
            horizontal_scroll_elasticity: NSScrollElasticity,
        );

        #[method(verticalScrollElasticity)]
        pub unsafe fn verticalScrollElasticity(&self) -> NSScrollElasticity;

        #[method(setVerticalScrollElasticity:)]
        pub unsafe fn setVerticalScrollElasticity(
            &self,
            vertical_scroll_elasticity: NSScrollElasticity,
        );

        #[method(usesPredominantAxisScrolling)]
        pub unsafe fn usesPredominantAxisScrolling(&self) -> bool;

        #[method(setUsesPredominantAxisScrolling:)]
        pub unsafe fn setUsesPredominantAxisScrolling(&self, uses_predominant_axis_scrolling: bool);

        #[method(allowsMagnification)]
        pub unsafe fn allowsMagnification(&self) -> bool;

        #[method(setAllowsMagnification:)]
        pub unsafe fn setAllowsMagnification(&self, allows_magnification: bool);

        #[method(magnification)]
        pub unsafe fn magnification(&self) -> CGFloat;

        #[method(setMagnification:)]
        pub unsafe fn setMagnification(&self, magnification: CGFloat);

        #[method(maxMagnification)]
        pub unsafe fn maxMagnification(&self) -> CGFloat;

        #[method(setMaxMagnification:)]
        pub unsafe fn setMaxMagnification(&self, max_magnification: CGFloat);

        #[method(minMagnification)]
        pub unsafe fn minMagnification(&self) -> CGFloat;

        #[method(setMinMagnification:)]
        pub unsafe fn setMinMagnification(&self, min_magnification: CGFloat);

        #[method(magnifyToFitRect:)]
        pub unsafe fn magnifyToFitRect(&self, rect: NSRect);

        #[method(setMagnification:centeredAtPoint:)]
        pub unsafe fn setMagnification_centeredAtPoint(
            &self,
            magnification: CGFloat,
            point: NSPoint,
        );

        #[cfg(feature = "NSEvent")]
        #[method(addFloatingSubview:forAxis:)]
        pub unsafe fn addFloatingSubview_forAxis(&self, view: &NSView, axis: NSEventGestureAxis);

        #[method(automaticallyAdjustsContentInsets)]
        pub unsafe fn automaticallyAdjustsContentInsets(&self) -> bool;

        #[method(setAutomaticallyAdjustsContentInsets:)]
        pub unsafe fn setAutomaticallyAdjustsContentInsets(
            &self,
            automatically_adjusts_content_insets: bool,
        );

        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> NSEdgeInsets;

        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, content_insets: NSEdgeInsets);

        #[method(scrollerInsets)]
        pub unsafe fn scrollerInsets(&self) -> NSEdgeInsets;

        #[method(setScrollerInsets:)]
        pub unsafe fn setScrollerInsets(&self, scroller_insets: NSEdgeInsets);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScrollView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScrollView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern "C" {
    pub static NSScrollViewWillStartLiveMagnifyNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSScrollViewDidEndLiveMagnifyNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSScrollViewWillStartLiveScrollNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSScrollViewDidLiveScrollNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSScrollViewDidEndLiveScrollNotification: &'static NSNotificationName;
}

extern_methods!(
    /// NSRulerSupport
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScrollView {
        #[method(rulerViewClass)]
        pub unsafe fn rulerViewClass(mtm: MainThreadMarker) -> Option<&'static AnyClass>;

        #[method(setRulerViewClass:)]
        pub unsafe fn setRulerViewClass(ruler_view_class: Option<&AnyClass>, mtm: MainThreadMarker);

        #[method(rulersVisible)]
        pub unsafe fn rulersVisible(&self) -> bool;

        #[method(setRulersVisible:)]
        pub unsafe fn setRulersVisible(&self, rulers_visible: bool);

        #[method(hasHorizontalRuler)]
        pub unsafe fn hasHorizontalRuler(&self) -> bool;

        #[method(setHasHorizontalRuler:)]
        pub unsafe fn setHasHorizontalRuler(&self, has_horizontal_ruler: bool);

        #[method(hasVerticalRuler)]
        pub unsafe fn hasVerticalRuler(&self) -> bool;

        #[method(setHasVerticalRuler:)]
        pub unsafe fn setHasVerticalRuler(&self, has_vertical_ruler: bool);

        #[cfg(feature = "NSRulerView")]
        #[method_id(@__retain_semantics Other horizontalRulerView)]
        pub unsafe fn horizontalRulerView(&self) -> Option<Id<NSRulerView>>;

        #[cfg(feature = "NSRulerView")]
        #[method(setHorizontalRulerView:)]
        pub unsafe fn setHorizontalRulerView(&self, horizontal_ruler_view: Option<&NSRulerView>);

        #[cfg(feature = "NSRulerView")]
        #[method_id(@__retain_semantics Other verticalRulerView)]
        pub unsafe fn verticalRulerView(&self) -> Option<Id<NSRulerView>>;

        #[cfg(feature = "NSRulerView")]
        #[method(setVerticalRulerView:)]
        pub unsafe fn setVerticalRulerView(&self, vertical_ruler_view: Option<&NSRulerView>);
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSScrollViewFindBarPosition(pub NSInteger);
impl NSScrollViewFindBarPosition {
    #[doc(alias = "NSScrollViewFindBarPositionAboveHorizontalRuler")]
    pub const AboveHorizontalRuler: Self = Self(0);
    #[doc(alias = "NSScrollViewFindBarPositionAboveContent")]
    pub const AboveContent: Self = Self(1);
    #[doc(alias = "NSScrollViewFindBarPositionBelowContent")]
    pub const BelowContent: Self = Self(2);
}

unsafe impl Encode for NSScrollViewFindBarPosition {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSScrollViewFindBarPosition {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSFindBarSupport
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSScrollView {
        #[method(findBarPosition)]
        pub unsafe fn findBarPosition(&self) -> NSScrollViewFindBarPosition;

        #[method(setFindBarPosition:)]
        pub unsafe fn setFindBarPosition(&self, find_bar_position: NSScrollViewFindBarPosition);
    }
);
