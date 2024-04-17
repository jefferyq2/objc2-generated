//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CALayer")]
    pub struct CATiledLayer;

    #[cfg(feature = "CALayer")]
    unsafe impl ClassType for CATiledLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "CALayer", feature = "CAMediaTiming"))]
unsafe impl CAMediaTiming for CATiledLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSCoding for CATiledLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSObjectProtocol for CATiledLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSSecureCoding for CATiledLayer {}

extern_methods!(
    #[cfg(feature = "CALayer")]
    unsafe impl CATiledLayer {
        #[method(fadeDuration)]
        pub unsafe fn fadeDuration() -> CFTimeInterval;

        #[method(levelsOfDetail)]
        pub unsafe fn levelsOfDetail(&self) -> usize;

        #[method(setLevelsOfDetail:)]
        pub unsafe fn setLevelsOfDetail(&self, levels_of_detail: usize);

        #[method(levelsOfDetailBias)]
        pub unsafe fn levelsOfDetailBias(&self) -> usize;

        #[method(setLevelsOfDetailBias:)]
        pub unsafe fn setLevelsOfDetailBias(&self, levels_of_detail_bias: usize);

        #[method(tileSize)]
        pub unsafe fn tileSize(&self) -> CGSize;

        #[method(setTileSize:)]
        pub unsafe fn setTileSize(&self, tile_size: CGSize);
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CALayer")]
    unsafe impl CATiledLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CALayer")]
    unsafe impl CATiledLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
