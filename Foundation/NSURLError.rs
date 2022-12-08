//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSURLErrorDomain: &'static NSErrorDomain);

extern_static!(NSURLErrorFailingURLErrorKey: &'static NSString);

extern_static!(NSURLErrorFailingURLStringErrorKey: &'static NSString);

extern_static!(NSErrorFailingURLStringKey: &'static NSString);

extern_static!(NSURLErrorFailingURLPeerTrustErrorKey: &'static NSString);

extern_static!(NSURLErrorBackgroundTaskCancelledReasonKey: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum {
        NSURLErrorCancelledReasonUserForceQuitApplication = 0,
        NSURLErrorCancelledReasonBackgroundUpdatesDisabled = 1,
        NSURLErrorCancelledReasonInsufficientSystemResources = 2,
    }
);

extern_static!(NSURLErrorNetworkUnavailableReasonKey: &'static NSErrorUserInfoKey);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSURLErrorNetworkUnavailableReason {
        NSURLErrorNetworkUnavailableReasonCellular = 0,
        NSURLErrorNetworkUnavailableReasonExpensive = 1,
        NSURLErrorNetworkUnavailableReasonConstrained = 2,
    }
);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum {
        NSURLErrorUnknown = -1,
        NSURLErrorCancelled = -999,
        NSURLErrorBadURL = -1000,
        NSURLErrorTimedOut = -1001,
        NSURLErrorUnsupportedURL = -1002,
        NSURLErrorCannotFindHost = -1003,
        NSURLErrorCannotConnectToHost = -1004,
        NSURLErrorNetworkConnectionLost = -1005,
        NSURLErrorDNSLookupFailed = -1006,
        NSURLErrorHTTPTooManyRedirects = -1007,
        NSURLErrorResourceUnavailable = -1008,
        NSURLErrorNotConnectedToInternet = -1009,
        NSURLErrorRedirectToNonExistentLocation = -1010,
        NSURLErrorBadServerResponse = -1011,
        NSURLErrorUserCancelledAuthentication = -1012,
        NSURLErrorUserAuthenticationRequired = -1013,
        NSURLErrorZeroByteResource = -1014,
        NSURLErrorCannotDecodeRawData = -1015,
        NSURLErrorCannotDecodeContentData = -1016,
        NSURLErrorCannotParseResponse = -1017,
        NSURLErrorAppTransportSecurityRequiresSecureConnection = -1022,
        NSURLErrorFileDoesNotExist = -1100,
        NSURLErrorFileIsDirectory = -1101,
        NSURLErrorNoPermissionsToReadFile = -1102,
        NSURLErrorDataLengthExceedsMaximum = -1103,
        NSURLErrorFileOutsideSafeArea = -1104,
        NSURLErrorSecureConnectionFailed = -1200,
        NSURLErrorServerCertificateHasBadDate = -1201,
        NSURLErrorServerCertificateUntrusted = -1202,
        NSURLErrorServerCertificateHasUnknownRoot = -1203,
        NSURLErrorServerCertificateNotYetValid = -1204,
        NSURLErrorClientCertificateRejected = -1205,
        NSURLErrorClientCertificateRequired = -1206,
        NSURLErrorCannotLoadFromNetwork = -2000,
        NSURLErrorCannotCreateFile = -3000,
        NSURLErrorCannotOpenFile = -3001,
        NSURLErrorCannotCloseFile = -3002,
        NSURLErrorCannotWriteToFile = -3003,
        NSURLErrorCannotRemoveFile = -3004,
        NSURLErrorCannotMoveFile = -3005,
        NSURLErrorDownloadDecodingFailedMidStream = -3006,
        NSURLErrorDownloadDecodingFailedToComplete = -3007,
        NSURLErrorInternationalRoamingOff = -1018,
        NSURLErrorCallIsActive = -1019,
        NSURLErrorDataNotAllowed = -1020,
        NSURLErrorRequestBodyStreamExhausted = -1021,
        NSURLErrorBackgroundSessionRequiresSharedContainer = -995,
        NSURLErrorBackgroundSessionInUseByAnotherProcess = -996,
        NSURLErrorBackgroundSessionWasDisconnected = -997,
    }
);