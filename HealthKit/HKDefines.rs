//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

#[cfg(feature = "Foundation_NSString")]
extern_static!(HKErrorDomain: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKErrorCode {
        HKUnknownError = 0,
        #[deprecated]
        HKNoError = HKErrorCode::HKUnknownError.0,
        HKErrorHealthDataUnavailable = 1,
        HKErrorHealthDataRestricted = 2,
        HKErrorInvalidArgument = 3,
        HKErrorAuthorizationDenied = 4,
        HKErrorAuthorizationNotDetermined = 5,
        HKErrorDatabaseInaccessible = 6,
        HKErrorUserCanceled = 7,
        HKErrorAnotherWorkoutSessionStarted = 8,
        HKErrorUserExitedWorkoutSession = 9,
        HKErrorRequiredAuthorizationDenied = 10,
        HKErrorNoData = 11,
        HKErrorWorkoutActivityNotAllowed = 12,
        HKErrorDataSizeExceeded = 13,
        HKErrorBackgroundWorkoutSessionNotAllowed = 14,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKUpdateFrequency {
        #[doc(alias = "HKUpdateFrequencyImmediate")]
        Immediate = 1,
        #[doc(alias = "HKUpdateFrequencyHourly")]
        Hourly = 2,
        #[doc(alias = "HKUpdateFrequencyDaily")]
        Daily = 3,
        #[doc(alias = "HKUpdateFrequencyWeekly")]
        Weekly = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKAuthorizationStatus {
        #[doc(alias = "HKAuthorizationStatusNotDetermined")]
        NotDetermined = 0,
        #[doc(alias = "HKAuthorizationStatusSharingDenied")]
        SharingDenied = 1,
        #[doc(alias = "HKAuthorizationStatusSharingAuthorized")]
        SharingAuthorized = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKAuthorizationRequestStatus {
        #[doc(alias = "HKAuthorizationRequestStatusUnknown")]
        Unknown = 0,
        #[doc(alias = "HKAuthorizationRequestStatusShouldRequest")]
        ShouldRequest = 1,
        #[doc(alias = "HKAuthorizationRequestStatusUnnecessary")]
        Unnecessary = 2,
    }
);

extern_fn!(
    #[cfg(all(feature = "Foundation_NSNumber", feature = "Foundation_NSSet"))]
    pub unsafe fn HKCategoryValueSleepAnalysisAsleepValues() -> NonNull<NSSet<NSNumber>>;
);
