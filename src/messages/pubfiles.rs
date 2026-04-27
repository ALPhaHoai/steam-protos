//! Published file protobuf messages

use prost::Message;

/// Get published file details request
#[derive(Clone, PartialEq, Message)]
pub struct CPublishedFileGetDetailsRequest {
    #[prost(fixed64, repeated, packed = "false", tag = "1")]
    pub publishedfileids: Vec<u64>,
    #[prost(bool, optional, tag = "2")]
    pub includetags: Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub includeadditionalpreviews: Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub includechildren: Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub includekvtags: Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub includevotes: Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub short_description: Option<bool>,
    #[prost(bool, optional, tag = "10")]
    pub includeforsaledata: Option<bool>,
    #[prost(bool, optional, tag = "11")]
    pub includemetadata: Option<bool>,
    #[prost(int32, optional, tag = "12")]
    pub language: Option<i32>,
    #[prost(uint32, optional, tag = "13")]
    pub return_playtime_stats: Option<u32>,
    #[prost(uint32, optional, tag = "14")]
    pub appid: Option<u32>,
    #[prost(bool, optional, tag = "15")]
    pub strip_description_bbcode: Option<bool>,
}

/// Get published file details response
#[derive(Clone, PartialEq, Message)]
pub struct CPublishedFileGetDetailsResponse {
    #[prost(message, repeated, tag = "1")]
    pub publishedfiledetails: Vec<PublishedFileDetails>,
}

/// Published file details
#[derive(Clone, PartialEq, Message)]
pub struct PublishedFileDetails {
    #[prost(uint32, optional, tag = "1")]
    pub result: Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub publishedfileid: Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub creator: Option<u64>,
    #[prost(uint32, optional, tag = "4")]
    pub creator_appid: Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub consumer_appid: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub consumer_shortcutid: Option<u32>,
    #[prost(string, optional, tag = "7")]
    pub filename: Option<String>,
    #[prost(uint64, optional, tag = "8")]
    pub file_size: Option<u64>,
    #[prost(uint64, optional, tag = "9")]
    pub preview_file_size: Option<u64>,
    #[prost(string, optional, tag = "10")]
    pub file_url: Option<String>,
    #[prost(string, optional, tag = "11")]
    pub preview_url: Option<String>,
    #[prost(string, optional, tag = "12")]
    pub youtubevideoid: Option<String>,
    #[prost(string, optional, tag = "13")]
    pub url: Option<String>,
    #[prost(fixed64, optional, tag = "14")]
    pub hcontent_file: Option<u64>,
    #[prost(fixed64, optional, tag = "15")]
    pub hcontent_preview: Option<u64>,
    #[prost(string, optional, tag = "16")]
    pub title: Option<String>,
    #[prost(string, optional, tag = "17")]
    pub file_description: Option<String>,
    #[prost(string, optional, tag = "18")]
    pub short_description: Option<String>,
    #[prost(uint32, optional, tag = "19")]
    pub time_created: Option<u32>,
    #[prost(uint32, optional, tag = "20")]
    pub time_updated: Option<u32>,
    #[prost(uint32, optional, tag = "21")]
    pub visibility: Option<u32>,
    #[prost(uint32, optional, tag = "22")]
    pub flags: Option<u32>,
    #[prost(bool, optional, tag = "23")]
    pub workshop_file: Option<bool>,
    #[prost(bool, optional, tag = "24")]
    pub workshop_accepted: Option<bool>,
    #[prost(bool, optional, tag = "28")]
    pub banned: Option<bool>,
    #[prost(string, optional, tag = "29")]
    pub ban_reason: Option<String>,
    #[prost(fixed64, optional, tag = "30")]
    pub banner: Option<u64>,
    #[prost(bool, optional, tag = "31")]
    pub can_be_deleted: Option<bool>,
    #[prost(string, optional, tag = "33")]
    pub app_name: Option<String>,
    #[prost(uint32, optional, tag = "34")]
    pub file_type: Option<u32>,
    #[prost(bool, optional, tag = "35")]
    pub can_subscribe: Option<bool>,
    #[prost(uint32, optional, tag = "36")]
    pub subscriptions: Option<u32>,
    #[prost(uint32, optional, tag = "37")]
    pub favorited: Option<u32>,
    #[prost(uint32, optional, tag = "38")]
    pub followers: Option<u32>,
    #[prost(uint32, optional, tag = "39")]
    pub lifetime_subscriptions: Option<u32>,
    #[prost(uint32, optional, tag = "40")]
    pub lifetime_favorited: Option<u32>,
    #[prost(uint32, optional, tag = "41")]
    pub lifetime_followers: Option<u32>,
    #[prost(uint32, optional, tag = "42")]
    pub views: Option<u32>,
    #[prost(message, repeated, tag = "52")]
    pub tags: Vec<published_file_details::Tag>,
    #[prost(message, repeated, tag = "54")]
    pub kvtags: Vec<published_file_details::KvTag>,
    #[prost(message, optional, tag = "55")]
    pub vote_data: Option<published_file_details::VoteData>,
}

pub mod published_file_details {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct Tag {
        #[prost(string, optional, tag = "1")]
        pub tag: Option<String>,
        #[prost(bool, optional, tag = "2")]
        pub adminonly: Option<bool>,
        #[prost(string, optional, tag = "3")]
        pub display_name: Option<String>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct KvTag {
        #[prost(string, optional, tag = "1")]
        pub key: Option<String>,
        #[prost(string, optional, tag = "2")]
        pub value: Option<String>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct VoteData {
        #[prost(float, optional, tag = "1")]
        pub score: Option<f32>,
        #[prost(uint32, optional, tag = "2")]
        pub votes_up: Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub votes_down: Option<u32>,
    }
}
