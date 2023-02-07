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
    #[cfg(feature = "MapKit_MKPlacemark")]
    pub struct MKPlacemark;

    #[cfg(feature = "MapKit_MKPlacemark")]
    unsafe impl ClassType for MKPlacemark {
        #[inherits(NSObject)]
        type Super = CLPlacemark;
    }
);

#[cfg(feature = "MapKit_MKPlacemark")]
unsafe impl MKAnnotation for MKPlacemark {}

#[cfg(feature = "MapKit_MKPlacemark")]
unsafe impl NSCoding for MKPlacemark {}

#[cfg(feature = "MapKit_MKPlacemark")]
unsafe impl NSObjectProtocol for MKPlacemark {}

#[cfg(feature = "MapKit_MKPlacemark")]
unsafe impl NSSecureCoding for MKPlacemark {}

extern_methods!(
    #[cfg(feature = "MapKit_MKPlacemark")]
    unsafe impl MKPlacemark {
        #[method_id(@__retain_semantics Init initWithCoordinate:)]
        pub unsafe fn initWithCoordinate(
            this: Option<Allocated<Self>>,
            coordinate: CLLocationCoordinate2D,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithCoordinate:addressDictionary:)]
        pub unsafe fn initWithCoordinate_addressDictionary(
            this: Option<Allocated<Self>>,
            coordinate: CLLocationCoordinate2D,
            address_dictionary: Option<&NSDictionary<NSString, Object>>,
        ) -> Id<Self>;

        #[cfg(feature = "Contacts_CNPostalAddress")]
        #[method_id(@__retain_semantics Init initWithCoordinate:postalAddress:)]
        pub unsafe fn initWithCoordinate_postalAddress(
            this: Option<Allocated<Self>>,
            coordinate: CLLocationCoordinate2D,
            postal_address: &CNPostalAddress,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CLPlacemark`
    #[cfg(feature = "MapKit_MKPlacemark")]
    unsafe impl MKPlacemark {
        #[method_id(@__retain_semantics Init initWithPlacemark:)]
        pub unsafe fn initWithPlacemark(
            this: Option<Allocated<Self>>,
            placemark: &CLPlacemark,
        ) -> Id<Self>;
    }
);
