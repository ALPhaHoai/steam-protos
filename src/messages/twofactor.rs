//! Two-factor authentication protobuf messages

use prost::Message;

/// Add authenticator request
///
/// From `steammessages_twofactor.steamclient.proto`
#[derive(Clone, PartialEq, Message)]
pub struct CTwoFactorAddAuthenticatorRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub authenticator_time: Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub serial_number: Option<u64>,
    #[prost(uint32, optional, tag = "4")]
    pub authenticator_type: Option<u32>,
    #[prost(string, optional, tag = "5")]
    pub device_identifier: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub sms_phone_id: Option<String>,
    #[prost(string, repeated, tag = "7")]
    pub http_headers: Vec<String>,
    #[prost(uint32, optional, tag = "8")]
    pub version: Option<u32>,
}

/// Add authenticator response
#[derive(Clone, PartialEq, Message)]
pub struct CTwoFactorAddAuthenticatorResponse {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub shared_secret: Option<Vec<u8>>,
    #[prost(fixed64, optional, tag = "2")]
    pub serial_number: Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub revocation_code: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub uri: Option<String>,
    #[prost(uint64, optional, tag = "5")]
    pub server_time: Option<u64>,
    #[prost(string, optional, tag = "6")]
    pub account_name: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub token_gid: Option<String>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub identity_secret: Option<Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "9")]
    pub secret_1: Option<Vec<u8>>,
    #[prost(int32, optional, tag = "10")]
    pub status: Option<i32>,
    #[prost(string, optional, tag = "11")]
    pub phone_number_hint: Option<String>,
    #[prost(int32, optional, tag = "12")]
    pub confirm_type: Option<i32>,
}

/// Finalize add authenticator request
#[derive(Clone, PartialEq, Message)]
pub struct CTwoFactorFinalizeAddAuthenticatorRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub authenticator_code: Option<String>,
    #[prost(uint64, optional, tag = "3")]
    pub authenticator_time: Option<u64>,
    #[prost(string, optional, tag = "4")]
    pub activation_code: Option<String>,
    #[prost(string, repeated, tag = "5")]
    pub http_headers: Vec<String>,
    #[prost(bool, optional, tag = "6")]
    pub validate_sms_code: Option<bool>,
}

/// Finalize add authenticator response
#[derive(Clone, PartialEq, Message)]
pub struct CTwoFactorFinalizeAddAuthenticatorResponse {
    #[prost(bool, optional, tag = "1")]
    pub success: Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub want_more: Option<bool>,
    #[prost(uint64, optional, tag = "3")]
    pub server_time: Option<u64>,
    #[prost(int32, optional, tag = "4")]
    pub status: Option<i32>,
}
