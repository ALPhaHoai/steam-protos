use prost::Message;

/// CLoyaltyRewards_BatchedQueryRewardItems_Request
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CLoyaltyRewardsBatchedQueryRewardItemsRequest {
    #[prost(message, repeated, tag = "1")]
    pub requests: Vec<CLoyaltyRewardsQueryRewardItemsRequest>,
}

/// CLoyaltyRewards_QueryRewardItems_Request
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CLoyaltyRewardsQueryRewardItemsRequest {
    #[prost(uint32, repeated, tag = "1")]
    pub appids: Vec<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub time_available: Option<u32>,
    #[prost(int32, repeated, tag = "3")]
    pub community_item_classes: Vec<i32>,
    #[prost(string, optional, tag = "4")]
    pub language: Option<String>,
    #[prost(int32, optional, tag = "5")]
    pub count: Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub cursor: Option<String>,
    #[prost(int32, optional, tag = "7", default = "1")]
    pub sort: Option<i32>,
    #[prost(bool, optional, tag = "8", default = "true")]
    pub sort_descending: Option<bool>,
    #[prost(int32, repeated, tag = "9")]
    pub reward_types: Vec<i32>,
    #[prost(int32, repeated, tag = "10")]
    pub excluded_community_item_classes: Vec<i32>,
    #[prost(uint32, repeated, tag = "11")]
    pub definitionids: Vec<u32>,
    #[prost(int32, repeated, tag = "12")]
    pub filters: Vec<i32>,
    #[prost(string, repeated, tag = "13")]
    pub filter_match_all_category_tags: Vec<String>,
    #[prost(string, repeated, tag = "14")]
    pub filter_match_any_category_tags: Vec<String>,
    #[prost(uint32, repeated, tag = "15")]
    pub contains_definitionids: Vec<u32>,
    #[prost(bool, optional, tag = "16")]
    pub include_direct_purchase_disabled: Option<bool>,
    #[prost(uint32, repeated, tag = "17")]
    pub excluded_content_descriptors: Vec<u32>,
    #[prost(uint32, repeated, tag = "18")]
    pub excluded_appids: Vec<u32>,
    #[prost(uint32, repeated, tag = "19")]
    pub excluded_store_tagids: Vec<u32>,
    #[prost(uint32, repeated, tag = "20")]
    pub store_tagids: Vec<u32>,
    #[prost(string, optional, tag = "21")]
    pub search_term: Option<String>,
}

/// CLoyaltyRewards_BatchedQueryRewardItems_Response
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CLoyaltyRewardsBatchedQueryRewardItemsResponse {
    #[prost(message, repeated, tag = "1")]
    pub responses: Vec<CLoyaltyRewardsBatchedQueryRewardItemsResponseResponse>,
}

/// CLoyaltyRewards_BatchedQueryRewardItems_Response_Response
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CLoyaltyRewardsBatchedQueryRewardItemsResponseResponse {
    #[prost(int32, optional, tag = "1")]
    pub eresult: Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub response: Option<CLoyaltyRewardsQueryRewardItemsResponse>,
}

/// CLoyaltyRewards_QueryRewardItems_Response
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CLoyaltyRewardsQueryRewardItemsResponse {
    #[prost(message, repeated, tag = "1")]
    pub definitions: Vec<LoyaltyRewardDefinition>,
    #[prost(int32, optional, tag = "2")]
    pub total_count: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub count: Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub next_cursor: Option<String>,
}

/// LoyaltyRewardDefinition
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct LoyaltyRewardDefinition {
    #[prost(uint32, optional, tag = "1")]
    pub appid: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub defid: Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub r#type: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub community_item_class: Option<i32>,
    #[prost(uint32, optional, tag = "5")]
    pub community_item_type: Option<u32>,
    #[prost(int64, optional, tag = "6")]
    pub point_cost: Option<i64>,
    #[prost(uint32, optional, tag = "7")]
    pub timestamp_created: Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub timestamp_updated: Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub timestamp_available: Option<u32>,
    #[prost(int64, optional, tag = "10")]
    pub quantity: Option<i64>,
    #[prost(string, optional, tag = "11")]
    pub internal_description: Option<String>,
    #[prost(bool, optional, tag = "12")]
    pub active: Option<bool>,
    #[prost(message, optional, tag = "13")]
    pub community_item_data: Option<LoyaltyRewardDefinitionCommunityItemData>,
}

/// LoyaltyRewardDefinition_CommunityItemData
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct LoyaltyRewardDefinitionCommunityItemData {
    #[prost(string, optional, tag = "1")]
    pub item_name: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub item_title: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub item_description: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub item_image_small: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub item_image_large: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub item_movie_webm: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub item_movie_mp4: Option<String>,
    #[prost(bool, optional, tag = "8")]
    pub animated: Option<bool>,
}

/// CLoyaltyRewards_RedeemPoints_Request
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CLoyaltyRewardsRedeemPointsRequest {
    #[prost(uint32, optional, tag = "1")]
    pub defid: Option<u32>,
    #[prost(int64, optional, tag = "2")]
    pub expected_points_cost: Option<i64>,
}

/// CLoyaltyRewards_RedeemPoints_Response
#[derive(Clone, PartialEq, Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct CLoyaltyRewardsRedeemPointsResponse {
    #[prost(uint64, optional, tag = "1")]
    pub communityitemid: Option<u64>,
    #[prost(uint64, repeated, tag = "2")]
    pub bundle_community_item_ids: Vec<u64>,
}
