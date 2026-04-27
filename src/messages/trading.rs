//! Trading-related protobuf messages

use prost::Message;

/// Initiate trade request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgTradingInitiateTradeRequest {
    #[prost(uint32, optional, tag = "1")]
    pub trade_request_id: Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub other_steamid: Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub other_name: Option<String>,
}

/// Initiate trade response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgTradingInitiateTradeResponse {
    #[prost(uint32, optional, tag = "1")]
    pub response: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub trade_request_id: Option<u32>,
    #[prost(uint64, optional, tag = "3")]
    pub other_steamid: Option<u64>,
    #[prost(uint32, optional, tag = "4")]
    pub steamguard_required_days: Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub new_device_cooldown_days: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub default_password_reset_probation_days: Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub password_reset_probation_days: Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub default_email_change_probation_days: Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub email_change_probation_days: Option<u32>,
}

/// Cancel trade request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgTradingCancelTradeRequest {
    #[prost(uint64, optional, tag = "1")]
    pub other_steamid: Option<u64>,
}

/// Start trading session
#[derive(Clone, PartialEq, Message)]
pub struct CMsgTradingStartSession {
    #[prost(uint64, optional, tag = "1")]
    pub other_steamid: Option<u64>,
}
