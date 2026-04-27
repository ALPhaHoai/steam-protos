//! Login-related protobuf messages

use prost::Message;

use super::base::CMsgIPAddress;

/// Client heartbeat message
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientHeartBeat {
    #[prost(bool, optional, tag = "1")]
    pub send_reply: Option<bool>,
}

/// Client hello message
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientHello {
    #[prost(uint32, optional, tag = "1")]
    pub protocol_version: Option<u32>,
}

/// Client secret for embedded authentication
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientSecret {
    #[prost(uint32, optional, tag = "1")]
    pub version: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub appid: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub deviceid: Option<u32>,
    #[prost(fixed64, optional, tag = "4")]
    pub nonce: Option<u64>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub hmac: Option<Vec<u8>>,
}

/// Client logon request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientLogon {
    #[prost(uint32, optional, tag = "1")]
    pub protocol_version: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub deprecated_obfustucated_private_ip: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub cell_id: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub last_session_id: Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub client_package_version: Option<u32>,
    #[prost(string, optional, tag = "6")]
    pub client_language: Option<String>,
    #[prost(uint32, optional, tag = "7")]
    pub client_os_type: Option<u32>,
    #[prost(bool, optional, tag = "8")]
    pub should_remember_password: Option<bool>,
    #[prost(string, optional, tag = "9")]
    pub wine_version: Option<String>,
    #[prost(message, optional, tag = "11")]
    pub obfuscated_private_ip: Option<CMsgIPAddress>,
    #[prost(uint32, optional, tag = "21")]
    pub qos_level: Option<u32>,
    #[prost(fixed64, optional, tag = "22")]
    pub client_supplied_steam_id: Option<u64>,
    #[prost(message, optional, tag = "23")]
    pub public_ip: Option<CMsgIPAddress>,
    #[prost(bytes = "vec", optional, tag = "30")]
    pub machine_id: Option<Vec<u8>>,
    #[prost(uint32, optional, tag = "31")]
    pub launcher_type: Option<u32>,
    #[prost(uint32, optional, tag = "32")]
    pub ui_mode: Option<u32>,
    #[prost(uint32, optional, tag = "33")]
    pub chat_mode: Option<u32>,
    #[prost(bytes = "vec", optional, tag = "41")]
    pub steam2_auth_ticket: Option<Vec<u8>>,
    #[prost(string, optional, tag = "42")]
    pub email_address: Option<String>,
    #[prost(fixed32, optional, tag = "43")]
    pub rtime32_account_creation: Option<u32>,
    #[prost(string, optional, tag = "50")]
    pub account_name: Option<String>,
    #[prost(string, optional, tag = "51")]
    pub password: Option<String>,
    #[prost(string, optional, tag = "52")]
    pub game_server_token: Option<String>,
    #[prost(string, optional, tag = "60")]
    pub login_key: Option<String>,
    #[prost(uint32, optional, tag = "66")]
    pub ping_ms_from_cell_search: Option<u32>,
    #[prost(string, optional, tag = "80")]
    pub anon_user_target_account_name: Option<String>,
    #[prost(fixed64, optional, tag = "81")]
    pub resolved_user_steam_id: Option<u64>,
    #[prost(int32, optional, tag = "82")]
    pub eresult_sentryfile: Option<i32>,
    #[prost(bytes = "vec", optional, tag = "83")]
    pub sha_sentryfile: Option<Vec<u8>>,
    #[prost(string, optional, tag = "84")]
    pub auth_code: Option<String>,
    #[prost(int32, optional, tag = "85")]
    pub otp_type: Option<i32>,
    #[prost(uint32, optional, tag = "86")]
    pub otp_value: Option<u32>,
    #[prost(string, optional, tag = "87")]
    pub otp_identifier: Option<String>,
    #[prost(bool, optional, tag = "88")]
    pub steam2_ticket_request: Option<bool>,
    #[prost(bytes = "vec", optional, tag = "90")]
    pub sony_psn_ticket: Option<Vec<u8>>,
    #[prost(string, optional, tag = "91")]
    pub sony_psn_service_id: Option<String>,
    #[prost(bool, optional, tag = "92")]
    pub create_new_psn_linked_account_if_needed: Option<bool>,
    #[prost(string, optional, tag = "93")]
    pub sony_psn_name: Option<String>,
    #[prost(int32, optional, tag = "94")]
    pub game_server_app_id: Option<i32>,
    #[prost(bool, optional, tag = "95")]
    pub steamguard_dont_remember_computer: Option<bool>,
    #[prost(string, optional, tag = "96")]
    pub machine_name: Option<String>,
    #[prost(string, optional, tag = "97")]
    pub machine_name_userchosen: Option<String>,
    #[prost(string, optional, tag = "98")]
    pub country_override: Option<String>,
    #[prost(uint64, optional, tag = "100")]
    pub client_instance_id: Option<u64>,
    #[prost(string, optional, tag = "101")]
    pub two_factor_code: Option<String>,
    #[prost(bool, optional, tag = "102")]
    pub supports_rate_limit_response: Option<bool>,
    #[prost(string, optional, tag = "103")]
    pub web_logon_nonce: Option<String>,
    #[prost(int32, optional, tag = "104")]
    pub priority_reason: Option<i32>,
    #[prost(message, optional, tag = "105")]
    pub embedded_client_secret: Option<CMsgClientSecret>,
    #[prost(bool, optional, tag = "106")]
    pub disable_partner_autogrants: Option<bool>,
    #[prost(string, optional, tag = "108")]
    pub access_token: Option<String>,
    #[prost(bool, optional, tag = "109")]
    pub is_chrome_os: Option<bool>,
    #[prost(uint32, optional, tag = "111")]
    pub gaming_device_type: Option<u32>,
}

