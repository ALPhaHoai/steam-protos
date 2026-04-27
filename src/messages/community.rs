use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct CCommunityGetAppRichPresenceLocalizationRequest {
    #[prost(int32, optional, tag = "1")]
    pub appid: Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub language: Option<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CCommunityGetAppRichPresenceLocalizationResponse {
    #[prost(int32, optional, tag = "1")]
    pub appid: Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub token_lists: Vec<c_community_get_app_rich_presence_localization_response::TokenList>,
}

pub mod c_community_get_app_rich_presence_localization_response {
    use super::*;

    #[derive(Clone, PartialEq, Message)]
    pub struct Token {
        #[prost(string, optional, tag = "1")]
        pub name: Option<String>,
        #[prost(string, optional, tag = "2")]
        pub value: Option<String>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct TokenList {
        #[prost(string, optional, tag = "1")]
        pub language: Option<String>,
        #[prost(message, repeated, tag = "2")]
        pub tokens: Vec<Token>,
    }
}

// ICommunityService/GetApps
#[derive(Clone, PartialEq, Message)]
pub struct CCommunityGetAppsRequest {
    #[prost(uint32, repeated, tag = "1")]
    pub appids: Vec<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub language: Option<u32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CCommunityGetAppsResponse {
    #[prost(message, repeated, tag = "1")]
    pub apps: Vec<c_community_get_apps_response::App>,
}

pub mod c_community_get_apps_response {
    use super::*;

    #[derive(Clone, PartialEq, Message)]
    pub struct App {
        #[prost(uint32, optional, tag = "1")]
        pub appid: Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub name: Option<String>,
        #[prost(string, optional, tag = "3")]
        pub icon: Option<String>,
        #[prost(string, optional, tag = "4")]
        pub logo: Option<String>,
        #[prost(bool, optional, tag = "5")]
        pub community_visible_stats: Option<bool>,
        #[prost(string, optional, tag = "6")]
        pub propagation: Option<String>,
        #[prost(bool, optional, tag = "7")]
        pub has_community_visible_stats: Option<bool>,
    }
}

// ICommunityService/GetAvatarHistory
#[derive(Clone, PartialEq, Message)]
pub struct CCommunityGetAvatarHistoryRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(bool, optional, tag = "2")]
    pub filter_user_uploaded_only: Option<bool>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CCommunityGetAvatarHistoryResponse {
    #[prost(message, repeated, tag = "1")]
    pub avatars: Vec<c_community_get_avatar_history_response::AvatarData>,
}

pub mod c_community_get_avatar_history_response {
    use super::*;

    #[derive(Clone, PartialEq, Message)]
    pub struct AvatarData {
        #[prost(string, optional, tag = "1")]
        pub avatar_sha1: Option<String>,
        #[prost(bool, optional, tag = "2")]
        pub user_uploaded: Option<bool>,
        #[prost(uint32, optional, tag = "3")]
        pub timestamp: Option<u32>,
    }
}
