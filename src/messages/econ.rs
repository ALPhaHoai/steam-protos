//! Economy service protobuf messages
//!
//! Messages for Steam economy features like trade URLs, asset class info,
//! emoticons, and profile items.

use prost::Message;

// ============================================================================
// Asset Class Info
// ============================================================================

/// Request asset class information for items
#[derive(Clone, PartialEq, Message)]
pub struct CEconGetAssetClassInfoRequest {
    /// Language for descriptions
    #[prost(string, optional, tag = "1")]
    pub language: Option<String>,
    /// App ID
    #[prost(uint32, optional, tag = "2")]
    pub appid: Option<u32>,
    /// List of asset classes to look up
    #[prost(message, repeated, tag = "3")]
    pub classes: Vec<c_econ_get_asset_class_info_request::Class>,
}

pub mod c_econ_get_asset_class_info_request {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct Class {
        /// Class ID of the asset
        #[prost(uint64, optional, tag = "1")]
        pub classid: Option<u64>,
        /// Instance ID of the asset (optional)
        #[prost(uint64, optional, tag = "2")]
        pub instanceid: Option<u64>,
    }
}

/// Response for asset class information
#[derive(Clone, PartialEq, Message)]
pub struct CEconGetAssetClassInfoResponse {
    /// Asset descriptions
    #[prost(message, repeated, tag = "1")]
    pub descriptions: Vec<CEconAssetDescription>,
}

/// Asset description data
#[derive(Clone, PartialEq, Message)]
pub struct CEconAssetDescription {
    /// App ID
    #[prost(int32, optional, tag = "1")]
    pub appid: Option<i32>,
    /// Class ID
    #[prost(uint64, optional, tag = "2")]
    pub classid: Option<u64>,
    /// Instance ID
    #[prost(uint64, optional, tag = "3")]
    pub instanceid: Option<u64>,
    /// Display name
    #[prost(string, optional, tag = "4")]
    pub name: Option<String>,
    /// Market hash name
    #[prost(string, optional, tag = "5")]
    pub market_hash_name: Option<String>,
    /// Market name
    #[prost(string, optional, tag = "6")]
    pub market_name: Option<String>,
    /// Item color
    #[prost(string, optional, tag = "7")]
    pub name_color: Option<String>,
    /// Background color
    #[prost(string, optional, tag = "8")]
    pub background_color: Option<String>,
    /// Type text
    #[prost(string, optional, tag = "9")]
    pub r#type: Option<String>,
    /// Icon URL suffix
    #[prost(string, optional, tag = "10")]
    pub icon_url: Option<String>,
    /// Large icon URL suffix
    #[prost(string, optional, tag = "11")]
    pub icon_url_large: Option<String>,
    /// Is tradable
    #[prost(bool, optional, tag = "12")]
    pub tradable: Option<bool>,
    /// Is marketable
    #[prost(bool, optional, tag = "13")]
    pub marketable: Option<bool>,
    /// Is commodity
    #[prost(bool, optional, tag = "14")]
    pub commodity: Option<bool>,
    /// Currency type
    #[prost(int32, optional, tag = "15")]
    pub currency: Option<i32>,
}

// ============================================================================
// Trade URL / Access Token
// ============================================================================

/// Request trade offer access token
#[derive(Clone, PartialEq, Message)]
pub struct CEconGetTradeOfferAccessTokenRequest {
    /// Generate a new token
    #[prost(bool, optional, tag = "1")]
    pub generate_new_token: Option<bool>,
}

/// Response with trade offer access token
#[derive(Clone, PartialEq, Message)]
pub struct CEconGetTradeOfferAccessTokenResponse {
    /// The trade offer access token
    #[prost(string, optional, tag = "1")]
    pub trade_offer_access_token: Option<String>,
}

// ============================================================================
// Emoticon List
// ============================================================================

/// Request emoticon list for player
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetEmoticonListRequest {}

/// Response with emoticon list
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetEmoticonListResponse {
    /// List of emoticons
    #[prost(message, repeated, tag = "1")]
    pub emoticons: Vec<CPlayerEmoticon>,
}

/// Emoticon data
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerEmoticon {
    /// Emoticon name (e.g., ":steamhappy:")
    #[prost(string, optional, tag = "1")]
    pub name: Option<String>,
    /// Number of times used
    #[prost(int32, optional, tag = "2")]
    pub count: Option<i32>,
    /// Timestamp when acquired
    #[prost(uint32, optional, tag = "3")]
    pub time_last_used: Option<u32>,
    /// Number of uses
    #[prost(uint32, optional, tag = "4")]
    pub use_count: Option<u32>,
    /// Time received
    #[prost(uint32, optional, tag = "5")]
    pub time_received: Option<u32>,
    /// App ID that granted this emoticon
    #[prost(uint32, optional, tag = "6")]
    pub appid: Option<u32>,
}

// ============================================================================
// Profile Items
// ============================================================================

/// Request owned profile items
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetProfileItemsOwnedRequest {
    /// Language for item descriptions
    #[prost(string, optional, tag = "1")]
    pub language: Option<String>,
}

