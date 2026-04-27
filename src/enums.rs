//! Steam protobuf enums used in authentication and player services
//!
//! These enums follow the Steam protobuf naming convention with K prefix.

/// Platform type for authentication tokens
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum EAuthTokenPlatformType {
    #[prost(name = "k_EAuthTokenPlatformType_Unknown")]
    KEAuthTokenPlatformTypeUnknown = 0,
    #[prost(name = "k_EAuthTokenPlatformType_SteamClient")]
    KEAuthTokenPlatformTypeSteamClient = 1,
    #[prost(name = "k_EAuthTokenPlatformType_WebBrowser")]
    KEAuthTokenPlatformTypeWebBrowser = 2,
    #[prost(name = "k_EAuthTokenPlatformType_MobileApp")]
    KEAuthTokenPlatformTypeMobileApp = 3,
}

/// Guard type for authentication sessions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum EAuthSessionGuardType {
    #[prost(name = "k_EAuthSessionGuardType_Unknown")]
    KEAuthSessionGuardTypeUnknown = 0,
    #[prost(name = "k_EAuthSessionGuardType_None")]
    KEAuthSessionGuardTypeNone = 1,
    #[prost(name = "k_EAuthSessionGuardType_EmailCode")]
    KEAuthSessionGuardTypeEmailCode = 2,
    #[prost(name = "k_EAuthSessionGuardType_DeviceCode")]
    KEAuthSessionGuardTypeDeviceCode = 3,
    #[prost(name = "k_EAuthSessionGuardType_DeviceConfirmation")]
    KEAuthSessionGuardTypeDeviceConfirmation = 4,
    #[prost(name = "k_EAuthSessionGuardType_EmailConfirmation")]
    KEAuthSessionGuardTypeEmailConfirmation = 5,
    #[prost(name = "k_EAuthSessionGuardType_MachineToken")]
    KEAuthSessionGuardTypeMachineToken = 6,
    #[prost(name = "k_EAuthSessionGuardType_LegacyMachineAuth")]
    KEAuthSessionGuardTypeLegacyMachineAuth = 7,
}

/// Session persistence type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum ESessionPersistence {
    #[prost(name = "k_ESessionPersistence_Invalid")]
    KESessionPersistenceInvalid = -1,
    #[prost(name = "k_ESessionPersistence_Ephemeral")]
    KESessionPersistenceEphemeral = 0,
    #[prost(name = "k_ESessionPersistence_Persistent")]
    KESessionPersistencePersistent = 1,
}

/// Token renewal type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum ETokenRenewalType {
    #[prost(name = "k_ETokenRenewalType_None")]
    KETokenRenewalTypeNone = 0,
    #[prost(name = "k_ETokenRenewalType_Allow")]
    KETokenRenewalTypeAllow = 1,
}

/// Security history for authentication sessions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum EAuthSessionSecurityHistory {
    #[prost(name = "k_EAuthSessionSecurityHistory_Invalid")]
    KEAuthSessionSecurityHistoryInvalid = 0,
    #[prost(name = "k_EAuthSessionSecurityHistory_UsedPreviously")]
    KEAuthSessionSecurityHistoryUsedPreviously = 1,
    #[prost(name = "k_EAuthSessionSecurityHistory_NoPriorHistory")]
    KEAuthSessionSecurityHistoryNoPriorHistory = 2,
}

/// Notification setting preference
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum ENotificationSetting {
    KENotificationSettingNotifyUseDefault = 0,
    KENotificationSettingAlways = 1,
    KENotificationSettingNever = 2,
}

/// Steam notification types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum ESteamNotificationType {
    Invalid = 0,
    Test = 1,
    Gift = 2,
    Comment = 3,
    Item = 4,
    FriendInvite = 5,
    MajorSale = 6,
    PreloadAvailable = 7,
    Wishlist = 8,
    TradeOffer = 9,
    General = 10,
    HelpRequest = 11,
    AsyncGame = 12,
    ChatMsg = 13,
    ModeratorMsg = 14,
    ParentalFeatureAccessRequest = 15,
    FamilyInvite = 16,
    FamilyPurchaseRequest = 17,
    ParentalPlaytimeRequest = 18,
    FamilyPurchaseRequestResponse = 19,
    ParentalFeatureAccessResponse = 20,
    ParentalPlaytimeResponse = 21,
    RequestedGameAdded = 22,
    SendToPhone = 23,
    ClipDownloaded = 24,
    TwoFaPrompt = 25,
    MobileConfirmation = 26,
    PartnerEvent = 27,
    PlaytestInvite = 28,
    TradeReversal = 29,
}

/// App type for authentication tokens
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum EAuthTokenAppType {
    KEAuthTokenAppTypeUnknown = 0,
    KEAuthTokenAppTypeMobileSteamApp = 1,
    KEAuthTokenAppTypeMobileChatApp = 2,
}

/// Authentication type for sessions
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum EAuthenticationType {
    #[prost(name = "k_EAuthenticationType_Unknown")]
    KEAuthenticationTypeUnknown = 0,
    #[prost(name = "k_EAuthenticationType_Password")]
    KEAuthenticationTypePassword = 1,
    #[prost(name = "k_EAuthenticationType_QR")]
    KEAuthenticationTypeQR = 2,
    #[prost(name = "k_EAuthenticationType_AccountCreation")]
    KEAuthenticationTypeAccountCreation = 3,
    #[prost(name = "k_EAuthenticationType_GuestAccount")]
    KEAuthenticationTypeGuestAccount = 4,
}

/// State of an authentication token
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum EAuthTokenState {
    #[prost(name = "k_EAuthTokenState_Invalid")]
    KEAuthTokenStateInvalid = 0,
    #[prost(name = "k_EAuthTokenState_New")]
    KEAuthTokenStateNew = 1,
    #[prost(name = "k_EAuthTokenState_Confirmed")]
    KEAuthTokenStateConfirmed = 2,
    #[prost(name = "k_EAuthTokenState_Issued")]
    KEAuthTokenStateIssued = 3,
    #[prost(name = "k_EAuthTokenState_Denied")]
    KEAuthTokenStateDenied = 4,
    #[prost(name = "k_EAuthTokenState_LoggedOut")]
    KEAuthTokenStateLoggedOut = 5,
    #[prost(name = "k_EAuthTokenState_Consumed")]
    KEAuthTokenStateConsumed = 6,
    #[prost(name = "k_EAuthTokenState_Revoked")]
    KEAuthTokenStateRevoked = 99,
}

/// Action to take when revoking an authentication token
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i32)]
pub enum EAuthTokenRevokeAction {
    #[prost(name = "k_EAuthTokenRevokeLogout")]
    Logout = 0,
    #[prost(name = "k_EAuthTokenRevokePermanent")]
    Permanent = 1,
    #[prost(name = "k_EAuthTokenRevokeReplaced")]
    Replaced = 2,
    #[prost(name = "k_EAuthTokenRevokeSupport")]
    Support = 3,
    #[prost(name = "k_EAuthTokenRevokeConsume")]
    Consume = 4,
    #[prost(name = "k_EAuthTokenRevokeNonRememberedLogout")]
    NonRememberedLogout = 5,
    #[prost(name = "k_EAuthTokenRevokeNonRememberedPermanent")]
    NonRememberedPermanent = 6,
    #[prost(name = "k_EAuthTokenRevokeAutomatic")]
    Automatic = 7,
}
