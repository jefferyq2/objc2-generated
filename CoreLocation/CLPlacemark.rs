//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLPlacemark")]
    pub struct CLPlacemark;

    #[cfg(feature = "CoreLocation_CLPlacemark")]
    unsafe impl ClassType for CLPlacemark {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreLocation_CLPlacemark")]
unsafe impl NSCoding for CLPlacemark {}

#[cfg(feature = "CoreLocation_CLPlacemark")]
unsafe impl NSObjectProtocol for CLPlacemark {}

#[cfg(feature = "CoreLocation_CLPlacemark")]
unsafe impl NSSecureCoding for CLPlacemark {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLPlacemark")]
    unsafe impl CLPlacemark {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithPlacemark:)]
        pub unsafe fn initWithPlacemark(
            this: Option<Allocated<Self>>,
            placemark: &CLPlacemark,
        ) -> Id<Self>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Id<CLLocation>>;

        #[cfg(feature = "CoreLocation_CLRegion")]
        #[method_id(@__retain_semantics Other region)]
        pub unsafe fn region(&self) -> Option<Id<CLRegion>>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Id<NSTimeZone>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[deprecated = "Use @properties"]
        #[method_id(@__retain_semantics Other addressDictionary)]
        pub unsafe fn addressDictionary(&self) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other thoroughfare)]
        pub unsafe fn thoroughfare(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subThoroughfare)]
        pub unsafe fn subThoroughfare(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other locality)]
        pub unsafe fn locality(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subLocality)]
        pub unsafe fn subLocality(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other administrativeArea)]
        pub unsafe fn administrativeArea(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subAdministrativeArea)]
        pub unsafe fn subAdministrativeArea(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other postalCode)]
        pub unsafe fn postalCode(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other ISOcountryCode)]
        pub unsafe fn ISOcountryCode(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other country)]
        pub unsafe fn country(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other inlandWater)]
        pub unsafe fn inlandWater(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other ocean)]
        pub unsafe fn ocean(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other areasOfInterest)]
        pub unsafe fn areasOfInterest(&self) -> Option<Id<NSArray<NSString>>>;
    }
);

extern_methods!(
    /// ContactsAdditions
    #[cfg(feature = "CoreLocation_CLPlacemark")]
    unsafe impl CLPlacemark {
        #[cfg(feature = "Contacts_CNPostalAddress")]
        #[method_id(@__retain_semantics Other postalAddress)]
        pub unsafe fn postalAddress(&self) -> Option<Id<CNPostalAddress>>;
    }
);
