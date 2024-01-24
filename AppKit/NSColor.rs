//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSAppKitVersionNumberWithPatternColorLeakFix: NSAppKitVersion = 641.0);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSColorType {
        NSColorTypeComponentBased = 0,
        NSColorTypePattern = 1,
        NSColorTypeCatalog = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSColorSystemEffect {
        NSColorSystemEffectNone = 0,
        NSColorSystemEffectPressed = 1,
        NSColorSystemEffectDeepPressed = 2,
        NSColorSystemEffectDisabled = 3,
        NSColorSystemEffectRollover = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSColor")]
    pub struct NSColor;

    #[cfg(feature = "AppKit_NSColor")]
    unsafe impl ClassType for NSColor {
        type Super = NSObject;
        type Mutability = Immutable;
    }
);

#[cfg(feature = "AppKit_NSColor")]
unsafe impl Send for NSColor {}

#[cfg(feature = "AppKit_NSColor")]
unsafe impl Sync for NSColor {}

#[cfg(feature = "AppKit_NSColor")]
unsafe impl NSCoding for NSColor {}

#[cfg(feature = "AppKit_NSColor")]
unsafe impl NSCopying for NSColor {}

#[cfg(feature = "AppKit_NSColor")]
unsafe impl NSObjectProtocol for NSColor {}

#[cfg(feature = "AppKit_NSColor")]
unsafe impl NSPasteboardReading for NSColor {}

#[cfg(feature = "AppKit_NSColor")]
unsafe impl NSPasteboardWriting for NSColor {}

#[cfg(feature = "AppKit_NSColor")]
unsafe impl NSSecureCoding for NSColor {}

