//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

pub type MKDirectionsHandler = *mut Block<dyn Fn(*mut MKDirectionsResponse, *mut NSError)>;

pub type MKETAHandler = *mut Block<dyn Fn(*mut MKETAResponse, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKDirections")]
    pub struct MKDirections;

    #[cfg(feature = "MapKit_MKDirections")]
    unsafe impl ClassType for MKDirections {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKDirections")]
unsafe impl NSObjectProtocol for MKDirections {}

extern_methods!(
    #[cfg(feature = "MapKit_MKDirections")]
    unsafe impl MKDirections {
        #[cfg(feature = "MapKit_MKDirectionsRequest")]
        #[method_id(@__retain_semantics Init initWithRequest:)]
        pub unsafe fn initWithRequest(
            this: Allocated<Self>,
            request: &MKDirectionsRequest,
        ) -> Id<Self>;

        #[method(calculateDirectionsWithCompletionHandler:)]
        pub unsafe fn calculateDirectionsWithCompletionHandler(
            &self,
            completion_handler: MKDirectionsHandler,
        );

        #[method(calculateETAWithCompletionHandler:)]
        pub unsafe fn calculateETAWithCompletionHandler(&self, completion_handler: MKETAHandler);

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(isCalculating)]
        pub unsafe fn isCalculating(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKDirections")]
    unsafe impl MKDirections {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
