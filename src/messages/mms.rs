//! MMS (Matchmaking Service) protobuf messages

use prost::Message;

use super::base::CMsgIPAddress;

/// CMsgClientMMSCreateLobby
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientMmsCreateLobby {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub max_members: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub lobby_type: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub lobby_flags: Option<i32>,
    #[prost(uint32, optional, tag = "5")]
    pub cell_id: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub deprecated_public_ip: Option<u32>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub metadata: Option<Vec<u8>>,
    #[prost(string, optional, tag = "8")]
    pub persona_name_owner: Option<String>,
    #[prost(message, optional, tag = "9")]
    pub public_ip: Option<CMsgIPAddress>,
}

/// CMsgClientMMSCreateLobbyResponse
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientMmsCreateLobbyResponse {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id_lobby: Option<u64>,
    #[prost(int32, optional, tag = "3", default = "2")]
    pub eresult: Option<i32>,
}

/// CMsgClientMMSJoinLobby
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientMmsJoinLobby {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id_lobby: Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub persona_name: Option<String>,
}

/// CMsgClientMMSJoinLobbyResponse
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientMmsJoinLobbyResponse {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id_lobby: Option<u64>,
    #[prost(int32, optional, tag = "3")]
    pub chat_room_enter_response: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub max_members: Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub lobby_type: Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub lobby_flags: Option<i32>,
    #[prost(fixed64, optional, tag = "7")]
    pub steam_id_owner: Option<u64>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub metadata: Option<Vec<u8>>,
    #[prost(message, repeated, tag = "9")]
    pub members: Vec<c_msg_client_mms_join_lobby_response::Member>,
}

pub mod c_msg_client_mms_join_lobby_response {
    use prost::Message;
    #[derive(Clone, PartialEq, Message)]
    pub struct Member {
        #[prost(fixed64, optional, tag = "1")]
        pub steam_id: Option<u64>,
        #[prost(string, optional, tag = "2")]
        pub persona_name: Option<String>,
        #[prost(bytes = "vec", optional, tag = "3")]
        pub metadata: Option<Vec<u8>>,
    }
}

/// CMsgClientMMSInviteToLobby
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientMmsInviteToLobby {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id_lobby: Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub steam_id_user_invited: Option<u64>,
}

/// CMsgClientMMSLeaveLobby
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientMmsLeaveLobby {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id_lobby: Option<u64>,
}

/// CMsgClientMMSLeaveLobbyResponse
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientMmsLeaveLobbyResponse {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id_lobby: Option<u64>,
    #[prost(int32, optional, tag = "3", default = "2")]
    pub eresult: Option<i32>,
}

/// CMsgClientMMSSetLobbyData
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientMmsSetLobbyData {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id_lobby: Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub steam_id_member: Option<u64>,
    #[prost(int32, optional, tag = "4")]
    pub max_members: Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub lobby_type: Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub lobby_flags: Option<i32>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub metadata: Option<Vec<u8>>,
}

/// CMsgClientMMSSetLobbyDataResponse
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientMmsSetLobbyDataResponse {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id_lobby: Option<u64>,
    #[prost(int32, optional, tag = "3", default = "2")]
    pub eresult: Option<i32>,
}

/// CMsgClientMMSGetLobbyData
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientMmsGetLobbyData {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id_lobby: Option<u64>,
}

/// CMsgClientMMSLobbyData - Response to GetLobbyData
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientMmsLobbyData {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id_lobby: Option<u64>,
    #[prost(int32, optional, tag = "3")]
    pub num_members: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub max_members: Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub lobby_type: Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub lobby_flags: Option<i32>,
    #[prost(fixed64, optional, tag = "7")]
    pub steam_id_owner: Option<u64>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub metadata: Option<Vec<u8>>,
    #[prost(message, repeated, tag = "9")]
    pub members: Vec<c_msg_client_mms_lobby_data::Member>,
    #[prost(uint32, optional, tag = "10")]
    pub lobby_cellid: Option<u32>,
    #[prost(bool, optional, tag = "11")]
    pub owner_should_accept_changes: Option<bool>,
}

pub mod c_msg_client_mms_lobby_data {
    use prost::Message;
    #[derive(Clone, PartialEq, Message)]
    pub struct Member {
        #[prost(fixed64, optional, tag = "1")]
        pub steam_id: Option<u64>,
        #[prost(string, optional, tag = "2")]
        pub persona_name: Option<String>,
        #[prost(bytes = "vec", optional, tag = "3")]
        pub metadata: Option<Vec<u8>>,
        #[prost(string, optional, tag = "4")]
        pub ping_data: Option<String>,
    }
}
