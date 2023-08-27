//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSCollectionElementCategory {
        NSCollectionElementCategoryItem = 0,
        NSCollectionElementCategorySupplementaryView = 1,
        NSCollectionElementCategoryDecorationView = 2,
        NSCollectionElementCategoryInterItemGap = 3,
    }
);

pub type NSCollectionViewDecorationElementKind = NSString;

extern_static!(NSCollectionElementKindInterItemGapIndicator: &'static NSCollectionViewSupplementaryElementKind);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSCollectionViewLayoutAttributes")]
    pub struct NSCollectionViewLayoutAttributes;

    #[cfg(feature = "AppKit_NSCollectionViewLayoutAttributes")]
    unsafe impl ClassType for NSCollectionViewLayoutAttributes {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSCollectionViewLayoutAttributes")]
unsafe impl NSCopying for NSCollectionViewLayoutAttributes {}

#[cfg(feature = "AppKit_NSCollectionViewLayoutAttributes")]
unsafe impl NSObjectProtocol for NSCollectionViewLayoutAttributes {}

extern_methods!(
    #[cfg(feature = "AppKit_NSCollectionViewLayoutAttributes")]
    unsafe impl NSCollectionViewLayoutAttributes {
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: NSRect);

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;

        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: CGFloat);

        #[method(zIndex)]
        pub unsafe fn zIndex(&self) -> NSInteger;

        #[method(setZIndex:)]
        pub unsafe fn setZIndex(&self, z_index: NSInteger);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other indexPath)]
        pub unsafe fn indexPath(&self) -> Option<Id<NSIndexPath>>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method(setIndexPath:)]
        pub unsafe fn setIndexPath(&self, index_path: Option<&NSIndexPath>);

        #[method(representedElementCategory)]
        pub unsafe fn representedElementCategory(&self) -> NSCollectionElementCategory;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other representedElementKind)]
        pub unsafe fn representedElementKind(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other layoutAttributesForItemWithIndexPath:)]
        pub unsafe fn layoutAttributesForItemWithIndexPath(index_path: &NSIndexPath) -> Id<Self>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other layoutAttributesForInterItemGapBeforeIndexPath:)]
        pub unsafe fn layoutAttributesForInterItemGapBeforeIndexPath(
            index_path: &NSIndexPath,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other layoutAttributesForSupplementaryViewOfKind:withIndexPath:)]
        pub unsafe fn layoutAttributesForSupplementaryViewOfKind_withIndexPath(
            element_kind: &NSCollectionViewSupplementaryElementKind,
            index_path: &NSIndexPath,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other layoutAttributesForDecorationViewOfKind:withIndexPath:)]
        pub unsafe fn layoutAttributesForDecorationViewOfKind_withIndexPath(
            decoration_view_kind: &NSCollectionViewDecorationElementKind,
            index_path: &NSIndexPath,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSCollectionViewLayoutAttributes")]
    unsafe impl NSCollectionViewLayoutAttributes {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSCollectionUpdateAction {
        NSCollectionUpdateActionInsert = 0,
        NSCollectionUpdateActionDelete = 1,
        NSCollectionUpdateActionReload = 2,
        NSCollectionUpdateActionMove = 3,
        NSCollectionUpdateActionNone = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSCollectionViewUpdateItem")]
    pub struct NSCollectionViewUpdateItem;

    #[cfg(feature = "AppKit_NSCollectionViewUpdateItem")]
    unsafe impl ClassType for NSCollectionViewUpdateItem {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSCollectionViewUpdateItem")]
unsafe impl NSObjectProtocol for NSCollectionViewUpdateItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSCollectionViewUpdateItem")]
    unsafe impl NSCollectionViewUpdateItem {
        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other indexPathBeforeUpdate)]
        pub unsafe fn indexPathBeforeUpdate(&self) -> Option<Id<NSIndexPath>>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other indexPathAfterUpdate)]
        pub unsafe fn indexPathAfterUpdate(&self) -> Option<Id<NSIndexPath>>;

        #[method(updateAction)]
        pub unsafe fn updateAction(&self) -> NSCollectionUpdateAction;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSCollectionViewUpdateItem")]
    unsafe impl NSCollectionViewUpdateItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSCollectionViewLayoutInvalidationContext")]
    pub struct NSCollectionViewLayoutInvalidationContext;

    #[cfg(feature = "AppKit_NSCollectionViewLayoutInvalidationContext")]
    unsafe impl ClassType for NSCollectionViewLayoutInvalidationContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSCollectionViewLayoutInvalidationContext")]
unsafe impl NSObjectProtocol for NSCollectionViewLayoutInvalidationContext {}

extern_methods!(
    #[cfg(feature = "AppKit_NSCollectionViewLayoutInvalidationContext")]
    unsafe impl NSCollectionViewLayoutInvalidationContext {
        #[method(invalidateEverything)]
        pub unsafe fn invalidateEverything(&self) -> bool;

        #[method(invalidateDataSourceCounts)]
        pub unsafe fn invalidateDataSourceCounts(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSIndexPath", feature = "Foundation_NSSet"))]
        #[method(invalidateItemsAtIndexPaths:)]
        pub unsafe fn invalidateItemsAtIndexPaths(&self, index_paths: &NSSet<NSIndexPath>);

        #[cfg(all(feature = "Foundation_NSIndexPath", feature = "Foundation_NSSet"))]
        #[method(invalidateSupplementaryElementsOfKind:atIndexPaths:)]
        pub unsafe fn invalidateSupplementaryElementsOfKind_atIndexPaths(
            &self,
            element_kind: &NSCollectionViewSupplementaryElementKind,
            index_paths: &NSSet<NSIndexPath>,
        );

        #[cfg(all(feature = "Foundation_NSIndexPath", feature = "Foundation_NSSet"))]
        #[method(invalidateDecorationElementsOfKind:atIndexPaths:)]
        pub unsafe fn invalidateDecorationElementsOfKind_atIndexPaths(
            &self,
            element_kind: &NSCollectionViewDecorationElementKind,
            index_paths: &NSSet<NSIndexPath>,
        );

        #[cfg(all(feature = "Foundation_NSIndexPath", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other invalidatedItemIndexPaths)]
        pub unsafe fn invalidatedItemIndexPaths(&self) -> Option<Id<NSSet<NSIndexPath>>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSIndexPath",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other invalidatedSupplementaryIndexPaths)]
        pub unsafe fn invalidatedSupplementaryIndexPaths(
            &self,
        ) -> Option<Id<NSDictionary<NSCollectionViewSupplementaryElementKind, NSSet<NSIndexPath>>>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSIndexPath",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other invalidatedDecorationIndexPaths)]
        pub unsafe fn invalidatedDecorationIndexPaths(
            &self,
        ) -> Option<Id<NSDictionary<NSCollectionViewDecorationElementKind, NSSet<NSIndexPath>>>>;

        #[method(contentOffsetAdjustment)]
        pub unsafe fn contentOffsetAdjustment(&self) -> NSPoint;

        #[method(setContentOffsetAdjustment:)]
        pub unsafe fn setContentOffsetAdjustment(&self, content_offset_adjustment: NSPoint);

        #[method(contentSizeAdjustment)]
        pub unsafe fn contentSizeAdjustment(&self) -> NSSize;

        #[method(setContentSizeAdjustment:)]
        pub unsafe fn setContentSizeAdjustment(&self, content_size_adjustment: NSSize);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSCollectionViewLayoutInvalidationContext")]
    unsafe impl NSCollectionViewLayoutInvalidationContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    pub struct NSCollectionViewLayout;

    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    unsafe impl ClassType for NSCollectionViewLayout {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSCollectionViewLayout")]
unsafe impl NSCoding for NSCollectionViewLayout {}

#[cfg(feature = "AppKit_NSCollectionViewLayout")]
unsafe impl NSObjectProtocol for NSCollectionViewLayout {}

extern_methods!(
    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    unsafe impl NSCollectionViewLayout {
        #[cfg(feature = "AppKit_NSCollectionView")]
        #[method_id(@__retain_semantics Other collectionView)]
        pub unsafe fn collectionView(&self) -> Option<Id<NSCollectionView>>;

        #[method(invalidateLayout)]
        pub unsafe fn invalidateLayout(&self);

        #[cfg(feature = "AppKit_NSCollectionViewLayoutInvalidationContext")]
        #[method(invalidateLayoutWithContext:)]
        pub unsafe fn invalidateLayoutWithContext(
            &self,
            context: &NSCollectionViewLayoutInvalidationContext,
        );

        #[method(registerClass:forDecorationViewOfKind:)]
        pub unsafe fn registerClass_forDecorationViewOfKind(
            &self,
            view_class: Option<&AnyClass>,
            element_kind: &NSCollectionViewDecorationElementKind,
        );

        #[cfg(feature = "AppKit_NSNib")]
        #[method(registerNib:forDecorationViewOfKind:)]
        pub unsafe fn registerNib_forDecorationViewOfKind(
            &self,
            nib: Option<&NSNib>,
            element_kind: &NSCollectionViewDecorationElementKind,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    unsafe impl NSCollectionViewLayout {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSSubclassingHooks
    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    unsafe impl NSCollectionViewLayout {
        #[method(layoutAttributesClass)]
        pub unsafe fn layoutAttributesClass() -> &'static AnyClass;

        #[method(invalidationContextClass)]
        pub unsafe fn invalidationContextClass() -> &'static AnyClass;

        #[method(prepareLayout)]
        pub unsafe fn prepareLayout(&self);

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayoutAttributes",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other layoutAttributesForElementsInRect:)]
        pub unsafe fn layoutAttributesForElementsInRect(
            &self,
            rect: NSRect,
        ) -> Id<NSArray<NSCollectionViewLayoutAttributes>>;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayoutAttributes",
            feature = "Foundation_NSIndexPath"
        ))]
        #[method_id(@__retain_semantics Other layoutAttributesForItemAtIndexPath:)]
        pub unsafe fn layoutAttributesForItemAtIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes>>;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayoutAttributes",
            feature = "Foundation_NSIndexPath"
        ))]
        #[method_id(@__retain_semantics Other layoutAttributesForSupplementaryViewOfKind:atIndexPath:)]
        pub unsafe fn layoutAttributesForSupplementaryViewOfKind_atIndexPath(
            &self,
            element_kind: &NSCollectionViewSupplementaryElementKind,
            index_path: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes>>;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayoutAttributes",
            feature = "Foundation_NSIndexPath"
        ))]
        #[method_id(@__retain_semantics Other layoutAttributesForDecorationViewOfKind:atIndexPath:)]
        pub unsafe fn layoutAttributesForDecorationViewOfKind_atIndexPath(
            &self,
            element_kind: &NSCollectionViewDecorationElementKind,
            index_path: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes>>;

        #[cfg(feature = "AppKit_NSCollectionViewLayoutAttributes")]
        #[method_id(@__retain_semantics Other layoutAttributesForDropTargetAtPoint:)]
        pub unsafe fn layoutAttributesForDropTargetAtPoint(
            &self,
            point_in_collection_view: NSPoint,
        ) -> Option<Id<NSCollectionViewLayoutAttributes>>;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayoutAttributes",
            feature = "Foundation_NSIndexPath"
        ))]
        #[method_id(@__retain_semantics Other layoutAttributesForInterItemGapBeforeIndexPath:)]
        pub unsafe fn layoutAttributesForInterItemGapBeforeIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes>>;

        #[method(shouldInvalidateLayoutForBoundsChange:)]
        pub unsafe fn shouldInvalidateLayoutForBoundsChange(&self, new_bounds: NSRect) -> bool;

        #[cfg(feature = "AppKit_NSCollectionViewLayoutInvalidationContext")]
        #[method_id(@__retain_semantics Other invalidationContextForBoundsChange:)]
        pub unsafe fn invalidationContextForBoundsChange(
            &self,
            new_bounds: NSRect,
        ) -> Id<NSCollectionViewLayoutInvalidationContext>;

        #[cfg(feature = "AppKit_NSCollectionViewLayoutAttributes")]
        #[method(shouldInvalidateLayoutForPreferredLayoutAttributes:withOriginalAttributes:)]
        pub unsafe fn shouldInvalidateLayoutForPreferredLayoutAttributes_withOriginalAttributes(
            &self,
            preferred_attributes: &NSCollectionViewLayoutAttributes,
            original_attributes: &NSCollectionViewLayoutAttributes,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayoutAttributes",
            feature = "AppKit_NSCollectionViewLayoutInvalidationContext"
        ))]
        #[method_id(@__retain_semantics Other invalidationContextForPreferredLayoutAttributes:withOriginalAttributes:)]
        pub unsafe fn invalidationContextForPreferredLayoutAttributes_withOriginalAttributes(
            &self,
            preferred_attributes: &NSCollectionViewLayoutAttributes,
            original_attributes: &NSCollectionViewLayoutAttributes,
        ) -> Id<NSCollectionViewLayoutInvalidationContext>;

        #[method(targetContentOffsetForProposedContentOffset:withScrollingVelocity:)]
        pub unsafe fn targetContentOffsetForProposedContentOffset_withScrollingVelocity(
            &self,
            proposed_content_offset: NSPoint,
            velocity: NSPoint,
        ) -> NSPoint;

        #[method(targetContentOffsetForProposedContentOffset:)]
        pub unsafe fn targetContentOffsetForProposedContentOffset(
            &self,
            proposed_content_offset: NSPoint,
        ) -> NSPoint;

        #[method(collectionViewContentSize)]
        pub unsafe fn collectionViewContentSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// NSUpdateSupportHooks
    #[cfg(feature = "AppKit_NSCollectionViewLayout")]
    unsafe impl NSCollectionViewLayout {
        #[cfg(all(
            feature = "AppKit_NSCollectionViewUpdateItem",
            feature = "Foundation_NSArray"
        ))]
        #[method(prepareForCollectionViewUpdates:)]
        pub unsafe fn prepareForCollectionViewUpdates(
            &self,
            update_items: &NSArray<NSCollectionViewUpdateItem>,
        );

        #[method(finalizeCollectionViewUpdates)]
        pub unsafe fn finalizeCollectionViewUpdates(&self);

        #[method(prepareForAnimatedBoundsChange:)]
        pub unsafe fn prepareForAnimatedBoundsChange(&self, old_bounds: NSRect);

        #[method(finalizeAnimatedBoundsChange)]
        pub unsafe fn finalizeAnimatedBoundsChange(&self);

        #[method(prepareForTransitionToLayout:)]
        pub unsafe fn prepareForTransitionToLayout(&self, new_layout: &NSCollectionViewLayout);

        #[method(prepareForTransitionFromLayout:)]
        pub unsafe fn prepareForTransitionFromLayout(&self, old_layout: &NSCollectionViewLayout);

        #[method(finalizeLayoutTransition)]
        pub unsafe fn finalizeLayoutTransition(&self);

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayoutAttributes",
            feature = "Foundation_NSIndexPath"
        ))]
        #[method_id(@__retain_semantics Other initialLayoutAttributesForAppearingItemAtIndexPath:)]
        pub unsafe fn initialLayoutAttributesForAppearingItemAtIndexPath(
            &self,
            item_index_path: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes>>;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayoutAttributes",
            feature = "Foundation_NSIndexPath"
        ))]
        #[method_id(@__retain_semantics Other finalLayoutAttributesForDisappearingItemAtIndexPath:)]
        pub unsafe fn finalLayoutAttributesForDisappearingItemAtIndexPath(
            &self,
            item_index_path: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes>>;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayoutAttributes",
            feature = "Foundation_NSIndexPath"
        ))]
        #[method_id(@__retain_semantics Other initialLayoutAttributesForAppearingSupplementaryElementOfKind:atIndexPath:)]
        pub unsafe fn initialLayoutAttributesForAppearingSupplementaryElementOfKind_atIndexPath(
            &self,
            element_kind: &NSCollectionViewSupplementaryElementKind,
            element_index_path: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes>>;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayoutAttributes",
            feature = "Foundation_NSIndexPath"
        ))]
        #[method_id(@__retain_semantics Other finalLayoutAttributesForDisappearingSupplementaryElementOfKind:atIndexPath:)]
        pub unsafe fn finalLayoutAttributesForDisappearingSupplementaryElementOfKind_atIndexPath(
            &self,
            element_kind: &NSCollectionViewSupplementaryElementKind,
            element_index_path: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes>>;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayoutAttributes",
            feature = "Foundation_NSIndexPath"
        ))]
        #[method_id(@__retain_semantics Other initialLayoutAttributesForAppearingDecorationElementOfKind:atIndexPath:)]
        pub unsafe fn initialLayoutAttributesForAppearingDecorationElementOfKind_atIndexPath(
            &self,
            element_kind: &NSCollectionViewDecorationElementKind,
            decoration_index_path: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes>>;

        #[cfg(all(
            feature = "AppKit_NSCollectionViewLayoutAttributes",
            feature = "Foundation_NSIndexPath"
        ))]
        #[method_id(@__retain_semantics Other finalLayoutAttributesForDisappearingDecorationElementOfKind:atIndexPath:)]
        pub unsafe fn finalLayoutAttributesForDisappearingDecorationElementOfKind_atIndexPath(
            &self,
            element_kind: &NSCollectionViewDecorationElementKind,
            decoration_index_path: &NSIndexPath,
        ) -> Option<Id<NSCollectionViewLayoutAttributes>>;

        #[cfg(all(feature = "Foundation_NSIndexPath", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other indexPathsToDeleteForSupplementaryViewOfKind:)]
        pub unsafe fn indexPathsToDeleteForSupplementaryViewOfKind(
            &self,
            element_kind: &NSCollectionViewSupplementaryElementKind,
        ) -> Id<NSSet<NSIndexPath>>;

        #[cfg(all(feature = "Foundation_NSIndexPath", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other indexPathsToDeleteForDecorationViewOfKind:)]
        pub unsafe fn indexPathsToDeleteForDecorationViewOfKind(
            &self,
            element_kind: &NSCollectionViewDecorationElementKind,
        ) -> Id<NSSet<NSIndexPath>>;

        #[cfg(all(feature = "Foundation_NSIndexPath", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other indexPathsToInsertForSupplementaryViewOfKind:)]
        pub unsafe fn indexPathsToInsertForSupplementaryViewOfKind(
            &self,
            element_kind: &NSCollectionViewSupplementaryElementKind,
        ) -> Id<NSSet<NSIndexPath>>;

        #[cfg(all(feature = "Foundation_NSIndexPath", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other indexPathsToInsertForDecorationViewOfKind:)]
        pub unsafe fn indexPathsToInsertForDecorationViewOfKind(
            &self,
            element_kind: &NSCollectionViewDecorationElementKind,
        ) -> Id<NSSet<NSIndexPath>>;
    }
);
