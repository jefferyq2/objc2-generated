//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKUserLocationView")]
    pub struct MKUserLocationView;

    #[cfg(feature = "MapKit_MKUserLocationView")]
    unsafe impl ClassType for MKUserLocationView {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = MKAnnotationView;
    }
);

#[cfg(feature = "MapKit_MKUserLocationView")]
unsafe impl NSAccessibility for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
unsafe impl NSAccessibilityElementProtocol for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
unsafe impl NSAnimatablePropertyContainer for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
unsafe impl NSAppearanceCustomization for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
unsafe impl NSCoding for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
unsafe impl NSDraggingDestination for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
unsafe impl NSObjectProtocol for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
unsafe impl NSUserInterfaceItemIdentification for MKUserLocationView {}

extern_methods!(
    #[cfg(feature = "MapKit_MKUserLocationView")]
    unsafe impl MKUserLocationView {}
);

extern_methods!(
    /// Methods declared on superclass `MKAnnotationView`
    #[cfg(feature = "MapKit_MKUserLocationView")]
    unsafe impl MKUserLocationView {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithAnnotation:reuseIdentifier:)]
        pub unsafe fn initWithAnnotation_reuseIdentifier(
            this: Option<Allocated<Self>>,
            annotation: Option<&ProtocolObject<dyn MKAnnotation>>,
            reuse_identifier: Option<&NSString>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "MapKit_MKUserLocationView")]
    unsafe impl MKUserLocationView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);
