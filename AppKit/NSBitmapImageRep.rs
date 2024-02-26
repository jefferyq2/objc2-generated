//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTIFFCompression {
        #[doc(alias = "NSTIFFCompressionNone")]
        None = 1,
        #[doc(alias = "NSTIFFCompressionCCITTFAX3")]
        CCITTFAX3 = 3,
        #[doc(alias = "NSTIFFCompressionCCITTFAX4")]
        CCITTFAX4 = 4,
        #[doc(alias = "NSTIFFCompressionLZW")]
        LZW = 5,
        #[doc(alias = "NSTIFFCompressionJPEG")]
        JPEG = 6,
        #[doc(alias = "NSTIFFCompressionNEXT")]
        NEXT = 32766,
        #[doc(alias = "NSTIFFCompressionPackBits")]
        PackBits = 32773,
        #[doc(alias = "NSTIFFCompressionOldJPEG")]
        OldJPEG = 32865,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBitmapImageFileType {
        #[doc(alias = "NSBitmapImageFileTypeTIFF")]
        TIFF = 0,
        #[doc(alias = "NSBitmapImageFileTypeBMP")]
        BMP = 1,
        #[doc(alias = "NSBitmapImageFileTypeGIF")]
        GIF = 2,
        #[doc(alias = "NSBitmapImageFileTypeJPEG")]
        JPEG = 3,
        #[doc(alias = "NSBitmapImageFileTypePNG")]
        PNG = 4,
        #[doc(alias = "NSBitmapImageFileTypeJPEG2000")]
        JPEG2000 = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSImageRepLoadStatus {
        #[doc(alias = "NSImageRepLoadStatusUnknownType")]
        UnknownType = -1,
        #[doc(alias = "NSImageRepLoadStatusReadingHeader")]
        ReadingHeader = -2,
        #[doc(alias = "NSImageRepLoadStatusWillNeedAllData")]
        WillNeedAllData = -3,
        #[doc(alias = "NSImageRepLoadStatusInvalidData")]
        InvalidData = -4,
        #[doc(alias = "NSImageRepLoadStatusUnexpectedEOF")]
        UnexpectedEOF = -5,
        #[doc(alias = "NSImageRepLoadStatusCompleted")]
        Completed = -6,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSBitmapFormat {
        #[doc(alias = "NSBitmapFormatAlphaFirst")]
        AlphaFirst = 1 << 0,
        #[doc(alias = "NSBitmapFormatAlphaNonpremultiplied")]
        AlphaNonpremultiplied = 1 << 1,
        #[doc(alias = "NSBitmapFormatFloatingPointSamples")]
        FloatingPointSamples = 1 << 2,
        #[doc(alias = "NSBitmapFormatSixteenBitLittleEndian")]
        SixteenBitLittleEndian = 1 << 8,
        #[doc(alias = "NSBitmapFormatThirtyTwoBitLittleEndian")]
        ThirtyTwoBitLittleEndian = 1 << 9,
        #[doc(alias = "NSBitmapFormatSixteenBitBigEndian")]
        SixteenBitBigEndian = 1 << 10,
        #[doc(alias = "NSBitmapFormatThirtyTwoBitBigEndian")]
        ThirtyTwoBitBigEndian = 1 << 11,
    }
);

#[cfg(feature = "Foundation_NSString")]
typed_extensible_enum!(
    pub type NSBitmapImageRepPropertyKey = NSString;
);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageCompressionMethod: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageCompressionFactor: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageDitherTransparency: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageRGBColorTable: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageInterlaced: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageColorSyncProfileData: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageFrameCount: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageCurrentFrame: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageCurrentFrameDuration: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageLoopCount: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageGamma: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageProgressive: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageEXIFData: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageIPTCData: &'static NSBitmapImageRepPropertyKey);

#[cfg(feature = "Foundation_NSString")]
extern_static!(NSImageFallbackBackgroundColor: &'static NSBitmapImageRepPropertyKey);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSBitmapImageRep")]
    pub struct NSBitmapImageRep;

    #[cfg(feature = "AppKit_NSBitmapImageRep")]
    unsafe impl ClassType for NSBitmapImageRep {
        #[inherits(NSObject)]
        type Super = NSImageRep;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSBitmapImageRep")]
unsafe impl NSCoding for NSBitmapImageRep {}

#[cfg(feature = "AppKit_NSBitmapImageRep")]
unsafe impl NSCopying for NSBitmapImageRep {}

#[cfg(feature = "AppKit_NSBitmapImageRep")]
unsafe impl NSObjectProtocol for NSBitmapImageRep {}

#[cfg(feature = "AppKit_NSBitmapImageRep")]
unsafe impl NSSecureCoding for NSBitmapImageRep {}

extern_methods!(
    #[cfg(feature = "AppKit_NSBitmapImageRep")]
    unsafe impl NSBitmapImageRep {
        #[deprecated = "Use -[NSView cacheDisplayInRect:toBitmapImageRep:] to snapshot a view."]
        #[method_id(@__retain_semantics Init initWithFocusedViewRect:)]
        pub unsafe fn initWithFocusedViewRect(
            this: Allocated<Self>,
            rect: NSRect,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithBitmapDataPlanes:pixelsWide:pixelsHigh:bitsPerSample:samplesPerPixel:hasAlpha:isPlanar:colorSpaceName:bytesPerRow:bitsPerPixel:)]
        pub unsafe fn initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bytesPerRow_bitsPerPixel(
            this: Allocated<Self>,
            planes: *mut *mut c_uchar,
            width: NSInteger,
            height: NSInteger,
            bps: NSInteger,
            spp: NSInteger,
            alpha: bool,
            is_planar: bool,
            color_space_name: &NSColorSpaceName,
            r_bytes: NSInteger,
            p_bits: NSInteger,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithBitmapDataPlanes:pixelsWide:pixelsHigh:bitsPerSample:samplesPerPixel:hasAlpha:isPlanar:colorSpaceName:bitmapFormat:bytesPerRow:bitsPerPixel:)]
        pub unsafe fn initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bitmapFormat_bytesPerRow_bitsPerPixel(
            this: Allocated<Self>,
            planes: *mut *mut c_uchar,
            width: NSInteger,
            height: NSInteger,
            bps: NSInteger,
            spp: NSInteger,
            alpha: bool,
            is_planar: bool,
            color_space_name: &NSColorSpaceName,
            bitmap_format: NSBitmapFormat,
            r_bytes: NSInteger,
            p_bits: NSInteger,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[method_id(@__retain_semantics Other imageRepsWithData:)]
        pub unsafe fn imageRepsWithData(data: &NSData) -> Id<NSArray<NSImageRep>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other imageRepWithData:)]
        pub unsafe fn imageRepWithData(data: &NSData) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(this: Allocated<Self>, data: &NSData) -> Option<Id<Self>>;

        #[method(bitmapData)]
        pub unsafe fn bitmapData(&self) -> *mut c_uchar;

        #[method(getBitmapDataPlanes:)]
        pub unsafe fn getBitmapDataPlanes(&self, data: NonNull<*mut c_uchar>);

        #[method(isPlanar)]
        pub unsafe fn isPlanar(&self) -> bool;

        #[method(samplesPerPixel)]
        pub unsafe fn samplesPerPixel(&self) -> NSInteger;

        #[method(bitsPerPixel)]
        pub unsafe fn bitsPerPixel(&self) -> NSInteger;

        #[method(bytesPerRow)]
        pub unsafe fn bytesPerRow(&self) -> NSInteger;

        #[method(bytesPerPlane)]
        pub unsafe fn bytesPerPlane(&self) -> NSInteger;

        #[method(numberOfPlanes)]
        pub unsafe fn numberOfPlanes(&self) -> NSInteger;

        #[method(bitmapFormat)]
        pub unsafe fn bitmapFormat(&self) -> NSBitmapFormat;

        #[method(getCompression:factor:)]
        pub unsafe fn getCompression_factor(
            &self,
            compression: *mut NSTIFFCompression,
            factor: *mut c_float,
        );

        #[method(setCompression:factor:)]
        pub unsafe fn setCompression_factor(&self, compression: NSTIFFCompression, factor: c_float);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other TIFFRepresentation)]
        pub unsafe fn TIFFRepresentation(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other TIFFRepresentationUsingCompression:factor:)]
        pub unsafe fn TIFFRepresentationUsingCompression_factor(
            &self,
            comp: NSTIFFCompression,
            factor: c_float,
        ) -> Option<Id<NSData>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[method_id(@__retain_semantics Other TIFFRepresentationOfImageRepsInArray:)]
        pub unsafe fn TIFFRepresentationOfImageRepsInArray(
            array: &NSArray<NSImageRep>,
        ) -> Option<Id<NSData>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[method_id(@__retain_semantics Other TIFFRepresentationOfImageRepsInArray:usingCompression:factor:)]
        pub unsafe fn TIFFRepresentationOfImageRepsInArray_usingCompression_factor(
            array: &NSArray<NSImageRep>,
            comp: NSTIFFCompression,
            factor: c_float,
        ) -> Option<Id<NSData>>;

        #[method(getTIFFCompressionTypes:count:)]
        pub unsafe fn getTIFFCompressionTypes_count(
            list: NonNull<*mut NSTIFFCompression>,
            num_types: NonNull<NSInteger>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedNameForTIFFCompressionType:)]
        pub unsafe fn localizedNameForTIFFCompressionType(
            compression: NSTIFFCompression,
        ) -> Option<Id<NSString>>;

        #[method(canBeCompressedUsing:)]
        pub unsafe fn canBeCompressedUsing(&self, compression: NSTIFFCompression) -> bool;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(colorizeByMappingGray:toColor:blackMapping:whiteMapping:)]
        pub unsafe fn colorizeByMappingGray_toColor_blackMapping_whiteMapping(
            &self,
            mid_point: CGFloat,
            mid_point_color: Option<&NSColor>,
            shadow_color: Option<&NSColor>,
            light_color: Option<&NSColor>,
        );

        #[method_id(@__retain_semantics Init initForIncrementalLoad)]
        pub unsafe fn initForIncrementalLoad(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(incrementalLoadFromData:complete:)]
        pub unsafe fn incrementalLoadFromData_complete(
            &self,
            data: &NSData,
            complete: bool,
        ) -> NSInteger;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setColor:atX:y:)]
        pub unsafe fn setColor_atX_y(&self, color: &NSColor, x: NSInteger, y: NSInteger);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other colorAtX:y:)]
        pub unsafe fn colorAtX_y(&self, x: NSInteger, y: NSInteger) -> Option<Id<NSColor>>;

        #[method(getPixel:atX:y:)]
        pub unsafe fn getPixel_atX_y(&self, p: NonNull<NSUInteger>, x: NSInteger, y: NSInteger);

        #[method(setPixel:atX:y:)]
        pub unsafe fn setPixel_atX_y(&self, p: NonNull<NSUInteger>, x: NSInteger, y: NSInteger);

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other colorSpace)]
        pub unsafe fn colorSpace(&self) -> Id<NSColorSpace>;

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other bitmapImageRepByConvertingToColorSpace:renderingIntent:)]
        pub unsafe fn bitmapImageRepByConvertingToColorSpace_renderingIntent(
            &self,
            target_space: &NSColorSpace,
            rendering_intent: NSColorRenderingIntent,
        ) -> Option<Id<NSBitmapImageRep>>;

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other bitmapImageRepByRetaggingWithColorSpace:)]
        pub unsafe fn bitmapImageRepByRetaggingWithColorSpace(
            &self,
            new_space: &NSColorSpace,
        ) -> Option<Id<NSBitmapImageRep>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSImageRep`
    #[cfg(feature = "AppKit_NSBitmapImageRep")]
    unsafe impl NSBitmapImageRep {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSBitmapImageRep")]
    unsafe impl NSBitmapImageRep {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSBitmapImageFileTypeExtensions
    #[cfg(feature = "AppKit_NSBitmapImageRep")]
    unsafe impl NSBitmapImageRep {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other representationOfImageRepsInArray:usingType:properties:)]
        pub unsafe fn representationOfImageRepsInArray_usingType_properties(
            image_reps: &NSArray<NSImageRep>,
            storage_type: NSBitmapImageFileType,
            properties: &NSDictionary<NSBitmapImageRepPropertyKey, AnyObject>,
        ) -> Option<Id<NSData>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other representationUsingType:properties:)]
        pub unsafe fn representationUsingType_properties(
            &self,
            storage_type: NSBitmapImageFileType,
            properties: &NSDictionary<NSBitmapImageRepPropertyKey, AnyObject>,
        ) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setProperty:withValue:)]
        pub unsafe fn setProperty_withValue(
            &self,
            property: &NSBitmapImageRepPropertyKey,
            value: Option<&AnyObject>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueForProperty:)]
        pub unsafe fn valueForProperty(
            &self,
            property: &NSBitmapImageRepPropertyKey,
        ) -> Option<Id<AnyObject>>;
    }
);

extern_static!(NSTIFFFileType: NSBitmapImageFileType = NSBitmapImageFileType(NSBitmapImageFileType::TIFF.0));

extern_static!(NSBMPFileType: NSBitmapImageFileType = NSBitmapImageFileType(NSBitmapImageFileType::BMP.0));

extern_static!(NSGIFFileType: NSBitmapImageFileType = NSBitmapImageFileType(NSBitmapImageFileType::GIF.0));

extern_static!(NSJPEGFileType: NSBitmapImageFileType = NSBitmapImageFileType(NSBitmapImageFileType::JPEG.0));

extern_static!(NSPNGFileType: NSBitmapImageFileType = NSBitmapImageFileType(NSBitmapImageFileType::PNG.0));

extern_static!(NSJPEG2000FileType: NSBitmapImageFileType = NSBitmapImageFileType(NSBitmapImageFileType::JPEG2000.0));

extern_static!(NSAlphaFirstBitmapFormat: NSBitmapFormat = NSBitmapFormat(NSBitmapFormat::AlphaFirst.0));

extern_static!(NSAlphaNonpremultipliedBitmapFormat: NSBitmapFormat = NSBitmapFormat(NSBitmapFormat::AlphaNonpremultiplied.0));

extern_static!(NSFloatingPointSamplesBitmapFormat: NSBitmapFormat = NSBitmapFormat(NSBitmapFormat::FloatingPointSamples.0));

extern_static!(NS16BitLittleEndianBitmapFormat: NSBitmapFormat = NSBitmapFormat(NSBitmapFormat::SixteenBitLittleEndian.0));

extern_static!(NS32BitLittleEndianBitmapFormat: NSBitmapFormat = NSBitmapFormat(NSBitmapFormat::ThirtyTwoBitLittleEndian.0));

extern_static!(NS16BitBigEndianBitmapFormat: NSBitmapFormat = NSBitmapFormat(NSBitmapFormat::SixteenBitBigEndian.0));

extern_static!(NS32BitBigEndianBitmapFormat: NSBitmapFormat = NSBitmapFormat(NSBitmapFormat::ThirtyTwoBitBigEndian.0));
