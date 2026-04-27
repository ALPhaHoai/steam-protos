//! Client-server miscellaneous protobuf messages

use prost::Message;

use super::base::{CMsgAuthTicket, CMsgIPAddress};

/// License list
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientLicenseList {
    #[prost(int32, optional, tag = "1")]
    pub eresult: Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub licenses: Vec<cmsg_client_license_list::License>,
}

pub mod cmsg_client_license_list {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct License {
        #[prost(uint32, optional, tag = "1")]
        pub package_id: Option<u32>,
        #[prost(fixed32, optional, tag = "2")]
        pub time_created: Option<u32>,
        #[prost(fixed32, optional, tag = "3")]
        pub time_next_process: Option<u32>,
        #[prost(int32, optional, tag = "4")]
        pub minute_limit: Option<i32>,
        #[prost(int32, optional, tag = "5")]
        pub minutes_used: Option<i32>,
        #[prost(uint32, optional, tag = "6")]
        pub payment_method: Option<u32>,
        #[prost(uint32, optional, tag = "7")]
        pub flags: Option<u32>,
        #[prost(string, optional, tag = "8")]
        pub purchase_country_code: Option<String>,
        #[prost(uint32, optional, tag = "9")]
        pub license_type: Option<u32>,
        #[prost(int32, optional, tag = "10")]
        pub territory_code: Option<i32>,
        #[prost(int32, optional, tag = "11")]
        pub change_number: Option<i32>,
        #[prost(uint32, optional, tag = "12")]
        pub owner_id: Option<u32>,
        #[prost(uint32, optional, tag = "13")]
        pub initial_period: Option<u32>,
        #[prost(uint32, optional, tag = "14")]
        pub initial_time_unit: Option<u32>,
        #[prost(uint32, optional, tag = "15")]
        pub renewal_period: Option<u32>,
        #[prost(uint32, optional, tag = "16")]
        pub renewal_time_unit: Option<u32>,
        #[prost(uint64, optional, tag = "17")]
        pub access_token: Option<u64>,
        #[prost(uint32, optional, tag = "18")]
        pub master_package_id: Option<u32>,
    }
}

/// Games played message
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientGamesPlayed {
    #[prost(message, repeated, tag = "1")]
    pub games_played: Vec<cmsg_client_games_played::GamePlayed>,
    #[prost(uint32, optional, tag = "2")]
    pub client_os_type: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub cloud_gaming_platform: Option<u32>,
    #[prost(bool, optional, tag = "4")]
    pub recent_reauthentication: Option<bool>,
}

pub mod cmsg_client_games_played {
    use prost::Message;

    use super::CMsgIPAddress;

