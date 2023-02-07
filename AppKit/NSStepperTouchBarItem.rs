//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSStepperTouchBarItem")]
    pub struct NSStepperTouchBarItem;

    #[cfg(feature = "AppKit_NSStepperTouchBarItem")]
    unsafe impl ClassType for NSStepperTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
    }
);

#[cfg(feature = "AppKit_NSStepperTouchBarItem")]
unsafe impl NSCoding for NSStepperTouchBarItem {}

#[cfg(feature = "AppKit_NSStepperTouchBarItem")]
unsafe impl NSObjectProtocol for NSStepperTouchBarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSStepperTouchBarItem")]
    unsafe impl NSStepperTouchBarItem {
        #[cfg(feature = "Foundation_NSFormatter")]
        #[method_id(@__retain_semantics Other stepperTouchBarItemWithIdentifier:formatter:)]
        pub unsafe fn stepperTouchBarItemWithIdentifier_formatter(
            identifier: &NSTouchBarItemIdentifier,
            formatter: &NSFormatter,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other stepperTouchBarItemWithIdentifier:drawingHandler:)]
        pub unsafe fn stepperTouchBarItemWithIdentifier_drawingHandler(
            identifier: &NSTouchBarItemIdentifier,
            drawing_handler: &Block<(NSRect, c_double), ()>,
        ) -> Id<Self>;

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[method(increment)]
        pub unsafe fn increment(&self) -> c_double;

        #[method(setIncrement:)]
        pub unsafe fn setIncrement(&self, increment: c_double);

        #[method(value)]
        pub unsafe fn value(&self) -> c_double;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_double);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSStepperTouchBarItem")]
    unsafe impl NSStepperTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self>;
    }
);