/// Response with owned profile items
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetProfileItemsOwnedResponse {
    /// Profile backgrounds
    #[prost(message, repeated, tag = "1")]
    pub profile_backgrounds: Vec<CPlayerProfileItem>,
    /// Mini profile backgrounds
    #[prost(message, repeated, tag = "2")]
    pub mini_profile_backgrounds: Vec<CPlayerProfileItem>,
    /// Avatar frames
    #[prost(message, repeated, tag = "3")]
    pub avatar_frames: Vec<CPlayerProfileItem>,
    /// Animated avatars
    #[prost(message, repeated, tag = "4")]
    pub animated_avatars: Vec<CPlayerProfileItem>,
    /// Profile modifiers
    #[prost(message, repeated, tag = "5")]
    pub profile_modifiers: Vec<CPlayerProfileItem>,
}

/// Request equipped profile items
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetProfileItemsEquippedRequest {
    /// Steam ID to look up
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    /// Language for item descriptions
    #[prost(string, optional, tag = "2")]
    pub language: Option<String>,
}

/// Response with equipped profile items
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetProfileItemsEquippedResponse {
    /// Profile background
    #[prost(message, optional, tag = "1")]
    pub profile_background: Option<CPlayerProfileItem>,
    /// Mini profile background
    #[prost(message, optional, tag = "2")]
    pub mini_profile_background: Option<CPlayerProfileItem>,
    /// Avatar frame
    #[prost(message, optional, tag = "3")]
    pub avatar_frame: Option<CPlayerProfileItem>,
    /// Animated avatar
    #[prost(message, optional, tag = "4")]
    pub animated_avatar: Option<CPlayerProfileItem>,
    /// Profile modifier
    #[prost(message, optional, tag = "5")]
    pub profile_modifier: Option<CPlayerProfileItem>,
}

/// Profile item data
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerProfileItem {
    /// Community item ID
    #[prost(uint64, optional, tag = "1")]
    pub communityitemid: Option<u64>,
    /// Large image URL path
    #[prost(string, optional, tag = "2")]
    pub image_large: Option<String>,
    /// Small image URL path
    #[prost(string, optional, tag = "3")]
    pub image_small: Option<String>,
    /// Item name
    #[prost(string, optional, tag = "4")]
    pub name: Option<String>,
    /// Item title
    #[prost(string, optional, tag = "5")]
    pub item_title: Option<String>,
    /// Item description
    #[prost(string, optional, tag = "6")]
    pub item_description: Option<String>,
    /// App ID that granted this item
    #[prost(uint32, optional, tag = "7")]
    pub appid: Option<u32>,
    /// Item type
    #[prost(uint32, optional, tag = "8")]
    pub item_type: Option<u32>,
    /// Item class
    #[prost(uint32, optional, tag = "9")]
    pub item_class: Option<u32>,
    /// Movie WebM URL path
    #[prost(string, optional, tag = "10")]
    pub movie_webm: Option<String>,
    /// Movie MP4 URL path
    #[prost(string, optional, tag = "11")]
    pub movie_mp4: Option<String>,
    /// Movie WebM small URL path
    #[prost(string, optional, tag = "12")]
    pub movie_webm_small: Option<String>,
    /// Movie MP4 small URL path
    #[prost(string, optional, tag = "13")]
    pub movie_mp4_small: Option<String>,
    /// Flags
    #[prost(uint32, optional, tag = "14")]
    pub flags: Option<u32>,
}

/// Request to set profile background
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerSetProfileBackgroundRequest {
    /// Community item ID of the background to set
    #[prost(uint64, optional, tag = "1")]
    pub communityitemid: Option<u64>,
}

/// Response for setting profile background
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerSetProfileBackgroundResponse {}

// ============================================================================
// GC Econ Messages
// ============================================================================

/// Econ Item Attribute
#[derive(Clone, PartialEq, Message)]
pub struct CSOEconItemAttribute {
    #[prost(uint32, optional, tag = "1")]
    pub def_index: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub value: Option<u32>,
    #[prost(bytes, optional, tag = "3")]
    pub value_bytes: Option<Vec<u8>>,
}

/// Econ Item Equipped state
#[derive(Clone, PartialEq, Message)]
pub struct CSOEconItemEquipped {
    #[prost(uint32, optional, tag = "1")]
    pub new_class: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub new_slot: Option<u32>,
}

/// Econ Item
#[derive(Clone, PartialEq, Message)]
pub struct CSOEconItem {
    #[prost(uint64, optional, tag = "1")]
    pub id: Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub account_id: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub inventory: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub def_index: Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub quantity: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub level: Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub quality: Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub flags: Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub origin: Option<u32>,
    #[prost(string, optional, tag = "10")]
    pub custom_name: Option<String>,
    #[prost(string, optional, tag = "11")]
    pub custom_desc: Option<String>,
    #[prost(message, repeated, tag = "12")]
    pub attribute: Vec<CSOEconItemAttribute>,
    #[prost(message, optional, boxed, tag = "13")]
    pub interior_item: Option<Box<CSOEconItem>>,
    #[prost(bool, optional, tag = "14")]
    pub in_use: Option<bool>,
    #[prost(uint32, optional, tag = "15")]
    pub style: Option<u32>,
    #[prost(uint64, optional, tag = "16")]
    pub original_id: Option<u64>,
    #[prost(message, repeated, tag = "18")]
    pub equipped_state: Vec<CSOEconItemEquipped>,
    #[prost(uint32, optional, tag = "19")]
    pub rarity: Option<u32>,
}
