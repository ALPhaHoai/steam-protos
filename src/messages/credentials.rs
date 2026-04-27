//! Credentials protobuf messages

use prost::Message;

use crate::messages::base::CMsgIPAddress;

/// Get Steam Guard details request
#[derive(Clone, PartialEq, Message)]
pub struct CCredentialsGetSteamGuardDetailsRequest {
    #[prost(string, optional, tag = "2")]
    pub webcookie: Option<String>,
    #[prost(fixed32, optional, tag = "3")]
    pub timestamp_minimum_wanted: Option<u32>,
    #[prost(int32, optional, tag = "4")]
    pub deprecated_ipaddress: Option<i32>,
    #[prost(message, optional, tag = "5")]
    pub ip_address: Option<CMsgIPAddress>,
}

/// Get Steam Guard details response
#[derive(Clone, PartialEq, Message)]
pub struct CCredentialsGetSteamGuardDetailsResponse {
    #[prost(bool, optional, tag = "1")]
    pub is_steamguard_enabled: Option<bool>,
    #[prost(fixed32, optional, tag = "2")]
    pub timestamp_steamguard_enabled: Option<u32>,
    #[prost(string, optional, tag = "4")]
    pub deprecated_machine_name_userchosen: Option<String>,
    #[prost(fixed32, optional, tag = "5")]
    pub deprecated_timestamp_machine_steamguard_enabled: Option<u32>,
    #[prost(bool, optional, tag = "6")]
    pub deprecated_authentication_exists_from_geoloc_before_mintime: Option<bool>,
    #[prost(uint64, optional, tag = "7")]
    pub deprecated_machine_id: Option<u64>,
    #[prost(message, repeated, tag = "8")]
    pub session_data: Vec<CredentialsSessionData>,
    #[prost(bool, optional, tag = "9")]
    pub is_twofactor_enabled: Option<bool>,
    #[prost(fixed32, optional, tag = "10")]
    pub timestamp_twofactor_enabled: Option<u32>,
    #[prost(bool, optional, tag = "11")]
    pub is_phone_verified: Option<bool>,
}

/// Session data for Steam Guard details
#[derive(Clone, PartialEq, Message)]
pub struct CredentialsSessionData {
    #[prost(uint64, optional, tag = "1")]
    pub machine_id: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub machine_name_userchosen: Option<String>,
    #[prost(fixed32, optional, tag = "3")]
    pub timestamp_machine_steamguard_enabled: Option<u32>,
    #[prost(bool, optional, tag = "4")]
    pub authentication_exists_from_geoloc_before_mintime: Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub authentication_exists_from_same_ip_before_mintime: Option<bool>,
    #[prost(uint32, optional, tag = "7")]
    pub public_ipv4: Option<u32>,
    #[prost(string, optional, tag = "8")]
    pub public_ip_address: Option<String>,
}

/// Get last credential change time request
#[derive(Clone, PartialEq, Message)]
pub struct CCredentialsLastCredentialChangeTimeRequest {
    #[prost(bool, optional, tag = "1")]
    pub user_changes_only: Option<bool>,
}

/// Get last credential change time response
#[derive(Clone, PartialEq, Message)]
pub struct CCredentialsLastCredentialChangeTimeResponse {
    #[prost(fixed32, optional, tag = "1")]
    pub timestamp_last_password_change: Option<u32>,
    #[prost(fixed32, optional, tag = "2")]
    pub timestamp_last_email_change: Option<u32>,
    #[prost(fixed32, optional, tag = "3")]
    pub timestamp_last_password_reset: Option<u32>,
}

/// Get account auth secret request
#[derive(Clone, PartialEq, Message)]
pub struct CCredentialsGetAccountAuthSecretRequest {}

/// Get account auth secret response
#[derive(Clone, PartialEq, Message)]
pub struct CCredentialsGetAccountAuthSecretResponse {
    #[prost(int32, optional, tag = "1")]
    pub secret_id: Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub secret: Option<Vec<u8>>,
}
