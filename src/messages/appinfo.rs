//! AppInfo/PICS protobuf messages

use prost::Message;

/// PICS changes since request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientPICSChangesSinceRequest {
    #[prost(uint32, optional, tag = "1")]
    pub since_change_number: Option<u32>,
    #[prost(bool, optional, tag = "2")]
    pub send_app_info_changes: Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub send_package_info_changes: Option<bool>,
    #[prost(uint32, optional, tag = "4")]
    pub num_app_info_cached: Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub num_package_info_cached: Option<u32>,
}

/// PICS changes since response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientPICSChangesSinceResponse {
    #[prost(uint32, optional, tag = "1")]
    pub current_change_number: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub since_change_number: Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub force_full_update: Option<bool>,
    #[prost(message, repeated, tag = "4")]
    pub package_changes: Vec<cmsg_client_pics_changes_since_response::PackageChange>,
    #[prost(message, repeated, tag = "5")]
    pub app_changes: Vec<cmsg_client_pics_changes_since_response::AppChange>,
    #[prost(bool, optional, tag = "6")]
    pub force_full_app_update: Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub force_full_package_update: Option<bool>,
}

pub mod cmsg_client_pics_changes_since_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct PackageChange {
        #[prost(uint32, optional, tag = "1")]
        pub packageid: Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub change_number: Option<u32>,
        #[prost(bool, optional, tag = "3")]
        pub needs_token: Option<bool>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct AppChange {
        #[prost(uint32, optional, tag = "1")]
        pub appid: Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub change_number: Option<u32>,
        #[prost(bool, optional, tag = "3")]
        pub needs_token: Option<bool>,
    }
}

/// PICS product info request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientPICSProductInfoRequest {
    #[prost(message, repeated, tag = "1")]
    pub packages: Vec<cmsg_client_pics_product_info_request::PackageInfo>,
    #[prost(message, repeated, tag = "2")]
    pub apps: Vec<cmsg_client_pics_product_info_request::AppInfo>,
    #[prost(bool, optional, tag = "3")]
    pub meta_data_only: Option<bool>,
    #[prost(uint32, optional, tag = "4")]
    pub num_prev_failed: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub sequence_number: Option<u32>,
    #[prost(bool, optional, tag = "7")]
    pub single_response: Option<bool>,
}

pub mod cmsg_client_pics_product_info_request {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct AppInfo {
        #[prost(uint32, optional, tag = "1")]
        pub appid: Option<u32>,
        #[prost(uint64, optional, tag = "2")]
        pub access_token: Option<u64>,
        #[prost(bool, optional, tag = "3")]
        pub only_public_obsolete: Option<bool>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct PackageInfo {
        #[prost(uint32, optional, tag = "1")]
        pub packageid: Option<u32>,
        #[prost(uint64, optional, tag = "2")]
        pub access_token: Option<u64>,
    }
}

/// PICS product info response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientPICSProductInfoResponse {
    #[prost(message, repeated, tag = "1")]
    pub apps: Vec<cmsg_client_pics_product_info_response::AppInfo>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub unknown_appids: Vec<u32>,
    #[prost(message, repeated, tag = "3")]
    pub packages: Vec<cmsg_client_pics_product_info_response::PackageInfo>,
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub unknown_packageids: Vec<u32>,
    #[prost(bool, optional, tag = "5")]
    pub meta_data_only: Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub response_pending: Option<bool>,
    #[prost(uint32, optional, tag = "7")]
    pub http_min_size: Option<u32>,
    #[prost(string, optional, tag = "8")]
    pub http_host: Option<String>,
}

pub mod cmsg_client_pics_product_info_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct AppInfo {
        #[prost(uint32, optional, tag = "1")]
        pub appid: Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub change_number: Option<u32>,
        #[prost(bool, optional, tag = "3")]
        pub missing_token: Option<bool>,
        #[prost(bytes = "vec", optional, tag = "4")]
        pub sha: Option<Vec<u8>>,
        #[prost(bytes = "vec", optional, tag = "5")]
        pub buffer: Option<Vec<u8>>,
        #[prost(bool, optional, tag = "6")]
        pub only_public: Option<bool>,
        #[prost(uint32, optional, tag = "7")]
        pub size: Option<u32>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct PackageInfo {
        #[prost(uint32, optional, tag = "1")]
        pub packageid: Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub change_number: Option<u32>,
        #[prost(bool, optional, tag = "3")]
        pub missing_token: Option<bool>,
        #[prost(bytes = "vec", optional, tag = "4")]
        pub sha: Option<Vec<u8>>,
        #[prost(bytes = "vec", optional, tag = "5")]
        pub buffer: Option<Vec<u8>>,
        #[prost(uint32, optional, tag = "6")]
        pub size: Option<u32>,
    }
}

/// PICS access token request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientPICSAccessTokenRequest {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub packageids: Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub appids: Vec<u32>,
}

/// PICS access token response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientPICSAccessTokenResponse {
    #[prost(message, repeated, tag = "1")]
    pub package_access_tokens: Vec<cmsg_client_pics_access_token_response::PackageToken>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub package_denied_tokens: Vec<u32>,
    #[prost(message, repeated, tag = "3")]
    pub app_access_tokens: Vec<cmsg_client_pics_access_token_response::AppToken>,
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub app_denied_tokens: Vec<u32>,
}

pub mod cmsg_client_pics_access_token_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct PackageToken {
        #[prost(uint32, optional, tag = "1")]
        pub packageid: Option<u32>,
        #[prost(uint64, optional, tag = "2")]
        pub access_token: Option<u64>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct AppToken {
        #[prost(uint32, optional, tag = "1")]
        pub appid: Option<u32>,
        #[prost(uint64, optional, tag = "2")]
        pub access_token: Option<u64>,
    }
}
