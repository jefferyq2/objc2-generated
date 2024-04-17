//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub struct GCExtendedGamepadSnapshot;

    #[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl ClassType for GCExtendedGamepadSnapshot {
        #[inherits(GCPhysicalInputProfile, NSObject)]
        type Super = GCExtendedGamepad;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
unsafe impl NSObjectProtocol for GCExtendedGamepadSnapshot {}

extern_methods!(
    #[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl GCExtendedGamepadSnapshot {
        #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
        #[method_id(@__retain_semantics Other snapshotData)]
        pub unsafe fn snapshotData(&self) -> Id<NSData>;

        #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
        #[method(setSnapshotData:)]
        pub unsafe fn setSnapshotData(&self, snapshot_data: &NSData);

        #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
        #[method_id(@__retain_semantics Init initWithSnapshotData:)]
        pub unsafe fn initWithSnapshotData(this: Allocated<Self>, data: &NSData) -> Id<Self>;

        #[cfg(feature = "GCController")]
        #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
        #[method_id(@__retain_semantics Init initWithController:snapshotData:)]
        pub unsafe fn initWithController_snapshotData(
            this: Allocated<Self>,
            controller: &GCController,
            data: &NSData,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl GCExtendedGamepadSnapshot {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

// NS_ENUM
#[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GCExtendedGamepadSnapshotDataVersion(pub NSInteger);
impl GCExtendedGamepadSnapshotDataVersion {
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub const GCExtendedGamepadSnapshotDataVersion1: Self = Self(0x0100);
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub const GCExtendedGamepadSnapshotDataVersion2: Self = Self(0x0101);
}

unsafe impl Encode for GCExtendedGamepadSnapshotDataVersion {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GCExtendedGamepadSnapshotDataVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static GCCurrentExtendedGamepadSnapshotDataVersion: GCExtendedGamepadSnapshotDataVersion;
}

extern "C" {
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub fn GCExtendedGamepadSnapshotDataFromNSData(
        snapshot_data: *mut GCExtendedGamepadSnapshotData,
        data: Option<&NSData>,
    ) -> Bool;
}

extern "C" {
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub fn NSDataFromGCExtendedGamepadSnapshotData(
        snapshot_data: *mut GCExtendedGamepadSnapshotData,
    ) -> *mut NSData;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GCExtendedGamepadSnapShotDataV100 {
    pub version: u16,
    pub size: u16,
    pub dpadX: c_float,
    pub dpadY: c_float,
    pub buttonA: c_float,
    pub buttonB: c_float,
    pub buttonX: c_float,
    pub buttonY: c_float,
    pub leftShoulder: c_float,
    pub rightShoulder: c_float,
    pub leftThumbstickX: c_float,
    pub leftThumbstickY: c_float,
    pub rightThumbstickX: c_float,
    pub rightThumbstickY: c_float,
    pub leftTrigger: c_float,
    pub rightTrigger: c_float,
}

unsafe impl Encode for GCExtendedGamepadSnapShotDataV100 {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u16>::ENCODING,
            <u16>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GCExtendedGamepadSnapShotDataV100 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub fn GCExtendedGamepadSnapShotDataV100FromNSData(
        snapshot_data: *mut GCExtendedGamepadSnapShotDataV100,
        data: Option<&NSData>,
    ) -> Bool;
}

extern "C" {
    #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController controllerWithExtendedGamepad] instead"]
    pub fn NSDataFromGCExtendedGamepadSnapShotDataV100(
        snapshot_data: *mut GCExtendedGamepadSnapShotDataV100,
    ) -> *mut NSData;
}