extern_methods!(
    #[cfg(feature = "AppKit_NSColor")]
    unsafe impl NSColor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other colorWithColorSpace:components:count:)]
        pub unsafe fn colorWithColorSpace_components_count(
            space: &NSColorSpace,
            components: NonNull<CGFloat>,
            number_of_components: NSInteger,
        ) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithSRGBRed:green:blue:alpha:)]
        pub unsafe fn colorWithSRGBRed_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithGenericGamma22White:alpha:)]
        pub unsafe fn colorWithGenericGamma22White_alpha(
            white: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithDisplayP3Red:green:blue:alpha:)]
        pub unsafe fn colorWithDisplayP3Red_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithWhite:alpha:)]
        pub unsafe fn colorWithWhite_alpha(white: CGFloat, alpha: CGFloat) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithRed:green:blue:alpha:)]
        pub unsafe fn colorWithRed_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithHue:saturation:brightness:alpha:)]
        pub unsafe fn colorWithHue_saturation_brightness_alpha(
            hue: CGFloat,
            saturation: CGFloat,
            brightness: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other colorWithColorSpace:hue:saturation:brightness:alpha:)]
        pub unsafe fn colorWithColorSpace_hue_saturation_brightness_alpha(
            space: &NSColorSpace,
            hue: CGFloat,
            saturation: CGFloat,
            brightness: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithCatalogName:colorName:)]
        pub unsafe fn colorWithCatalogName_colorName(
            list_name: &NSColorListName,
            color_name: &NSColorName,
        ) -> Option<Id<NSColor>>;

        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Other colorNamed:bundle:)]
        pub unsafe fn colorNamed_bundle(
            name: &NSColorName,
            bundle: Option<&NSBundle>,
        ) -> Option<Id<NSColor>>;

        #[method_id(@__retain_semantics Other colorNamed:)]
        pub unsafe fn colorNamed(name: &NSColorName) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSAppearance")]
        #[method_id(@__retain_semantics Other colorWithName:dynamicProvider:)]
        pub unsafe fn colorWithName_dynamicProvider(
            color_name: Option<&NSColorName>,
            dynamic_provider: &Block<dyn Fn(NonNull<NSAppearance>) -> NonNull<NSColor>>,
        ) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithDeviceWhite:alpha:)]
        pub unsafe fn colorWithDeviceWhite_alpha(white: CGFloat, alpha: CGFloat) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithDeviceRed:green:blue:alpha:)]
        pub unsafe fn colorWithDeviceRed_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithDeviceHue:saturation:brightness:alpha:)]
        pub unsafe fn colorWithDeviceHue_saturation_brightness_alpha(
            hue: CGFloat,
            saturation: CGFloat,
            brightness: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithDeviceCyan:magenta:yellow:black:alpha:)]
        pub unsafe fn colorWithDeviceCyan_magenta_yellow_black_alpha(
            cyan: CGFloat,
            magenta: CGFloat,
            yellow: CGFloat,
            black: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithCalibratedWhite:alpha:)]
        pub unsafe fn colorWithCalibratedWhite_alpha(white: CGFloat, alpha: CGFloat)
            -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithCalibratedRed:green:blue:alpha:)]
        pub unsafe fn colorWithCalibratedRed_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other colorWithCalibratedHue:saturation:brightness:alpha:)]
        pub unsafe fn colorWithCalibratedHue_saturation_brightness_alpha(
            hue: CGFloat,
            saturation: CGFloat,
            brightness: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other colorWithPatternImage:)]
        pub unsafe fn colorWithPatternImage(image: &NSImage) -> Id<NSColor>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> NSColorType;

        #[method_id(@__retain_semantics Other colorUsingType:)]
        pub unsafe fn colorUsingType(&self, r#type: NSColorType) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other colorUsingColorSpace:)]
        pub unsafe fn colorUsingColorSpace(&self, space: &NSColorSpace) -> Option<Id<NSColor>>;

        #[method_id(@__retain_semantics Other blackColor)]
        pub unsafe fn blackColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other darkGrayColor)]
        pub unsafe fn darkGrayColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other lightGrayColor)]
        pub unsafe fn lightGrayColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other whiteColor)]
        pub unsafe fn whiteColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other grayColor)]
        pub unsafe fn grayColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other redColor)]
        pub unsafe fn redColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other greenColor)]
        pub unsafe fn greenColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other blueColor)]
        pub unsafe fn blueColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other cyanColor)]
        pub unsafe fn cyanColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other yellowColor)]
        pub unsafe fn yellowColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other magentaColor)]
        pub unsafe fn magentaColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other orangeColor)]
        pub unsafe fn orangeColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other purpleColor)]
        pub unsafe fn purpleColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other brownColor)]
        pub unsafe fn brownColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other clearColor)]
        pub unsafe fn clearColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other labelColor)]
        pub unsafe fn labelColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other secondaryLabelColor)]
        pub unsafe fn secondaryLabelColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other tertiaryLabelColor)]
        pub unsafe fn tertiaryLabelColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other quaternaryLabelColor)]
        pub unsafe fn quaternaryLabelColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other quinaryLabelColor)]
        pub unsafe fn quinaryLabelColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other linkColor)]
        pub unsafe fn linkColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other placeholderTextColor)]
        pub unsafe fn placeholderTextColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other windowFrameTextColor)]
        pub unsafe fn windowFrameTextColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other selectedMenuItemTextColor)]
        pub unsafe fn selectedMenuItemTextColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other alternateSelectedControlTextColor)]
        pub unsafe fn alternateSelectedControlTextColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other headerTextColor)]
        pub unsafe fn headerTextColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other separatorColor)]
        pub unsafe fn separatorColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other gridColor)]
        pub unsafe fn gridColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other windowBackgroundColor)]
        pub unsafe fn windowBackgroundColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other underPageBackgroundColor)]
        pub unsafe fn underPageBackgroundColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other controlBackgroundColor)]
        pub unsafe fn controlBackgroundColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other selectedContentBackgroundColor)]
        pub unsafe fn selectedContentBackgroundColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other unemphasizedSelectedContentBackgroundColor)]
        pub unsafe fn unemphasizedSelectedContentBackgroundColor() -> Id<NSColor>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other alternatingContentBackgroundColors)]
        pub unsafe fn alternatingContentBackgroundColors() -> Id<NSArray<NSColor>>;

        #[method_id(@__retain_semantics Other findHighlightColor)]
        pub unsafe fn findHighlightColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other textBackgroundColor)]
        pub unsafe fn textBackgroundColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other textInsertionPointColor)]
        pub unsafe fn textInsertionPointColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other selectedTextColor)]
        pub unsafe fn selectedTextColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other selectedTextBackgroundColor)]
        pub unsafe fn selectedTextBackgroundColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other unemphasizedSelectedTextBackgroundColor)]
        pub unsafe fn unemphasizedSelectedTextBackgroundColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other unemphasizedSelectedTextColor)]
        pub unsafe fn unemphasizedSelectedTextColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other controlColor)]
        pub unsafe fn controlColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other controlTextColor)]
        pub unsafe fn controlTextColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other selectedControlColor)]
        pub unsafe fn selectedControlColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other selectedControlTextColor)]
        pub unsafe fn selectedControlTextColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other disabledControlTextColor)]
        pub unsafe fn disabledControlTextColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other keyboardFocusIndicatorColor)]
        pub unsafe fn keyboardFocusIndicatorColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other scrubberTexturedBackgroundColor)]
        pub unsafe fn scrubberTexturedBackgroundColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemRedColor)]
        pub unsafe fn systemRedColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemGreenColor)]
        pub unsafe fn systemGreenColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemBlueColor)]
        pub unsafe fn systemBlueColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemOrangeColor)]
        pub unsafe fn systemOrangeColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemYellowColor)]
        pub unsafe fn systemYellowColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemBrownColor)]
        pub unsafe fn systemBrownColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemPinkColor)]
        pub unsafe fn systemPinkColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemPurpleColor)]
        pub unsafe fn systemPurpleColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemGrayColor)]
        pub unsafe fn systemGrayColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemTealColor)]
        pub unsafe fn systemTealColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemIndigoColor)]
        pub unsafe fn systemIndigoColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemMintColor)]
        pub unsafe fn systemMintColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemCyanColor)]
        pub unsafe fn systemCyanColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other systemFillColor)]
        pub unsafe fn systemFillColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other secondarySystemFillColor)]
        pub unsafe fn secondarySystemFillColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other tertiarySystemFillColor)]
        pub unsafe fn tertiarySystemFillColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other quaternarySystemFillColor)]
        pub unsafe fn quaternarySystemFillColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other quinarySystemFillColor)]
        pub unsafe fn quinarySystemFillColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other controlAccentColor)]
        pub unsafe fn controlAccentColor() -> Id<NSColor>;

        #[method(currentControlTint)]
        pub unsafe fn currentControlTint() -> NSControlTint;

        #[deprecated = "NSControlTint does not describe the full range of available control accent colors. Use +[NSColor controlAccentColor] instead."]
        #[method_id(@__retain_semantics Other colorForControlTint:)]
        pub unsafe fn colorForControlTint(control_tint: NSControlTint) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other highlightColor)]
        pub unsafe fn highlightColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other shadowColor)]
        pub unsafe fn shadowColor() -> Id<NSColor>;

        #[method_id(@__retain_semantics Other highlightWithLevel:)]
        pub unsafe fn highlightWithLevel(&self, val: CGFloat) -> Option<Id<NSColor>>;

        #[method_id(@__retain_semantics Other shadowWithLevel:)]
        pub unsafe fn shadowWithLevel(&self, val: CGFloat) -> Option<Id<NSColor>>;

        #[method_id(@__retain_semantics Other colorWithSystemEffect:)]
        pub unsafe fn colorWithSystemEffect(
            &self,
            system_effect: NSColorSystemEffect,
        ) -> Id<NSColor>;

        #[method(set)]
        pub unsafe fn set(&self);

        #[method(setFill)]
        pub unsafe fn setFill(&self);

        #[method(setStroke)]
        pub unsafe fn setStroke(&self);

        #[method_id(@__retain_semantics Other blendedColorWithFraction:ofColor:)]
        pub unsafe fn blendedColorWithFraction_ofColor(
            &self,
            fraction: CGFloat,
            color: &NSColor,
        ) -> Option<Id<NSColor>>;

        #[method_id(@__retain_semantics Other colorWithAlphaComponent:)]
        pub unsafe fn colorWithAlphaComponent(&self, alpha: CGFloat) -> Id<NSColor>;

        #[method_id(@__retain_semantics Other catalogNameComponent)]
        pub unsafe fn catalogNameComponent(&self) -> Id<NSColorListName>;

        #[method_id(@__retain_semantics Other colorNameComponent)]
        pub unsafe fn colorNameComponent(&self) -> Id<NSColorName>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedCatalogNameComponent)]
        pub unsafe fn localizedCatalogNameComponent(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedColorNameComponent)]
        pub unsafe fn localizedColorNameComponent(&self) -> Id<NSString>;

        #[method(redComponent)]
        pub unsafe fn redComponent(&self) -> CGFloat;

        #[method(greenComponent)]
        pub unsafe fn greenComponent(&self) -> CGFloat;

        #[method(blueComponent)]
        pub unsafe fn blueComponent(&self) -> CGFloat;

        #[method(getRed:green:blue:alpha:)]
        pub unsafe fn getRed_green_blue_alpha(
            &self,
            red: *mut CGFloat,
            green: *mut CGFloat,
            blue: *mut CGFloat,
            alpha: *mut CGFloat,
        );

        #[method(hueComponent)]
        pub unsafe fn hueComponent(&self) -> CGFloat;

        #[method(saturationComponent)]
        pub unsafe fn saturationComponent(&self) -> CGFloat;

        #[method(brightnessComponent)]
        pub unsafe fn brightnessComponent(&self) -> CGFloat;

        #[method(getHue:saturation:brightness:alpha:)]
        pub unsafe fn getHue_saturation_brightness_alpha(
            &self,
            hue: *mut CGFloat,
            saturation: *mut CGFloat,
            brightness: *mut CGFloat,
            alpha: *mut CGFloat,
        );

        #[method(whiteComponent)]
        pub unsafe fn whiteComponent(&self) -> CGFloat;

        #[method(getWhite:alpha:)]
        pub unsafe fn getWhite_alpha(&self, white: *mut CGFloat, alpha: *mut CGFloat);

        #[method(cyanComponent)]
        pub unsafe fn cyanComponent(&self) -> CGFloat;

        #[method(magentaComponent)]
        pub unsafe fn magentaComponent(&self) -> CGFloat;

        #[method(yellowComponent)]
        pub unsafe fn yellowComponent(&self) -> CGFloat;

        #[method(blackComponent)]
        pub unsafe fn blackComponent(&self) -> CGFloat;

        #[method(getCyan:magenta:yellow:black:alpha:)]
        pub unsafe fn getCyan_magenta_yellow_black_alpha(
            &self,
            cyan: *mut CGFloat,
            magenta: *mut CGFloat,
            yellow: *mut CGFloat,
            black: *mut CGFloat,
            alpha: *mut CGFloat,
        );

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other colorSpace)]
        pub unsafe fn colorSpace(&self) -> Id<NSColorSpace>;

        #[method(numberOfComponents)]
        pub unsafe fn numberOfComponents(&self) -> NSInteger;

        #[method(getComponents:)]
        pub unsafe fn getComponents(&self, components: NonNull<CGFloat>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other patternImage)]
        pub unsafe fn patternImage(&self) -> Id<NSImage>;

        #[method(alphaComponent)]
        pub unsafe fn alphaComponent(&self) -> CGFloat;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Other colorFromPasteboard:)]
        pub unsafe fn colorFromPasteboard(paste_board: &NSPasteboard) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(writeToPasteboard:)]
        pub unsafe fn writeToPasteboard(&self, paste_board: &NSPasteboard);

        #[method(drawSwatchInRect:)]
        pub unsafe fn drawSwatchInRect(&self, rect: NSRect);

        #[deprecated = "Use `showsAlpha` in `NSColorPanel` and `supportsAlpha` in `NSColorWell` to control alpha behavior for individual controls."]
        #[method(ignoresAlpha)]
        pub unsafe fn ignoresAlpha(mtm: MainThreadMarker) -> bool;

        #[deprecated = "Use `showsAlpha` in `NSColorPanel` and `supportsAlpha` in `NSColorWell` to control alpha behavior for individual controls."]
        #[method(setIgnoresAlpha:)]
        pub unsafe fn setIgnoresAlpha(ignores_alpha: bool, mtm: MainThreadMarker);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSColor")]
    unsafe impl NSColor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSColor")]
    unsafe impl NSColor {
        #[deprecated = "Use a color that matches the semantics being used, such as `separatorColor`"]
        #[method_id(@__retain_semantics Other controlHighlightColor)]
        pub unsafe fn controlHighlightColor() -> Id<NSColor>;

        #[deprecated = "Use a color that matches the semantics being used, such as `separatorColor`"]
        #[method_id(@__retain_semantics Other controlLightHighlightColor)]
        pub unsafe fn controlLightHighlightColor() -> Id<NSColor>;

        #[deprecated = "Use a color that matches the semantics being used, such as `separatorColor`"]
        #[method_id(@__retain_semantics Other controlShadowColor)]
        pub unsafe fn controlShadowColor() -> Id<NSColor>;

        #[deprecated = "Use a color that matches the semantics being used, such as `separatorColor`"]
        #[method_id(@__retain_semantics Other controlDarkShadowColor)]
        pub unsafe fn controlDarkShadowColor() -> Id<NSColor>;

        #[deprecated = "Use NSScroller instead"]
        #[method_id(@__retain_semantics Other scrollBarColor)]
        pub unsafe fn scrollBarColor() -> Id<NSColor>;

        #[deprecated = "Use NSScroller instead"]
        #[method_id(@__retain_semantics Other knobColor)]
        pub unsafe fn knobColor() -> Id<NSColor>;

        #[deprecated = "Use NSScroller instead"]
        #[method_id(@__retain_semantics Other selectedKnobColor)]
        pub unsafe fn selectedKnobColor() -> Id<NSColor>;

        #[deprecated = "Use NSVisualEffectMaterialTitlebar"]
        #[method_id(@__retain_semantics Other windowFrameColor)]
        pub unsafe fn windowFrameColor() -> Id<NSColor>;

        #[deprecated = "Use NSVisualEffectMaterialSelection"]
        #[method_id(@__retain_semantics Other selectedMenuItemColor)]
        pub unsafe fn selectedMenuItemColor() -> Id<NSColor>;

        #[deprecated = "Use NSVisualEffectMaterialHeaderView"]
        #[method_id(@__retain_semantics Other headerColor)]
        pub unsafe fn headerColor() -> Id<NSColor>;

        #[deprecated]
        #[method_id(@__retain_semantics Other secondarySelectedControlColor)]
        pub unsafe fn secondarySelectedControlColor() -> Id<NSColor>;

        #[deprecated]
        #[method_id(@__retain_semantics Other alternateSelectedControlColor)]
        pub unsafe fn alternateSelectedControlColor() -> Id<NSColor>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated]
        #[method_id(@__retain_semantics Other controlAlternatingRowBackgroundColors)]
        pub unsafe fn controlAlternatingRowBackgroundColors() -> Id<NSArray<NSColor>>;

        #[deprecated = "Use -type and NSColorType instead"]
        #[method_id(@__retain_semantics Other colorSpaceName)]
        pub unsafe fn colorSpaceName(&self) -> Id<NSColorSpaceName>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[deprecated = "Use -colorUsingType: or -colorUsingColorSpace: instead"]
        #[method_id(@__retain_semantics Other colorUsingColorSpaceName:device:)]
        pub unsafe fn colorUsingColorSpaceName_device(
            &self,
            name: Option<&NSColorSpaceName>,
            device_description: Option<&NSDictionary<NSDeviceDescriptionKey, AnyObject>>,
        ) -> Option<Id<NSColor>>;

        #[deprecated = "Use -colorUsingType: or -colorUsingColorSpace: instead"]
        #[method_id(@__retain_semantics Other colorUsingColorSpaceName:)]
        pub unsafe fn colorUsingColorSpaceName(
            &self,
            name: &NSColorSpaceName,
        ) -> Option<Id<NSColor>>;
    }
);

extern_methods!(
    /// NSQuartzCoreAdditions
    #[cfg(feature = "AppKit_NSColor")]
    unsafe impl NSColor {}
);

extern_methods!(
    /// NSAppKitColorExtensions
    #[cfg(feature = "Foundation_NSCoder")]
    unsafe impl NSCoder {
        #[cfg(feature = "AppKit_NSColor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other decodeNXColor)]
        pub unsafe fn decodeNXColor(&self) -> Option<Id<NSColor>>;
    }
);

extern_static!(NSSystemColorsDidChangeNotification: &'static NSNotificationName);
