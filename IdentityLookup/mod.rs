// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `IdentityLookup` framework

#[cfg_attr(feature = "apple", link(name = "IdentityLookup", kind = "framework"))]
extern "C" {}

#[path = "ILBase.rs"]
mod __ILBase;
#[path = "ILCallClassificationRequest.rs"]
mod __ILCallClassificationRequest;
#[path = "ILCallCommunication.rs"]
mod __ILCallCommunication;
#[path = "ILClassificationActions.rs"]
mod __ILClassificationActions;
#[path = "ILClassificationRequest.rs"]
mod __ILClassificationRequest;
#[path = "ILClassificationResponse.rs"]
mod __ILClassificationResponse;
#[path = "ILCommunication.rs"]
mod __ILCommunication;
#[path = "ILMessageClassificationRequest.rs"]
mod __ILMessageClassificationRequest;
#[path = "ILMessageCommunication.rs"]
mod __ILMessageCommunication;
#[path = "ILMessageFilterAction.rs"]
mod __ILMessageFilterAction;
#[path = "ILMessageFilterCapabilitiesQueryHandling.rs"]
mod __ILMessageFilterCapabilitiesQueryHandling;
#[path = "ILMessageFilterCapabilitiesQueryRequest.rs"]
mod __ILMessageFilterCapabilitiesQueryRequest;
#[path = "ILMessageFilterCapabilitiesQueryResponse.rs"]
mod __ILMessageFilterCapabilitiesQueryResponse;
#[path = "ILMessageFilterError.rs"]
mod __ILMessageFilterError;
#[path = "ILMessageFilterExtension.rs"]
mod __ILMessageFilterExtension;
#[path = "ILMessageFilterExtensionContext.rs"]
mod __ILMessageFilterExtensionContext;
#[path = "ILMessageFilterQueryHandling.rs"]
mod __ILMessageFilterQueryHandling;
#[path = "ILMessageFilterQueryRequest.rs"]
mod __ILMessageFilterQueryRequest;
#[path = "ILMessageFilterQueryResponse.rs"]
mod __ILMessageFilterQueryResponse;
#[path = "ILNetworkResponse.rs"]
mod __ILNetworkResponse;

#[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
pub use self::__ILCallClassificationRequest::ILCallClassificationRequest;
#[cfg(feature = "IdentityLookup_ILCallCommunication")]
pub use self::__ILCallCommunication::ILCallCommunication;
pub use self::__ILClassificationActions::{
    ILClassificationAction, ILClassificationActionNone, ILClassificationActionReportJunk,
    ILClassificationActionReportJunkAndBlockSender, ILClassificationActionReportNotJunk,
};
#[cfg(feature = "IdentityLookup_ILClassificationRequest")]
pub use self::__ILClassificationRequest::ILClassificationRequest;
#[cfg(feature = "IdentityLookup_ILClassificationResponse")]
pub use self::__ILClassificationResponse::ILClassificationResponse;
#[cfg(feature = "IdentityLookup_ILCommunication")]
pub use self::__ILCommunication::ILCommunication;
#[cfg(feature = "IdentityLookup_ILMessageClassificationRequest")]
pub use self::__ILMessageClassificationRequest::ILMessageClassificationRequest;
#[cfg(feature = "IdentityLookup_ILMessageCommunication")]
pub use self::__ILMessageCommunication::ILMessageCommunication;
pub use self::__ILMessageFilterAction::{
    ILMessageFilterAction, ILMessageFilterActionAllow, ILMessageFilterActionFilter,
    ILMessageFilterActionJunk, ILMessageFilterActionNone, ILMessageFilterActionPromotion,
    ILMessageFilterActionTransaction,
};
pub use self::__ILMessageFilterAction::{
    ILMessageFilterSubAction, ILMessageFilterSubActionNone,
    ILMessageFilterSubActionPromotionalCoupons, ILMessageFilterSubActionPromotionalOffers,
    ILMessageFilterSubActionPromotionalOthers, ILMessageFilterSubActionTransactionalCarrier,
    ILMessageFilterSubActionTransactionalFinance, ILMessageFilterSubActionTransactionalHealth,
    ILMessageFilterSubActionTransactionalOrders, ILMessageFilterSubActionTransactionalOthers,
    ILMessageFilterSubActionTransactionalPublicServices,
    ILMessageFilterSubActionTransactionalReminders, ILMessageFilterSubActionTransactionalRewards,
    ILMessageFilterSubActionTransactionalWeather,
};
pub use self::__ILMessageFilterCapabilitiesQueryHandling::ILMessageFilterCapabilitiesQueryHandling;
#[cfg(feature = "IdentityLookup_ILMessageFilterCapabilitiesQueryRequest")]
pub use self::__ILMessageFilterCapabilitiesQueryRequest::ILMessageFilterCapabilitiesQueryRequest;
#[cfg(feature = "IdentityLookup_ILMessageFilterCapabilitiesQueryResponse")]
pub use self::__ILMessageFilterCapabilitiesQueryResponse::ILMessageFilterCapabilitiesQueryResponse;
pub use self::__ILMessageFilterError::ILMessageFilterErrorDomain;
pub use self::__ILMessageFilterError::{
    ILMessageFilterError, ILMessageFilterErrorInvalidNetworkURL,
    ILMessageFilterErrorNetworkRequestFailed, ILMessageFilterErrorNetworkURLUnauthorized,
    ILMessageFilterErrorRedundantNetworkDeferral, ILMessageFilterErrorSystem,
};
#[cfg(feature = "IdentityLookup_ILMessageFilterExtension")]
pub use self::__ILMessageFilterExtension::ILMessageFilterExtension;
#[cfg(feature = "IdentityLookup_ILMessageFilterExtensionContext")]
pub use self::__ILMessageFilterExtensionContext::ILMessageFilterExtensionContext;
pub use self::__ILMessageFilterQueryHandling::ILMessageFilterQueryHandling;
#[cfg(feature = "IdentityLookup_ILMessageFilterQueryRequest")]
pub use self::__ILMessageFilterQueryRequest::ILMessageFilterQueryRequest;
#[cfg(feature = "IdentityLookup_ILMessageFilterQueryResponse")]
pub use self::__ILMessageFilterQueryResponse::ILMessageFilterQueryResponse;
#[cfg(feature = "IdentityLookup_ILNetworkResponse")]
pub use self::__ILNetworkResponse::ILNetworkResponse;