    #[derive(Clone, PartialEq, Message)]
    pub struct GamePlayed {
        #[prost(uint64, optional, tag = "1")]
        pub steam_id_gs: Option<u64>,
        #[prost(fixed64, optional, tag = "2")]
        pub game_id: Option<u64>,
        #[prost(uint32, optional, tag = "3")]
        pub deprecated_game_ip_address: Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub game_port: Option<u32>,
        #[prost(bool, optional, tag = "5")]
        pub is_secure: Option<bool>,
        #[prost(bytes = "vec", optional, tag = "6")]
        pub token: Option<Vec<u8>>,
        #[prost(string, optional, tag = "7")]
        pub game_extra_info: Option<String>,
        #[prost(bytes = "vec", optional, tag = "8")]
        pub game_data_blob: Option<Vec<u8>>,
        #[prost(uint32, optional, tag = "9")]
        pub process_id: Option<u32>,
        #[prost(uint32, optional, tag = "10")]
        pub streaming_provider_id: Option<u32>,
        #[prost(uint32, optional, tag = "11")]
        pub game_flags: Option<u32>,
        #[prost(uint32, optional, tag = "12")]
        pub owner_id: Option<u32>,
        #[prost(string, optional, tag = "13")]
        pub vr_hmd_vendor: Option<String>,
        #[prost(string, optional, tag = "14")]
        pub vr_hmd_model: Option<String>,
        #[prost(uint32, optional, tag = "15")]
        pub launch_option_type: Option<u32>,
        #[prost(int32, optional, tag = "16")]
        pub primary_controller_type: Option<i32>,
        #[prost(string, optional, tag = "17")]
        pub primary_steam_controller_serial: Option<String>,
        #[prost(uint32, optional, tag = "18")]
        pub total_steam_controller_count: Option<u32>,
        #[prost(uint32, optional, tag = "19")]
        pub total_non_steam_controller_count: Option<u32>,
        #[prost(uint64, optional, tag = "20")]
        pub controller_workshop_file_id: Option<u64>,
        #[prost(uint32, optional, tag = "21")]
        pub launch_source: Option<u32>,
        #[prost(uint32, optional, tag = "22")]
        pub vr_hmd_runtime: Option<u32>,
        #[prost(message, optional, tag = "23")]
        pub game_ip_address: Option<CMsgIPAddress>,
        #[prost(uint32, optional, tag = "24")]
        pub controller_connection_type: Option<u32>,
        #[prost(int32, optional, tag = "25")]
        pub game_os_platform: Option<i32>,
        #[prost(uint32, optional, tag = "26")]
        pub game_build_id: Option<u32>,
        #[prost(uint32, optional, tag = "27")]
        pub compat_tool_id: Option<u32>,
        #[prost(string, optional, tag = "28")]
        pub compat_tool_cmd: Option<String>,
        #[prost(uint32, optional, tag = "29")]
        pub compat_tool_build_id: Option<u32>,
        #[prost(string, optional, tag = "30")]
        pub beta_name: Option<String>,
        #[prost(uint32, optional, tag = "31")]
        pub dlc_context: Option<u32>,
    }
}

/// Service method legacy request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientServiceMethodLegacy {
    #[prost(string, optional, tag = "1")]
    pub method_name: Option<String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub serialized_method: Option<Vec<u8>>,
    #[prost(bool, optional, tag = "3")]
    pub is_notification: Option<bool>,
}

/// Service method legacy response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientServiceMethodLegacyResponse {
    #[prost(string, optional, tag = "1")]
    pub method_name: Option<String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub serialized_method_response: Option<Vec<u8>>,
}

/// CM list message
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientCMList {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub cm_addresses: Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub cm_ports: Vec<u32>,
    #[prost(string, repeated, tag = "3")]
    pub cm_websocket_addresses: Vec<String>,
    #[prost(uint32, optional, tag = "4")]
    pub percent_default_to_websocket: Option<u32>,
}

/// Auth list message
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientAuthList {
    #[prost(uint32, optional, tag = "1")]
    pub tokens_left: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub last_request_seq: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub last_request_seq_from_server: Option<u32>,
    #[prost(message, repeated, tag = "4")]
    pub tickets: Vec<CMsgAuthTicket>,
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub app_ids: Vec<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub message_sequence: Option<u32>,
    #[prost(bool, optional, tag = "7")]
    pub filtered: Option<bool>,
}

/// Auth list ack
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientAuthListAck {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub ticket_crc: Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub app_ids: Vec<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub message_sequence: Option<u32>,
}

/// Ticket auth complete
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientTicketAuthComplete {
    #[prost(fixed64, optional, tag = "1")]
    pub steam_id: Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub game_id: Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub estate: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub eauth_session_response: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub ticket_crc: Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub ticket_sequence: Option<u32>,
    #[prost(fixed64, optional, tag = "8")]
    pub owner_steam_id: Option<u64>,
}

/// Device auth add authorized borrowers request
#[derive(Clone, PartialEq, Message)]
pub struct CDeviceAuthAddAuthorizedBorrowersRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(fixed64, repeated, packed = "false", tag = "2")]
    pub steamid_borrower: Vec<u64>,
}

