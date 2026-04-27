//! Rich presence protobuf messages

use prost::Message;

/// Rich presence upload
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRichPresenceUpload {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub rich_presence_kv: Option<Vec<u8>>,
    #[prost(fixed64, repeated, packed = "false", tag = "2")]
    pub steamid_broadcast: Vec<u64>,
}

/// Rich presence request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRichPresenceRequest {
    #[prost(fixed64, repeated, packed = "false", tag = "1")]
    pub steamid_request: Vec<u64>,
}

/// Rich presence info
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRichPresenceInfo {
    #[prost(message, repeated, tag = "1")]
    pub rich_presence: Vec<cmsg_client_rich_presence_info::RichPresence>,
}

pub mod cmsg_client_rich_presence_info {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct RichPresence {
        #[prost(fixed64, optional, tag = "1")]
        pub steamid_user: Option<u64>,
        #[prost(message, repeated, tag = "3")]
        pub rich_presense: Vec<Kv>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct Kv {
        #[prost(string, optional, tag = "1")]
        pub key: Option<String>,
        #[prost(string, optional, tag = "2")]
        pub value: Option<String>,
    }
}
