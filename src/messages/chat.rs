//! Chat/friend message protobuf messages

use prost::Message;

/// Incoming friend message notification
#[derive(Clone, PartialEq, Message)]
pub struct CFriendMessagesIncomingMessageNotification {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid_friend: Option<u64>,
    #[prost(int32, optional, tag = "2")]
    pub chat_entry_type: Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub from_limited_account: Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub message: Option<String>,
    #[prost(fixed32, optional, tag = "5")]
    pub rtime32_server_timestamp: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub ordinal: Option<u32>,
    #[prost(bool, optional, tag = "7")]
    pub local_echo: Option<bool>,
    #[prost(string, optional, tag = "8")]
    pub message_no_bbcode: Option<String>,
    #[prost(bool, optional, tag = "9")]
    pub low_priority: Option<bool>,
}

/// Send message request
#[derive(Clone, PartialEq, Message)]
pub struct CFriendMessagesSendMessageRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(int32, optional, tag = "2")]
    pub chat_entry_type: Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub message: Option<String>,
    #[prost(bool, optional, tag = "4")]
    pub contains_bbcode: Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub echo_to_sender: Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub low_priority: Option<bool>,
    #[prost(string, optional, tag = "8")]
    pub client_message_id: Option<String>,
}

/// Get active message sessions request
#[derive(Clone, PartialEq, Message)]
pub struct CFriendMessagesGetActiveMessageSessionsRequest {
    #[prost(uint32, optional, tag = "1")]
    pub lastmessage_since: Option<u32>,
    #[prost(bool, optional, tag = "2")]
    pub only_sessions_with_messages: Option<bool>,
}

/// Ack message notification/request
#[derive(Clone, PartialEq, Message)]
pub struct CFriendMessagesAckMessageNotification {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid_partner: Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub timestamp: Option<u32>,
}

/// Get recent messages request
#[derive(Clone, PartialEq, Message)]
pub struct CFriendMessagesGetRecentMessagesRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid1: Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamid2: Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub count: Option<u32>,
    #[prost(bool, optional, tag = "4")]
    pub most_recent_conversation: Option<bool>,
    #[prost(fixed32, optional, tag = "5")]
    pub rtime32_start_time: Option<u32>,
    #[prost(bool, optional, tag = "6")]
    pub bbcode_format: Option<bool>,
    #[prost(uint32, optional, tag = "7")]
    pub start_ordinal: Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub time_last: Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub ordinal_last: Option<u32>,
}

/// Get recent messages response
#[derive(Clone, PartialEq, Message)]
pub struct CFriendMessagesGetRecentMessagesResponse {
    #[prost(message, repeated, tag = "1")]
    pub messages: Vec<c_friend_messages_get_recent_messages_response::FriendMessage>,
    #[prost(bool, optional, tag = "4")]
    pub more_available: Option<bool>,
}

pub mod c_friend_messages_get_recent_messages_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct FriendMessage {
        #[prost(uint32, optional, tag = "1")]
        pub accountid: Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub timestamp: Option<u32>,
        #[prost(string, optional, tag = "3")]
        pub message: Option<String>,
        #[prost(uint32, optional, tag = "4")]
        pub ordinal: Option<u32>,
    }
}

/// Get active message sessions response
#[derive(Clone, PartialEq, Message)]
pub struct CFriendMessagesGetActiveMessageSessionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub message_sessions: Vec<c_friend_messages_get_active_message_sessions_response::FriendMessageSession>,
    #[prost(uint32, optional, tag = "2")]
    pub timestamp: Option<u32>,
}

pub mod c_friend_messages_get_active_message_sessions_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct FriendMessageSession {
        #[prost(uint32, optional, tag = "1")]
        pub accountid_friend: Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub last_message: Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub last_view: Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub unread_message_count: Option<u32>,
    }
}

/// Send message response
#[derive(Clone, PartialEq, Message)]
pub struct CFriendMessagesSendMessageResponse {
    #[prost(string, optional, tag = "1")]
    pub modified_message: Option<String>,
    #[prost(uint32, optional, tag = "2")]
    pub server_timestamp: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub ordinal: Option<u32>,
    #[prost(string, optional, tag = "4")]
    pub message_without_bb_code: Option<String>,
}