/// Device auth remove authorized borrowers request
#[derive(Clone, PartialEq, Message)]
pub struct CDeviceAuthRemoveAuthorizedBorrowersRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(fixed64, repeated, packed = "false", tag = "2")]
    pub steamid_borrower: Vec<u64>,
}

/// Device auth get authorized borrowers request
#[derive(Clone, PartialEq, Message)]
pub struct CDeviceAuthGetAuthorizedBorrowersRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(bool, optional, tag = "2")]
    pub include_canceled: Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub include_pending: Option<bool>,
}

/// Device auth get own authorized devices request
#[derive(Clone, PartialEq, Message)]
pub struct CDeviceAuthGetOwnAuthorizedDevicesRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(bool, optional, tag = "2")]
    pub include_canceled: Option<bool>,
}

/// Device auth authorize local device request
#[derive(Clone, PartialEq, Message)]
pub struct CDeviceAuthAuthorizeLocalDeviceRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub deviceid: Option<u64>,
}

/// Device auth deauthorize device request
#[derive(Clone, PartialEq, Message)]
pub struct CDeviceAuthDeauthorizeDeviceRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub deviceid: Option<u64>,
}

/// Request validation email
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRequestValidationEmail {}

/// Email address info
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientEmailAddrInfo {
    #[prost(string, optional, tag = "1")]
    pub email_address: Option<String>,
    #[prost(bool, optional, tag = "2")]
    pub email_is_validated: Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub email_validation_changed: Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub credential_change_requires_code: Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub password_or_secretqa_change_requires_code: Option<bool>,
}

/// Account limitation status
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientIsLimitedAccount {
    #[prost(bool, optional, tag = "1")]
    pub bis_limited_account: Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub bis_community_banned: Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub bis_locked_account: Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub bis_limited_account_allowed_to_invite_friends: Option<bool>,
}

/// Wallet info update
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientWalletInfoUpdate {
    #[prost(bool, optional, tag = "1")]
    pub has_wallet: Option<bool>,
    #[prost(int32, optional, tag = "2")]
    pub balance: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub currency: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub balance_delayed: Option<i32>,
    #[prost(int64, optional, tag = "5")]
    pub balance64: Option<i64>,
    #[prost(int64, optional, tag = "6")]
    pub balance64_delayed: Option<i64>,
    #[prost(int32, optional, tag = "7")]
    pub realm: Option<i32>,
}

/// Playing session state - indicates if game playing is blocked by another
/// session
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientPlayingSessionState {
    /// True if playing is blocked because this account is playing a game
    /// elsewhere
    #[prost(bool, optional, tag = "2")]
    pub playing_blocked: Option<bool>,
    /// The app ID currently being played (elsewhere if blocked, or by us if not
    /// blocked)
    #[prost(uint32, optional, tag = "3")]
    pub playing_app: Option<u32>,
}

/// Game connect tokens
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientGameConnectTokens {
    #[prost(uint32, optional, tag = "1", default = "10")]
    pub max_tokens_to_keep: Option<u32>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub tokens: Vec<Vec<u8>>,
}

/// Register product key
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRegisterKey {
    #[prost(string, optional, tag = "1")]
    pub key: Option<String>,
}

/// Request free license
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRequestFreeLicense {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub app_ids: Vec<u32>,
}

/// Get legacy game key
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientGetLegacyGameKey {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
}

// ============================================================================
// New structs added for Family Sharing
// ============================================================================

/// Device auth get authorized borrowers response
#[derive(Clone, PartialEq, Message)]
pub struct CDeviceAuthGetAuthorizedBorrowersResponse {
    #[prost(message, repeated, tag = "1")]
    pub borrowers: Vec<c_device_auth_get_authorized_borrowers_response::Borrower>,
}

