// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `MetalFX` framework

#[link(name = "MetalFX", kind = "framework")]
extern "C" {}

#[path = "MTLFXSpatialScaler.rs"]
mod __MTLFXSpatialScaler;
#[path = "MTLFXTemporalScaler.rs"]
mod __MTLFXTemporalScaler;

pub use self::__MTLFXSpatialScaler::MTLFXSpatialScaler;
pub use self::__MTLFXSpatialScaler::MTLFXSpatialScalerColorProcessingMode;
#[cfg(feature = "MetalFX_MTLFXSpatialScalerDescriptor")]
pub use self::__MTLFXSpatialScaler::MTLFXSpatialScalerDescriptor;
pub use self::__MTLFXSpatialScaler::{
    MTLFXSpatialScalerColorProcessingModeHDR, MTLFXSpatialScalerColorProcessingModeLinear,
    MTLFXSpatialScalerColorProcessingModePerceptual,
};
pub use self::__MTLFXTemporalScaler::MTLFXTemporalScaler;
#[cfg(feature = "MetalFX_MTLFXTemporalScalerDescriptor")]
pub use self::__MTLFXTemporalScaler::MTLFXTemporalScalerDescriptor;
