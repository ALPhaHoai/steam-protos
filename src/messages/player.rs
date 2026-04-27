//! Player service protobuf messages

use prost::Message;

use crate::enums::ENotificationSetting;

/// Get player link details request
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetPlayerLinkDetailsRequest {
    #[prost(uint64, repeated, packed = "false", tag = "1")]
    pub steamids: Vec<u64>,
}

/// Get friends gameplay info request
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetFriendsGameplayInfoRequest {
    #[prost(uint32, optional, tag = "1")]
    pub appid: Option<u32>,
}

/// Get friends gameplay info response
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetFriendsGameplayInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub your_info: Option<c_player_get_friends_gameplay_info_response::OwnGameplayInfo>,
    #[prost(message, repeated, tag = "2")]
    pub in_game: Vec<c_player_get_friends_gameplay_info_response::FriendsGameplayInfo>,
    #[prost(message, repeated, tag = "3")]
    pub played_recently: Vec<c_player_get_friends_gameplay_info_response::FriendsGameplayInfo>,
    #[prost(message, repeated, tag = "4")]
    pub played_ever: Vec<c_player_get_friends_gameplay_info_response::FriendsGameplayInfo>,
    #[prost(message, repeated, tag = "5")]
    pub owns: Vec<c_player_get_friends_gameplay_info_response::FriendsGameplayInfo>,
    #[prost(message, repeated, tag = "6")]
    pub in_wishlist: Vec<c_player_get_friends_gameplay_info_response::FriendsGameplayInfo>,
}

pub mod c_player_get_friends_gameplay_info_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct FriendsGameplayInfo {
        #[prost(fixed64, optional, tag = "1")]
        pub steamid: Option<u64>,
        #[prost(uint32, optional, tag = "2")]
        pub minutes_played: Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub minutes_played_forever: Option<u32>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct OwnGameplayInfo {
        #[prost(fixed64, optional, tag = "1")]
        pub steamid: Option<u64>,
        #[prost(uint32, optional, tag = "2")]
        pub minutes_played: Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub minutes_played_forever: Option<u32>,
        #[prost(bool, optional, tag = "4")]
        pub in_wishlist: Option<bool>,
        #[prost(bool, optional, tag = "5")]
        pub owned: Option<bool>,
    }
}

/// Get player link details response
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetPlayerLinkDetailsResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: Vec<c_player_get_player_link_details_response::PlayerLinkDetails>,
}

