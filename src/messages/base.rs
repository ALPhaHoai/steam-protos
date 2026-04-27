//! Base protobuf messages used across Steam protocol

use prost::Message;

/// IP address - can be either IPv4 or IPv6
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CMsgIPAddress {
    #[prost(oneof = "cmsg_ip_address::Ip", tags = "1, 2")]
    pub ip: Option<cmsg_ip_address::Ip>,
}

pub mod cmsg_ip_address {
    #[derive(Clone, PartialEq, prost::Oneof)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum Ip {
        #[prost(fixed32, tag = "1")]
        V4(u32),
        #[prost(bytes, tag = "2")]
        V6(Vec<u8>),
    }
}

/// Protobuf header for Steam messages
#[derive(Clone, PartialEq, Message)]
pub struct CMsgProtoBufHeader {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(int32, optional, tag = "2")]
    pub client_sessionid: Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub routing_appid: Option<u32>,
    #[prost(fixed64, optional, tag = "10")]
    pub jobid_source: Option<u64>,
    #[prost(fixed64, optional, tag = "11")]
    pub jobid_target: Option<u64>,
    #[prost(string, optional, tag = "12")]
    pub target_job_name: Option<String>,
    #[prost(int32, optional, tag = "24")]
    pub seq_num: Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub eresult: Option<i32>,
    #[prost(string, optional, tag = "14")]
    pub error_message: Option<String>,
    #[prost(uint32, optional, tag = "16")]
    pub auth_account_flags: Option<u32>,
    #[prost(uint32, optional, tag = "22")]
    pub token_source: Option<u32>,
    #[prost(bool, optional, tag = "23")]
    pub admin_spoofing_user: Option<bool>,
    #[prost(int32, optional, tag = "17")]
    pub transport_error: Option<i32>,
    #[prost(uint64, optional, tag = "18")]
    pub messageid: Option<u64>,
    #[prost(uint32, optional, tag = "19")]
    pub publisher_group_id: Option<u32>,
    #[prost(uint32, optional, tag = "20")]
    pub sysid: Option<u32>,
    #[prost(uint64, optional, tag = "36")]
    pub token_id: Option<u64>,
    #[prost(uint32, optional, tag = "31")]
    pub launcher_type: Option<u32>,
    #[prost(uint32, optional, tag = "32")]
    pub realm: Option<u32>,
    #[prost(int32, optional, tag = "33")]
    pub timeout_ms: Option<i32>,
    #[prost(string, optional, tag = "34")]
    pub debug_source: Option<String>,
    #[prost(uint32, optional, tag = "35")]
    pub debug_source_string_index: Option<u32>,
    #[prost(oneof = "cmsg_proto_buf_header::IpAddr", tags = "15, 29")]
    pub ip_addr: Option<cmsg_proto_buf_header::IpAddr>,
}

pub mod cmsg_proto_buf_header {
    #[derive(Clone, PartialEq, prost::Oneof)]
    pub enum IpAddr {
        #[prost(uint32, tag = "15")]
        Ip(u32),
        #[prost(bytes, tag = "29")]
        IpV6(Vec<u8>),
    }
}

/// Multi message - contains compressed payload
#[derive(Clone, PartialEq, Message)]
pub struct CMsgMulti {
    #[prost(uint32, optional, tag = "1")]
    pub size_unzipped: Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub message_body: Option<Vec<u8>>,
}

/// Authentication ticket
#[derive(Clone, PartialEq, Message)]
pub struct CMsgAuthTicket {
    #[prost(uint32, optional, tag = "1")]
    pub estate: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub eresult: Option<u32>,
    #[prost(fixed64, optional, tag = "3")]
    pub steamid: Option<u64>,
    #[prost(fixed64, optional, tag = "4")]
    pub gameid: Option<u64>,
    #[prost(uint32, optional, tag = "5")]
    pub h_steam_pipe: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub ticket_crc: Option<u32>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub ticket: Option<Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub server_secret: Option<Vec<u8>>,
    #[prost(uint32, optional, tag = "9")]
    pub ticket_type: Option<u32>,
}

/// Key-value pair
#[derive(Clone, PartialEq, Message)]
pub struct CMsgKeyValuePair {
    #[prost(string, optional, tag = "1")]
    pub name: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub value: Option<String>,
}

/// Key-value set
#[derive(Clone, PartialEq, Message)]
pub struct CMsgKeyValueSet {
    #[prost(message, repeated, tag = "1")]
    pub pairs: Vec<CMsgKeyValuePair>,
}

/// Protobuf wrapped message
#[derive(Clone, PartialEq, Message)]
pub struct CMsgProtobufWrapped {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub message_body: Option<Vec<u8>>,
}