pub mod c_device_auth_get_authorized_borrowers_response {
    use prost::Message;
    #[derive(Clone, PartialEq, Message)]
    pub struct Borrower {
        #[prost(fixed64, optional, tag = "1")]
        pub steamid: Option<u64>,
        #[prost(bool, optional, tag = "2")]
        pub is_pending: Option<bool>,
        #[prost(bool, optional, tag = "3")]
        pub is_canceled: Option<bool>,
        #[prost(uint32, optional, tag = "4")]
        pub time_created: Option<u32>,
    }
}

/// Device auth get own authorized devices response
#[derive(Clone, PartialEq, Message)]
pub struct CDeviceAuthGetOwnAuthorizedDevicesResponse {
    #[prost(message, repeated, tag = "1")]
    pub devices: Vec<c_device_auth_get_own_authorized_devices_response::Device>,
}

pub mod c_device_auth_get_own_authorized_devices_response {
    use prost::Message;
    #[derive(Clone, PartialEq, Message)]
    pub struct Device {
        #[prost(fixed64, optional, tag = "1")]
        pub auth_device_token: Option<u64>,
        #[prost(string, optional, tag = "2")]
        pub device_name: Option<String>,
        #[prost(bool, optional, tag = "3")]
        pub is_pending: Option<bool>,
        #[prost(bool, optional, tag = "4")]
        pub is_canceled: Option<bool>,
        #[prost(uint32, optional, tag = "5")]
        pub last_time_used: Option<u32>,
        #[prost(fixed64, optional, tag = "6")]
        pub last_borrower_id: Option<u64>,
        #[prost(uint32, optional, tag = "7")]
        pub last_app_played: Option<u32>,
        #[prost(bool, optional, tag = "8")]
        pub is_limited: Option<bool>,
    }
}

/// Client authorize local device request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientAuthorizeLocalDeviceRequest {
    #[prost(string, optional, tag = "1")]
    pub device_description: Option<String>,
    #[prost(uint32, optional, tag = "2")]
    pub owner_account_id: Option<u32>,
    #[prost(uint64, optional, tag = "3")]
    pub local_device_token: Option<u64>,
}

/// Client authorize local device response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientAuthorizeLocalDevice {
    #[prost(int32, optional, tag = "1", default = "2")]
    pub eresult: Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub owner_account_id: Option<u32>,
    #[prost(uint64, optional, tag = "3")]
    pub authed_device_token: Option<u64>,
}

/// Client deauthorize device request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientDeauthorizeDeviceRequest {
    #[prost(uint32, optional, tag = "1")]
    pub deauthorization_account_id: Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub deauthorization_device_token: Option<u64>,
}

/// Client deauthorize device response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientDeauthorizeDevice {
    #[prost(int32, optional, tag = "1", default = "2")]
    pub eresult: Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub deauthorization_account_id: Option<u32>,
}

/// Client use local device authorizations
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientUseLocalDeviceAuthorizations {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub authorization_account_id: Vec<u32>,
    #[prost(message, repeated, tag = "2")]
    pub device_tokens: Vec<c_msg_client_use_local_device_authorizations::DeviceToken>,
}

pub mod c_msg_client_use_local_device_authorizations {
    use prost::Message;
    #[derive(Clone, PartialEq, Message)]
    pub struct DeviceToken {
        #[prost(uint32, optional, tag = "1")]
        pub owner_account_id: Option<u32>,
        #[prost(uint64, optional, tag = "2")]
        pub token_id: Option<u64>,
    }
}

/// Device auth add authorized borrowers response
#[derive(Clone, PartialEq, Message)]
pub struct CDeviceAuthAddAuthorizedBorrowersResponse {
    #[prost(int32, optional, tag = "1")]
    pub seconds_to_wait: Option<i32>,
}

/// Device auth remove authorized borrowers response
#[derive(Clone, PartialEq, Message)]
pub struct CDeviceAuthRemoveAuthorizedBorrowersResponse {}

/// Request free license response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRequestFreeLicenseResponse {
    #[prost(uint32, optional, tag = "1", default = "2")]
    pub eresult: Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub granted_packageids: Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub granted_appids: Vec<u32>,
}