pub mod c_player_get_player_link_details_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct PlayerLinkDetails {
        #[prost(message, optional, tag = "1")]
        pub public_data: Option<AccountPublicData>,
        #[prost(message, optional, tag = "2")]
        pub private_data: Option<AccountPrivateData>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct AccountPublicData {
        #[prost(fixed64, required, tag = "1")]
        pub steamid: u64,
        #[prost(int32, optional, tag = "2")]
        pub visibility_state: Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub privacy_state: Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub profile_state: Option<i32>,
        #[prost(uint32, optional, tag = "7")]
        pub ban_expires_time: Option<u32>,
        #[prost(uint32, optional, tag = "8")]
        pub account_flags: Option<u32>,
        #[prost(bytes = "vec", optional, tag = "9")]
        pub sha_digest_avatar: Option<Vec<u8>>,
        #[prost(string, optional, tag = "10")]
        pub persona_name: Option<String>,
        #[prost(string, optional, tag = "11")]
        pub profile_url: Option<String>,
        #[prost(bool, optional, tag = "12")]
        pub content_country_restricted: Option<bool>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct AccountPrivateData {
        #[prost(int32, optional, tag = "1")]
        pub persona_state: Option<i32>,
        #[prost(uint32, optional, tag = "2")]
        pub persona_state_flags: Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub time_created: Option<u32>,
        #[prost(fixed64, optional, tag = "4")]
        pub game_id: Option<u64>,
        #[prost(fixed64, optional, tag = "5")]
        pub game_server_steam_id: Option<u64>,
        #[prost(uint32, optional, tag = "6")]
        pub game_server_ip_address: Option<u32>,
        #[prost(uint32, optional, tag = "7")]
        pub game_server_port: Option<u32>,
        #[prost(string, optional, tag = "8")]
        pub game_extra_info: Option<String>,
        #[prost(string, optional, tag = "9")]
        pub account_name: Option<String>,
        #[prost(fixed64, optional, tag = "10")]
        pub lobby_steam_id: Option<u64>,
        #[prost(string, optional, tag = "11")]
        pub rich_presence_kv: Option<String>,
        #[prost(uint32, optional, tag = "17")]
        pub last_logoff_time: Option<u32>,
        #[prost(uint32, optional, tag = "18")]
        pub last_seen_online: Option<u32>,
    }
}

/// Get per-friend preferences request
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetPerFriendPreferencesRequest {}

/// Get per-friend preferences response
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetPerFriendPreferencesResponse {
    #[prost(message, repeated, tag = "1")]
    pub preferences: Vec<PerFriendPreferences>,
}

/// Per-friend preferences
#[derive(Clone, PartialEq, Message)]
pub struct PerFriendPreferences {
    #[prost(fixed32, optional, tag = "1")]
    pub accountid: Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub nickname: Option<String>,
    #[prost(enumeration = "ENotificationSetting", optional, tag = "3")]
    pub notifications_showingame: Option<i32>,
    #[prost(enumeration = "ENotificationSetting", optional, tag = "4")]
    pub notifications_showonline: Option<i32>,
    #[prost(enumeration = "ENotificationSetting", optional, tag = "5")]
    pub notifications_showmessages: Option<i32>,
    #[prost(enumeration = "ENotificationSetting", optional, tag = "6")]
    pub sounds_showingame: Option<i32>,
    #[prost(enumeration = "ENotificationSetting", optional, tag = "7")]
    pub sounds_showonline: Option<i32>,
    #[prost(enumeration = "ENotificationSetting", optional, tag = "8")]
    pub sounds_showmessages: Option<i32>,
    #[prost(enumeration = "ENotificationSetting", optional, tag = "9")]
    pub notifications_sendmobile: Option<i32>,
}

/// Set per-friend preferences request
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerSetPerFriendPreferencesRequest {
    #[prost(fixed32, optional, tag = "1")]
    pub accountid: Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub preferences: Option<PerFriendPreferences>,
}

/// Set per-friend preferences response
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerSetPerFriendPreferencesResponse {}

/// Ignore friend request
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerIgnoreFriendRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(bool, optional, tag = "2")]
    pub unignore: Option<bool>,
}

/// Ignore friend response
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerIgnoreFriendResponse {
    #[prost(uint32, optional, tag = "1")]
    pub friend_relationship: Option<u32>,
}

/// Get privacy settings request
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetPrivacySettingsRequest {}

/// Get privacy settings response
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetPrivacySettingsResponse {
    #[prost(message, optional, tag = "1")]
    pub privacy_settings: Option<CPrivacySettings>,
}

/// Privacy settings
#[derive(Clone, PartialEq, Message)]
pub struct CPrivacySettings {
    #[prost(int32, optional, tag = "1")]
    pub privacy_state: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub privacy_state_inventory: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub privacy_state_gifts: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub privacy_state_ownedgames: Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub privacy_state_playtime: Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub privacy_state_friendslist: Option<i32>,
}

/// Friend nickname changed notification
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerFriendNicknameChangedNotification {
    #[prost(fixed32, optional, tag = "1")]
    pub accountid: Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub nickname: Option<String>,
}

/// Get game badge levels request
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetGameBadgeLevelsRequest {
    #[prost(uint32, optional, tag = "1")]
    pub appid: Option<u32>,
}

/// Get game badge levels response
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetGameBadgeLevelsResponse {
    #[prost(uint32, optional, tag = "1")]
    pub player_level: Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub badges: Vec<c_player_get_game_badge_levels_response::Badge>,
}

pub mod c_player_get_game_badge_levels_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct Badge {
        #[prost(int32, optional, tag = "1")]
        pub level: Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub series: Option<i32>,
        #[prost(uint32, optional, tag = "3")]
        pub border_color: Option<u32>,
    }
}
