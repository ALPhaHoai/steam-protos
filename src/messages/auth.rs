//! Authentication protobuf messages for steam-session

use prost::Message;

#[allow(unused_imports)]
pub use crate::enums::EAuthTokenRevokeAction;
use crate::{
    enums::{EAuthSessionGuardType, EAuthSessionSecurityHistory, EAuthTokenAppType, EAuthTokenPlatformType, EAuthTokenState, EAuthenticationType, ESessionPersistence, ETokenRenewalType},
    messages::base::CMsgIPAddress,
};

/// Get password RSA public key request
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationGetPasswordRSAPublicKeyRequest {
    #[prost(string, optional, tag = "1")]
    pub account_name: Option<String>,
}

/// Get password RSA public key response
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationGetPasswordRSAPublicKeyResponse {
    #[prost(string, optional, tag = "1")]
    pub publickey_mod: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub publickey_exp: Option<String>,
    #[prost(uint64, optional, tag = "3")]
    pub timestamp: Option<u64>,
}

/// Device details for authentication
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationDeviceDetails {
    #[prost(string, optional, tag = "1")]
    pub device_friendly_name: Option<String>,
    #[prost(enumeration = "EAuthTokenPlatformType", optional, tag = "2")]
    pub platform_type: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub os_type: Option<i32>,
    #[prost(uint32, optional, tag = "4")]
    pub gaming_device_type: Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub client_count: Option<u32>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub machine_id: Option<Vec<u8>>,
    #[prost(enumeration = "EAuthTokenAppType", optional, tag = "7")]
    pub app_type: Option<i32>,
}

/// Begin auth session via QR request
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationBeginAuthSessionViaQRRequest {
    #[prost(string, optional, tag = "1")]
    pub device_friendly_name: Option<String>,
    #[prost(enumeration = "EAuthTokenPlatformType", optional, tag = "2")]
    pub platform_type: Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub device_details: Option<CAuthenticationDeviceDetails>,
    #[prost(string, optional, tag = "4")]
    pub website_id: Option<String>,
}

/// Begin auth session via QR response
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationBeginAuthSessionViaQRResponse {
    #[prost(uint64, optional, tag = "1")]
    pub client_id: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub challenge_url: Option<String>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub request_id: Option<Vec<u8>>,
    #[prost(float, optional, tag = "4")]
    pub interval: Option<f32>,
    #[prost(message, repeated, tag = "5")]
    pub allowed_confirmations: Vec<CAuthenticationAllowedConfirmation>,
    #[prost(int32, optional, tag = "6")]
    pub version: Option<i32>,
}

/// Allowed confirmation method
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationAllowedConfirmation {
    #[prost(enumeration = "EAuthSessionGuardType", optional, tag = "1")]
    pub confirmation_type: Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub associated_message: Option<String>,
}

/// Begin auth session via credentials request
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationBeginAuthSessionViaCredentialsRequest {
    #[prost(string, optional, tag = "1")]
    pub device_friendly_name: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub account_name: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub encrypted_password: Option<String>,
    #[prost(uint64, optional, tag = "4")]
    pub encryption_timestamp: Option<u64>,
    #[prost(bool, optional, tag = "5")]
    pub remember_login: Option<bool>,
    #[prost(enumeration = "EAuthTokenPlatformType", optional, tag = "6")]
    pub platform_type: Option<i32>,
    #[prost(enumeration = "ESessionPersistence", optional, tag = "7")]
    pub persistence: Option<i32>,
    #[prost(string, optional, tag = "8")]
    pub website_id: Option<String>,
    #[prost(message, optional, tag = "9")]
    pub device_details: Option<CAuthenticationDeviceDetails>,
    #[prost(string, optional, tag = "10")]
    pub guard_data: Option<String>,
    #[prost(uint32, optional, tag = "11")]
    pub language: Option<u32>,
    #[prost(int32, optional, tag = "12")]
    pub qos_level: Option<i32>,
}

/// Begin auth session via credentials response
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationBeginAuthSessionViaCredentialsResponse {
    #[prost(uint64, optional, tag = "1")]
    pub client_id: Option<u64>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub request_id: Option<Vec<u8>>,
    #[prost(float, optional, tag = "3")]
    pub interval: Option<f32>,
    #[prost(message, repeated, tag = "4")]
    pub allowed_confirmations: Vec<CAuthenticationAllowedConfirmation>,
    #[prost(uint64, optional, tag = "5")]
    pub steamid: Option<u64>,
    #[prost(string, optional, tag = "6")]
    pub weak_token: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub agreement_session_url: Option<String>,
    #[prost(string, optional, tag = "8")]
    pub extended_error_message: Option<String>,
}

/// Poll auth session status request
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationPollAuthSessionStatusRequest {
    #[prost(uint64, optional, tag = "1")]
    pub client_id: Option<u64>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub request_id: Option<Vec<u8>>,
    #[prost(fixed64, optional, tag = "3")]
    pub token_to_revoke: Option<u64>,
}

/// Poll auth session status response
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationPollAuthSessionStatusResponse {
    #[prost(uint64, optional, tag = "1")]
    pub new_client_id: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub new_challenge_url: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub refresh_token: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub access_token: Option<String>,
    #[prost(bool, optional, tag = "5")]
    pub had_remote_interaction: Option<bool>,
    #[prost(string, optional, tag = "6")]
    pub account_name: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub new_guard_data: Option<String>,
    #[prost(string, optional, tag = "8")]
    pub agreement_session_url: Option<String>,
}

