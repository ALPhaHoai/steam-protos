//! Game server protobuf messages

use prost::Message;

use super::base::CMsgIPAddress;

/// Game server query request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientGMSServerQuery {
    #[prost(uint32, optional, tag = "1")]
    pub app_id: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub geo_location_ip: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub region_code: Option<u32>,
    #[prost(string, optional, tag = "4")]
    pub filter_text: Option<String>,
    #[prost(uint32, optional, tag = "5")]
    pub max_servers: Option<u32>,
    #[prost(string, optional, tag = "6")]
    pub sdr_ping_location: Option<String>,
}

/// Game server query response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGMSClientServerQueryResponse {
    #[prost(message, repeated, tag = "1")]
    pub servers: Vec<cmsg_gms_client_server_query_response::Server>,
    #[prost(string, optional, tag = "2")]
    pub error: Option<String>,
    #[prost(message, optional, tag = "3")]
    pub default_server_data: Option<cmsg_gms_client_server_query_response::Server>,
    #[prost(string, repeated, tag = "4")]
    pub server_strings: Vec<String>,
}

pub mod cmsg_gms_client_server_query_response {
    use prost::Message;

    use super::CMsgIPAddress;

    #[derive(Clone, PartialEq, Message)]
    pub struct Server {
        #[prost(uint32, optional, tag = "1")]
        pub deprecated_server_ip: Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub query_port: Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub auth_players: Option<u32>,
        #[prost(message, optional, tag = "4")]
        pub server_ip: Option<CMsgIPAddress>,
        #[prost(fixed64, optional, tag = "6")]
        pub steam_id: Option<u64>,
        #[prost(uint32, optional, tag = "7")]
        pub revision: Option<u32>,
        #[prost(uint32, optional, tag = "8")]
        pub players: Option<u32>,
        #[prost(uint32, optional, tag = "9")]
        pub game_port: Option<u32>,
        #[prost(fixed32, optional, tag = "10")]
        pub sdr_popid: Option<u32>,
        #[prost(string, optional, tag = "32")]
        pub sdr_ping_location: Option<String>,
        #[prost(uint32, optional, tag = "11")]
        pub flags: Option<u32>,
        #[prost(uint32, optional, tag = "12")]
        pub app_id: Option<u32>,
        #[prost(uint32, optional, tag = "13")]
        pub max_players: Option<u32>,
        #[prost(uint32, optional, tag = "14")]
        pub bots: Option<u32>,
        #[prost(uint32, optional, tag = "15")]
        pub spectator_port: Option<u32>,
        #[prost(string, optional, tag = "16")]
        pub gamedir_str: Option<String>,
        #[prost(uint32, optional, tag = "17")]
        pub gamedir_strindex: Option<u32>,
        #[prost(string, optional, tag = "18")]
        pub map_str: Option<String>,
        #[prost(uint32, optional, tag = "19")]
        pub map_strindex: Option<u32>,
        #[prost(string, optional, tag = "20")]
        pub name_str: Option<String>,
        #[prost(uint32, optional, tag = "21")]
        pub name_strindex: Option<u32>,
        #[prost(string, optional, tag = "22")]
        pub game_description_str: Option<String>,
        #[prost(uint32, optional, tag = "23")]
        pub game_description_strindex: Option<u32>,
        #[prost(string, optional, tag = "24")]
        pub version_str: Option<String>,
        #[prost(uint32, optional, tag = "25")]
        pub version_strindex: Option<u32>,
        #[prost(string, optional, tag = "26")]
        pub gametype_str: Option<String>,
        #[prost(uint32, optional, tag = "27")]
        pub gametype_strindex: Option<u32>,
        #[prost(string, optional, tag = "30")]
        pub spectator_name_str: Option<String>,
        #[prost(uint32, optional, tag = "31")]
        pub spectator_name_strindex: Option<u32>,
    }
}

/// Get server steam IDs by IP request
#[derive(Clone, PartialEq, Message)]
pub struct CGameServersGetServerSteamIDsByIPRequest {
    #[prost(string, repeated, tag = "1")]
    pub server_ips: Vec<String>,
}

/// Response with server IPs and Steam IDs
#[derive(Clone, PartialEq, Message)]
pub struct CGameServersIPsWithSteamIDsResponse {
    #[prost(message, repeated, tag = "1")]
    pub servers: Vec<c_game_servers_i_ps_with_steam_i_ds_response::Server>,
}

pub mod c_game_servers_i_ps_with_steam_i_ds_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct Server {
        #[prost(string, optional, tag = "1")]
        pub addr: Option<String>,
        #[prost(fixed64, optional, tag = "2")]
        pub steamid: Option<u64>,
    }
}

/// Get server IPs by Steam ID request
#[derive(Clone, PartialEq, Message)]
pub struct CGameServersGetServerIPsBySteamIDRequest {
    #[prost(fixed64, repeated, packed = "false", tag = "1")]
    pub server_steamids: Vec<u64>,
}

/// Get server list request (unified service)
#[derive(Clone, PartialEq, Message)]
pub struct CGameServersGetServerListRequest {
    #[prost(string, optional, tag = "1")]
    pub filter: Option<String>,
    #[prost(uint32, optional, tag = "2")]
    pub limit: Option<u32>,
}

/// Get server list response
#[derive(Clone, PartialEq, Message)]
pub struct CGameServersGetServerListResponse {
    #[prost(message, repeated, tag = "1")]
    pub servers: Vec<c_game_servers_get_server_list_response::Server>,
}

pub mod c_game_servers_get_server_list_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct Server {
        #[prost(string, optional, tag = "1")]
        pub addr: Option<String>,
        #[prost(uint32, optional, tag = "2")]
        pub gameport: Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub specport: Option<u32>,
        #[prost(fixed64, optional, tag = "4")]
        pub steamid: Option<u64>,
        #[prost(string, optional, tag = "5")]
        pub name: Option<String>,
        #[prost(uint32, optional, tag = "6")]
        pub appid: Option<u32>,
        #[prost(string, optional, tag = "7")]
        pub gamedir: Option<String>,
        #[prost(string, optional, tag = "8")]
        pub version: Option<String>,
        #[prost(string, optional, tag = "9")]
        pub product: Option<String>,
        #[prost(int32, optional, tag = "10")]
        pub region: Option<i32>,
        #[prost(int32, optional, tag = "11")]
        pub players: Option<i32>,
        #[prost(int32, optional, tag = "12")]
        pub max_players: Option<i32>,
        #[prost(int32, optional, tag = "13")]
        pub bots: Option<i32>,
        #[prost(string, optional, tag = "14")]
        pub map: Option<String>,
        #[prost(bool, optional, tag = "15")]
        pub secure: Option<bool>,
        #[prost(bool, optional, tag = "16")]
        pub dedicated: Option<bool>,
        #[prost(string, optional, tag = "17")]
        pub os: Option<String>,
        #[prost(string, optional, tag = "18")]
        pub gametype: Option<String>,
    }
}
