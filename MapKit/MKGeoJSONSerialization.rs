//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait MKGeoJSONObject: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn MKGeoJSONObject {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKGeoJSONDecoder;

    unsafe impl ClassType for MKGeoJSONDecoder {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MKGeoJSONDecoder {}

extern_methods!(
    unsafe impl MKGeoJSONDecoder {
        #[method_id(@__retain_semantics Other geoJSONObjectsWithData:error:_)]
        pub unsafe fn geoJSONObjectsWithData_error(
            &self,
            data: &NSData,
        ) -> Result<Id<NSArray<ProtocolObject<dyn MKGeoJSONObject>>>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKGeoJSONDecoder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKGeoJSONFeature;

    unsafe impl ClassType for MKGeoJSONFeature {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl MKGeoJSONObject for MKGeoJSONFeature {}

unsafe impl NSObjectProtocol for MKGeoJSONFeature {}

extern_methods!(
    unsafe impl MKGeoJSONFeature {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "MKShape")]
        #[method_id(@__retain_semantics Other geometry)]
        pub unsafe fn geometry(&self) -> Id<NSArray<MKShape>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKGeoJSONFeature {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// MKGeoJSONSerialization
    #[cfg(all(feature = "MKPointAnnotation", feature = "MKShape"))]
    unsafe impl MKPointAnnotation {}
);

#[cfg(all(feature = "MKPointAnnotation", feature = "MKShape"))]
unsafe impl MKGeoJSONObject for MKPointAnnotation {}

extern_methods!(
    /// MKGeoJSONSerialization
    #[cfg(all(feature = "MKMultiPoint", feature = "MKShape"))]
    unsafe impl MKMultiPoint {}
);

#[cfg(all(feature = "MKMultiPoint", feature = "MKShape"))]
unsafe impl MKGeoJSONObject for MKMultiPoint {}

extern_methods!(
    /// MKGeoJSONSerialization
    #[cfg(all(feature = "MKMultiPolyline", feature = "MKShape"))]
    unsafe impl MKMultiPolyline {}
);

#[cfg(all(feature = "MKMultiPolyline", feature = "MKShape"))]
unsafe impl MKGeoJSONObject for MKMultiPolyline {}

extern_methods!(
    /// MKGeoJSONSerialization
    #[cfg(all(feature = "MKMultiPolygon", feature = "MKShape"))]
    unsafe impl MKMultiPolygon {}
);

#[cfg(all(feature = "MKMultiPolygon", feature = "MKShape"))]
unsafe impl MKGeoJSONObject for MKMultiPolygon {}

extern_methods!(
    /// MKGeoJSONSerialization
    #[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline", feature = "MKShape"))]
    unsafe impl MKPolyline {}
);

#[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline", feature = "MKShape"))]
unsafe impl MKGeoJSONObject for MKPolyline {}

extern_methods!(
    /// MKGeoJSONSerialization
    #[cfg(all(feature = "MKMultiPoint", feature = "MKPolygon", feature = "MKShape"))]
    unsafe impl MKPolygon {}
);

#[cfg(all(feature = "MKMultiPoint", feature = "MKPolygon", feature = "MKShape"))]
unsafe impl MKGeoJSONObject for MKPolygon {}
