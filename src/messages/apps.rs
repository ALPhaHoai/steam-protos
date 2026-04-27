//! Apps and CDN protobuf messages

use prost::Message;

/// Request encrypted app ticket
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRequestEncryptedAppTicket {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub userdata: Option<Vec<u8>>,
}

/// Encrypted app ticket response
#[derive(Clone, PartialEq, Message)]
pub struct EncryptedAppTicket {
    #[prost(uint32, optional, tag = "1")]
    pub ticket_version_no: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub crc_encryptedticket: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub cb_encrypteduserdata: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub cb_encrypted_appownershipticket: Option<u32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub encrypted_ticket: Option<Vec<u8>>,
}

/// Request encrypted app ticket response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientEncryptedAppTicketResponse {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(int32, optional, tag = "2", default = "2")]
    pub eresult: Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub encrypted_ticket: Option<EncryptedAppTicket>,
}

/// Get app ownership ticket request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientGetAppOwnershipTicket {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
}

/// Get app ownership ticket response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientGetAppOwnershipTicketResponse {
    #[prost(uint32, optional, tag = "1", default = "2")]
    pub eresult: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub app_id: Option<u32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub ticket: Option<Vec<u8>>,
}

/// Get number of current players request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgDpGetNumberOfCurrentPlayers {
    #[prost(uint32, optional, tag = "1")]
    pub appid: Option<u32>,
}

/// Kick playing session request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientKickPlayingSession {
    #[prost(bool, optional, tag = "1")]
    pub only_stop_game: Option<bool>,
}

/// Get depot decryption key request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientGetDepotDecryptionKey {
    #[prost(uint32, optional, tag = "1")]
    pub depot_id: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub app_id: Option<u32>,
}

/// Get CDN auth token request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientGetCdnAuthToken {
    #[prost(uint32, optional, tag = "1")]
    pub depot_id: Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub host_name: Option<String>,
    #[prost(uint32, optional, tag = "3")]
    pub app_id: Option<u32>,
}

/// Get manifest request code request
#[derive(Clone, PartialEq, Message)]
pub struct CContentServerDirectoryGetManifestRequestCodeRequest {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub depot_id: Option<u32>,
    #[prost(uint64, optional, tag = "3")]
    pub manifest_id: Option<u64>,
    #[prost(string, optional, tag = "4")]
    pub app_branch: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub branch_password_hash: Option<String>,
}

/// Get manifest request code response
#[derive(Clone, PartialEq, Message)]
pub struct CContentServerDirectoryGetManifestRequestCodeResponse {
    #[prost(uint64, optional, tag = "1")]
    pub manifest_request_code: Option<u64>,
}

/// Check app beta password request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientCheckAppBetaPassword {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub betapassword: Option<String>,
}

/// Beta password info
#[derive(Clone, PartialEq, Message)]
pub struct BetaPassword {
    #[prost(string, optional, tag = "1")]
    pub betaname: Option<String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub betapassword: Option<Vec<u8>>,
    #[prost(string, optional, tag = "3")]
    pub betadescription: Option<String>,
}

/// Check app beta password response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientCheckAppBetaPasswordResponse {
    #[prost(int32, optional, tag = "1", default = "2")]
    pub eresult: Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub betapasswords: Vec<BetaPassword>,
}