/// Get auth session info request
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationGetAuthSessionInfoRequest {
    #[prost(uint64, optional, tag = "1")]
    pub client_id: Option<u64>,
}

/// Get auth session info response
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationGetAuthSessionInfoResponse {
    #[prost(string, optional, tag = "1")]
    pub ip: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub geoloc: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub city: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub state: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub country: Option<String>,
    #[prost(enumeration = "EAuthTokenPlatformType", optional, tag = "6")]
    pub platform_type: Option<i32>,
    #[prost(string, optional, tag = "7")]
    pub device_friendly_name: Option<String>,
    #[prost(int32, optional, tag = "8")]
    pub version: Option<i32>,
    #[prost(enumeration = "EAuthSessionSecurityHistory", optional, tag = "9")]
    pub login_history: Option<i32>,
    #[prost(bool, optional, tag = "10")]
    pub requestor_location_mismatch: Option<bool>,
    #[prost(bool, optional, tag = "11")]
    pub high_usage_login: Option<bool>,
    #[prost(enumeration = "ESessionPersistence", optional, tag = "12")]
    pub requested_persistence: Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub device_trust: Option<i32>,
    #[prost(enumeration = "EAuthTokenAppType", optional, tag = "14")]
    pub app_type: Option<i32>,
}

/// Update auth session with mobile confirmation request
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationUpdateAuthSessionWithMobileConfirmationRequest {
    #[prost(int32, optional, tag = "1")]
    pub version: Option<i32>,
    #[prost(uint64, optional, tag = "2")]
    pub client_id: Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub steamid: Option<u64>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub signature: Option<Vec<u8>>,
    #[prost(bool, optional, tag = "5")]
    pub confirm: Option<bool>,
    #[prost(enumeration = "ESessionPersistence", optional, tag = "6")]
    pub persistence: Option<i32>,
}

/// Update auth session with mobile confirmation response
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationUpdateAuthSessionWithMobileConfirmationResponse {}

/// Update auth session with Steam Guard code request
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationUpdateAuthSessionWithSteamGuardCodeRequest {
    #[prost(uint64, optional, tag = "1")]
    pub client_id: Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamid: Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub code: Option<String>,
    #[prost(enumeration = "EAuthSessionGuardType", optional, tag = "4")]
    pub code_type: Option<i32>,
}

/// Update auth session with Steam Guard code response
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationUpdateAuthSessionWithSteamGuardCodeResponse {
    #[prost(string, optional, tag = "7")]
    pub agreement_session_url: Option<String>,
}

/// Generate access token for app request
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationAccessTokenGenerateForAppRequest {
    #[prost(string, optional, tag = "1")]
    pub refresh_token: Option<String>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamid: Option<u64>,
    #[prost(enumeration = "ETokenRenewalType", optional, tag = "3")]
    pub renewal_type: Option<i32>,
}

/// Generate access token for app response
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationAccessTokenGenerateForAppResponse {
    #[prost(string, optional, tag = "1")]
    pub access_token: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub refresh_token: Option<String>,
}

/// Enumerate refresh tokens request
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CAuthenticationRefreshTokenEnumerateRequest {
    #[prost(bool, optional, tag = "1")]
    pub include_revoked: Option<bool>,
}

/// Event information for token usage
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct TokenUsageEvent {
    #[prost(uint32, optional, tag = "1")]
    pub time: Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub ip: Option<CMsgIPAddress>,
    #[prost(string, optional, tag = "3")]
    pub locale: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub country: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub state: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub city: Option<String>,
}

/// Refresh token description
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct RefreshTokenDescription {
    #[prost(fixed64, optional, tag = "1")]
    pub token_id: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub token_description: Option<String>,
    #[prost(uint32, optional, tag = "3")]
    pub time_updated: Option<u32>,
    #[prost(enumeration = "EAuthTokenPlatformType", optional, tag = "4")]
    pub platform_type: Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub logged_in: Option<bool>,
    #[prost(uint32, optional, tag = "6")]
    pub os_platform: Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub auth_type: Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub gaming_device_type: Option<u32>,
    #[prost(message, optional, tag = "9")]
    pub first_seen: Option<TokenUsageEvent>,
    #[prost(message, optional, tag = "10")]
    pub last_seen: Option<TokenUsageEvent>,
    #[prost(int32, optional, tag = "11")]
    pub os_type: Option<i32>,
    #[prost(enumeration = "EAuthenticationType", optional, tag = "12")]
    pub authentication_type: Option<i32>,
    #[prost(enumeration = "EAuthTokenState", optional, tag = "13")]
    pub effective_token_state: Option<i32>,
}

/// Enumerate refresh tokens response
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CAuthenticationRefreshTokenEnumerateResponse {
    #[prost(message, repeated, tag = "1")]
    pub refresh_tokens: Vec<RefreshTokenDescription>,
    #[prost(fixed64, optional, tag = "2")]
    pub requesting_token: Option<u64>,
}

/// Refresh token revoke request
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationRefreshTokenRevokeRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub token_id: Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamid: Option<u64>,
    #[prost(enumeration = "EAuthTokenRevokeAction", optional, tag = "3")]
    pub revoke_action: Option<i32>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub signature: Option<Vec<u8>>,
}

/// Refresh token revoke response
#[derive(Clone, PartialEq, Message)]
pub struct CAuthenticationRefreshTokenRevokeResponse {}
