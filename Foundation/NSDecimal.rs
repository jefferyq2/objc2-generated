//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRoundingMode(pub NSUInteger);
impl NSRoundingMode {
    pub const NSRoundPlain: Self = Self(0);
    pub const NSRoundDown: Self = Self(1);
    pub const NSRoundUp: Self = Self(2);
    pub const NSRoundBankers: Self = Self(3);
}

unsafe impl Encode for NSRoundingMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSRoundingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCalculationError(pub NSUInteger);
impl NSCalculationError {
    pub const NSCalculationNoError: Self = Self(0);
    pub const NSCalculationLossOfPrecision: Self = Self(1);
    pub const NSCalculationUnderflow: Self = Self(2);
    pub const NSCalculationOverflow: Self = Self(3);
    pub const NSCalculationDivideByZero: Self = Self(4);
}

unsafe impl Encode for NSCalculationError {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCalculationError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn NSDecimalIsNotANumber(dcm: NonNull<NSDecimal>,) -> Bool;

extern "C" {
    pub fn NSDecimalCopy(destination: NonNull<NSDecimal>, source: NonNull<NSDecimal>);
}

extern "C" {
    pub fn NSDecimalCompact(number: NonNull<NSDecimal>);
}

extern "C" {
    #[cfg(feature = "NSObjCRuntime")]
    pub fn NSDecimalCompare(
        left_operand: NonNull<NSDecimal>,
        right_operand: NonNull<NSDecimal>,
    ) -> NSComparisonResult;
}

extern "C" {
    pub fn NSDecimalRound(
        result: NonNull<NSDecimal>,
        number: NonNull<NSDecimal>,
        scale: NSInteger,
        rounding_mode: NSRoundingMode,
    );
}

extern "C" {
    pub fn NSDecimalNormalize(
        number1: NonNull<NSDecimal>,
        number2: NonNull<NSDecimal>,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C" {
    pub fn NSDecimalAdd(
        result: NonNull<NSDecimal>,
        left_operand: NonNull<NSDecimal>,
        right_operand: NonNull<NSDecimal>,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C" {
    pub fn NSDecimalSubtract(
        result: NonNull<NSDecimal>,
        left_operand: NonNull<NSDecimal>,
        right_operand: NonNull<NSDecimal>,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C" {
    pub fn NSDecimalMultiply(
        result: NonNull<NSDecimal>,
        left_operand: NonNull<NSDecimal>,
        right_operand: NonNull<NSDecimal>,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C" {
    pub fn NSDecimalDivide(
        result: NonNull<NSDecimal>,
        left_operand: NonNull<NSDecimal>,
        right_operand: NonNull<NSDecimal>,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C" {
    pub fn NSDecimalPower(
        result: NonNull<NSDecimal>,
        number: NonNull<NSDecimal>,
        power: NSUInteger,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C" {
    pub fn NSDecimalMultiplyByPowerOf10(
        result: NonNull<NSDecimal>,
        number: NonNull<NSDecimal>,
        power: c_short,
        rounding_mode: NSRoundingMode,
    ) -> NSCalculationError;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub fn NSDecimalString(
        dcm: NonNull<NSDecimal>,
        locale: Option<&AnyObject>,
    ) -> NonNull<NSString>;
}