/// Client logon response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientLogonResponse {
    #[prost(int32, optional, tag = "1")]
    pub eresult: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub legacy_out_of_game_heartbeat_seconds: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub heartbeat_seconds: Option<i32>,
    #[prost(fixed32, optional, tag = "5")]
    pub rtime32_server_time: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub account_flags: Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub cell_id: Option<u32>,
    #[prost(string, optional, tag = "8")]
    pub email_domain: Option<String>,
    #[prost(bytes = "vec", optional, tag = "9")]
    pub steam2_ticket: Option<Vec<u8>>,
    #[prost(int32, optional, tag = "10")]
    pub eresult_extended: Option<i32>,
    #[prost(uint32, optional, tag = "12")]
    pub cell_id_ping_threshold: Option<u32>,
    #[prost(string, optional, tag = "14")]
    pub vanity_url: Option<String>,
    #[prost(message, optional, tag = "15")]
    pub public_ip: Option<CMsgIPAddress>,
    #[prost(string, optional, tag = "16")]
    pub user_country: Option<String>,
    #[prost(fixed64, optional, tag = "20")]
    pub client_supplied_steamid: Option<u64>,
    #[prost(string, optional, tag = "21")]
    pub ip_country_code: Option<String>,
    #[prost(bytes = "vec", optional, tag = "22")]
    pub parental_settings: Option<Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "23")]
    pub parental_setting_signature: Option<Vec<u8>>,
    #[prost(int32, optional, tag = "24")]
    pub count_loginfailures_to_migrate: Option<i32>,
    #[prost(int32, optional, tag = "25")]
    pub count_disconnects_to_migrate: Option<i32>,
    #[prost(int32, optional, tag = "26")]
    pub ogs_data_report_time_window: Option<i32>,
    #[prost(uint64, optional, tag = "27")]
    pub client_instance_id: Option<u64>,
    #[prost(bool, optional, tag = "28")]
    pub force_client_update_check: Option<bool>,
    #[prost(string, optional, tag = "29")]
    pub agreement_session_url: Option<String>,
    #[prost(uint64, optional, tag = "30")]
    pub token_id: Option<u64>,
    #[prost(uint64, optional, tag = "31")]
    pub family_group_id: Option<u64>,
}

/// Client log off request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientLogOff {}

/// Client logged off notification
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientLoggedOff {
    #[prost(int32, optional, tag = "1")]
    pub eresult: Option<i32>,
}

/// Client account info
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientAccountInfo {
    #[prost(string, optional, tag = "1")]
    pub persona_name: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub ip_country: Option<String>,
    #[prost(uint64, optional, tag = "3")]
    pub facebook_id: Option<u64>,
    #[prost(string, optional, tag = "4")]
    pub facebook_name: Option<String>,
    #[prost(int32, optional, tag = "5")]
    pub count_authed_computers: Option<i32>,
    #[prost(uint32, optional, tag = "7")]
    pub account_flags: Option<u32>,
    #[prost(string, optional, tag = "15")]
    pub steamguard_machine_name_user_chosen: Option<String>,
    #[prost(bool, optional, tag = "16")]
    pub is_phone_verified: Option<bool>,
    #[prost(uint32, optional, tag = "17")]
    pub two_factor_state: Option<u32>,
    #[prost(bool, optional, tag = "18")]
    pub is_phone_identifying: Option<bool>,
    #[prost(bool, optional, tag = "19")]
    pub is_phone_needing_reverify: Option<bool>,
}

/// New login key from server
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientNewLoginKey {
    #[prost(uint32, optional, tag = "1")]
    pub unique_id: Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub login_key: Option<String>,
}

/// Login key accepted acknowledgement
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientNewLoginKeyAccepted {
    #[prost(uint32, optional, tag = "1")]
    pub unique_id: Option<u32>,
}

/// Protocol version constant
pub const PROTOCOL_VERSION: u32 = 65580;

/// Private IP obfuscation mask
pub const PRIVATE_IP_OBFUSCATION_MASK: u32 = 0xBAADF00D;
