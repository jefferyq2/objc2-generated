//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKBiologicalSexObject;

    unsafe impl ClassType for HKBiologicalSexObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for HKBiologicalSexObject {}

unsafe impl NSCopying for HKBiologicalSexObject {}

unsafe impl NSObjectProtocol for HKBiologicalSexObject {}

unsafe impl NSSecureCoding for HKBiologicalSexObject {}

extern_methods!(
    unsafe impl HKBiologicalSexObject {
        #[cfg(feature = "HKCharacteristicValues")]
        #[method(biologicalSex)]
        pub unsafe fn biologicalSex(&self) -> HKBiologicalSex;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKBiologicalSexObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKBloodTypeObject;

    unsafe impl ClassType for HKBloodTypeObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for HKBloodTypeObject {}

unsafe impl NSCopying for HKBloodTypeObject {}

unsafe impl NSObjectProtocol for HKBloodTypeObject {}

unsafe impl NSSecureCoding for HKBloodTypeObject {}

extern_methods!(
    unsafe impl HKBloodTypeObject {
        #[cfg(feature = "HKCharacteristicValues")]
        #[method(bloodType)]
        pub unsafe fn bloodType(&self) -> HKBloodType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKBloodTypeObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKFitzpatrickSkinTypeObject;

    unsafe impl ClassType for HKFitzpatrickSkinTypeObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for HKFitzpatrickSkinTypeObject {}

unsafe impl NSCopying for HKFitzpatrickSkinTypeObject {}

unsafe impl NSObjectProtocol for HKFitzpatrickSkinTypeObject {}

unsafe impl NSSecureCoding for HKFitzpatrickSkinTypeObject {}

extern_methods!(
    unsafe impl HKFitzpatrickSkinTypeObject {
        #[cfg(feature = "HKCharacteristicValues")]
        #[method(skinType)]
        pub unsafe fn skinType(&self) -> HKFitzpatrickSkinType;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKFitzpatrickSkinTypeObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKWheelchairUseObject;

    unsafe impl ClassType for HKWheelchairUseObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for HKWheelchairUseObject {}

unsafe impl NSCopying for HKWheelchairUseObject {}

unsafe impl NSObjectProtocol for HKWheelchairUseObject {}

unsafe impl NSSecureCoding for HKWheelchairUseObject {}

extern_methods!(
    unsafe impl HKWheelchairUseObject {
        #[cfg(feature = "HKCharacteristicValues")]
        #[method(wheelchairUse)]
        pub unsafe fn wheelchairUse(&self) -> HKWheelchairUse;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKWheelchairUseObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKActivityMoveModeObject;

    unsafe impl ClassType for HKActivityMoveModeObject {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for HKActivityMoveModeObject {}

unsafe impl NSCopying for HKActivityMoveModeObject {}

unsafe impl NSObjectProtocol for HKActivityMoveModeObject {}

unsafe impl NSSecureCoding for HKActivityMoveModeObject {}

extern_methods!(
    unsafe impl HKActivityMoveModeObject {
        #[cfg(feature = "HKCharacteristicValues")]
        #[method(activityMoveMode)]
        pub unsafe fn activityMoveMode(&self) -> HKActivityMoveMode;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKActivityMoveModeObject {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
