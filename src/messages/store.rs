use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct CStoreGetLocalizedNameForTagsRequest {
    #[prost(string, optional, tag = "1")]
    pub language: Option<String>,
    #[prost(uint32, repeated, tag = "2")]
    pub tagids: Vec<u32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CStoreGetLocalizedNameForTagsResponse {
    #[prost(message, repeated, tag = "1")]
    pub tags: Vec<c_store_get_localized_name_for_tags_response::Tag>,
}

pub mod c_store_get_localized_name_for_tags_response {
    use super::*;

    #[derive(Clone, PartialEq, Message)]
    pub struct Tag {
        #[prost(uint32, optional, tag = "1")]
        pub tagid: Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub english_name: Option<String>,
        #[prost(string, optional, tag = "3")]
        pub name: Option<String>,
        #[prost(string, optional, tag = "4")]
        pub normalized_name: Option<String>,
    }
}

// IStoreBrowseService/GetItems
#[derive(Clone, PartialEq, Message)]
pub struct CStoreBrowseGetItemsRequest {
    #[prost(message, repeated, tag = "1")]
    pub ids: Vec<c_store_browse_get_items_request::StoreItemId>,
    #[prost(message, optional, tag = "2")]
    pub context: Option<c_store_browse_get_items_request::StoreBrowseContext>,
    #[prost(message, optional, tag = "3")]
    pub data_request: Option<c_store_browse_get_items_request::StoreBrowseItemDataRequest>,
}

pub mod c_store_browse_get_items_request {
    use super::*;

    #[derive(Clone, PartialEq, Message)]
    pub struct StoreItemId {
        #[prost(uint32, optional, tag = "1")]
        pub appid: Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub packageid: Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub bundleid: Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub tagid: Option<u32>,
        #[prost(uint32, optional, tag = "5")]
        pub creatorid: Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub hubcategoryid: Option<u32>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct StoreBrowseContext {
        #[prost(string, optional, tag = "1")]
        pub language: Option<String>,
        #[prost(int32, optional, tag = "2")]
        pub elanguage: Option<i32>,
        #[prost(string, optional, tag = "3")]
        pub country_code: Option<String>,
        #[prost(int32, optional, tag = "4")]
        pub steam_realm: Option<i32>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct StoreBrowseItemDataRequest {
        #[prost(bool, optional, tag = "1")]
        pub include_assets: Option<bool>,
        #[prost(bool, optional, tag = "2")]
        pub include_release: Option<bool>,
        #[prost(bool, optional, tag = "3")]
        pub include_platforms: Option<bool>,
        #[prost(bool, optional, tag = "4")]
        pub include_all_purchase_options: Option<bool>,
        #[prost(bool, optional, tag = "5")]
        pub include_screenshots: Option<bool>,
        #[prost(bool, optional, tag = "6")]
        pub include_trailers: Option<bool>,
        #[prost(bool, optional, tag = "7")]
        pub include_ratings: Option<bool>,
        #[prost(int32, optional, tag = "8")]
        pub include_tag_count: Option<i32>,
        #[prost(bool, optional, tag = "9")]
        pub include_reviews: Option<bool>,
        #[prost(bool, optional, tag = "10")]
        pub include_basic_info: Option<bool>,
        #[prost(bool, optional, tag = "11")]
        pub include_supported_languages: Option<bool>,
        #[prost(bool, optional, tag = "12")]
        pub include_full_description: Option<bool>,
        #[prost(bool, optional, tag = "13")]
        pub include_included_items: Option<bool>,
        #[prost(bool, optional, tag = "14")]
        pub included_item_data_request: Option<bool>,
        #[prost(bool, optional, tag = "15")]
        pub include_assets_without_overrides: Option<bool>,
        #[prost(bool, optional, tag = "16")]
        pub apply_user_filters: Option<bool>,
        #[prost(bool, optional, tag = "17")]
        pub include_links: Option<bool>,
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct CStoreBrowseGetItemsResponse {
    #[prost(message, repeated, tag = "1")]
    pub store_items: Vec<c_store_browse_get_items_response::StoreItem>,
}

pub mod c_store_browse_get_items_response {
    use super::*;

    #[derive(Clone, PartialEq, Message)]
    pub struct StoreItem {
        #[prost(int32, optional, tag = "1")]
        pub item_type: Option<i32>,
        #[prost(uint32, optional, tag = "2")]
        pub id: Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub success: Option<u32>,
        #[prost(bool, optional, tag = "4")]
        pub visible: Option<bool>,
        #[prost(string, optional, tag = "5")]
        pub name: Option<String>,
        #[prost(string, optional, tag = "6")]
        pub store_url_path: Option<String>,
        #[prost(uint32, optional, tag = "7")]
        pub appid: Option<u32>,
        #[prost(int32, optional, tag = "8")]
        pub r#type: Option<i32>,
        #[prost(bool, optional, tag = "9")]
        pub is_free: Option<bool>,
        #[prost(bool, optional, tag = "10")]
        pub is_early_access: Option<bool>,
        #[prost(message, optional, tag = "11")]
        pub assets: Option<StoreItemAssets>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct StoreItemAssets {
        #[prost(string, optional, tag = "1")]
        pub asset_url_format: Option<String>,
        #[prost(string, optional, tag = "2")]
        pub main_capsule: Option<String>,
        #[prost(string, optional, tag = "3")]
        pub small_capsule: Option<String>,
        #[prost(string, optional, tag = "4")]
        pub header: Option<String>,
        #[prost(string, optional, tag = "5")]
        pub package_header: Option<String>,
        #[prost(string, optional, tag = "6")]
        pub page_background: Option<String>,
        #[prost(string, optional, tag = "7")]
        pub hero_capsule: Option<String>,
        #[prost(string, optional, tag = "8")]
        pub hero_capsule_2x: Option<String>,
        #[prost(string, optional, tag = "9")]
        pub library_capsule: Option<String>,
        #[prost(string, optional, tag = "10")]
        pub library_capsule_2x: Option<String>,
        #[prost(string, optional, tag = "11")]
        pub library_hero: Option<String>,
        #[prost(string, optional, tag = "12")]
        pub library_hero_2x: Option<String>,
        #[prost(string, optional, tag = "13")]
        pub community_icon: Option<String>,
    }
}
