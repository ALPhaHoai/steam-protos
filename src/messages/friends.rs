//! Friends-related protobuf messages

use prost::Message;

/// Friends list message
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientFriendsList {
    #[prost(bool, optional, tag = "1")]
    pub bincremental: Option<bool>,
    #[prost(message, repeated, tag = "2")]
    pub friends: Vec<cmsg_client_friends_list::Friend>,
    #[prost(uint32, optional, tag = "3")]
    pub max_friend_count: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub active_friend_count: Option<u32>,
    #[prost(bool, optional, tag = "5")]
    pub friends_limit_hit: Option<bool>,
}

pub mod cmsg_client_friends_list {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct Friend {
        #[prost(fixed64, optional, tag = "1")]
        pub ulfriendid: Option<u64>,
        #[prost(uint32, optional, tag = "2")]
        pub efriendrelationship: Option<u32>,
    }
}

/// Persona state message
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientPersonaState {
    #[prost(uint32, optional, tag = "1")]
    pub status_flags: Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub friends: Vec<cmsg_client_persona_state::Friend>,
}

pub mod cmsg_client_persona_state {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct Friend {
        #[prost(fixed64, optional, tag = "1")]
        pub friendid: Option<u64>,
        #[prost(uint32, optional, tag = "2")]
        pub persona_state: Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub game_played_app_id: Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub game_server_ip: Option<u32>,
        #[prost(uint32, optional, tag = "5")]
        pub game_server_port: Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub persona_state_flags: Option<u32>,
        #[prost(uint32, optional, tag = "7")]
        pub online_session_instances: Option<u32>,
        #[prost(bool, optional, tag = "10")]
        pub persona_set_by_user: Option<bool>,
        #[prost(string, optional, tag = "15")]
        pub player_name: Option<String>,
        #[prost(uint32, optional, tag = "20")]
        pub query_port: Option<u32>,
        #[prost(fixed64, optional, tag = "25")]
        pub steamid_source: Option<u64>,
        #[prost(bytes = "vec", optional, tag = "31")]
        pub avatar_hash: Option<Vec<u8>>,
        #[prost(uint32, optional, tag = "45")]
        pub last_logoff: Option<u32>,
        #[prost(uint32, optional, tag = "46")]
        pub last_logon: Option<u32>,
        #[prost(uint32, optional, tag = "47")]
        pub last_seen_online: Option<u32>,
        #[prost(uint32, optional, tag = "50")]
        pub clan_rank: Option<u32>,
        #[prost(string, optional, tag = "55")]
        pub game_name: Option<String>,
        #[prost(fixed64, optional, tag = "56")]
        pub gameid: Option<u64>,
        #[prost(bytes = "vec", optional, tag = "60")]
        pub game_data_blob: Option<Vec<u8>>,
        #[prost(message, optional, tag = "64")]
        pub clan_data: Option<ClanData>,
        #[prost(string, optional, tag = "65")]
        pub clan_tag: Option<String>,
        #[prost(message, repeated, tag = "71")]
        pub rich_presence: Vec<Kv>,
        #[prost(fixed64, optional, tag = "72")]
        pub broadcast_id: Option<u64>,
        #[prost(fixed64, optional, tag = "73")]
        pub game_lobby_id: Option<u64>,
        #[prost(uint32, optional, tag = "74")]
        pub watching_broadcast_accountid: Option<u32>,
        #[prost(uint32, optional, tag = "75")]
        pub watching_broadcast_appid: Option<u32>,
        #[prost(uint32, optional, tag = "76")]
        pub watching_broadcast_viewers: Option<u32>,
        #[prost(string, optional, tag = "77")]
        pub watching_broadcast_title: Option<String>,
        #[prost(bool, optional, tag = "78")]
        pub is_community_banned: Option<bool>,
        #[prost(bool, optional, tag = "79")]
        pub player_name_pending_review: Option<bool>,
        #[prost(bool, optional, tag = "80")]
        pub avatar_pending_review: Option<bool>,
        #[prost(bool, optional, tag = "81")]
        pub on_steam_deck: Option<bool>,
        #[prost(uint32, optional, tag = "83")]
        pub gaming_device_type: Option<u32>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct ClanData {
        #[prost(uint32, optional, tag = "1")]
        pub ogg_app_id: Option<u32>,
        #[prost(uint64, optional, tag = "2")]
        pub chat_group_id: Option<u64>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct Kv {
        #[prost(string, optional, tag = "1")]
        pub key: Option<String>,
        #[prost(string, optional, tag = "2")]
        pub value: Option<String>,
    }
}

/// Change status request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientChangeStatus {
    #[prost(uint32, optional, tag = "1")]
    pub persona_state: Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub player_name: Option<String>,
    #[prost(bool, optional, tag = "3")]
    pub is_auto_generated_name: Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub high_priority: Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub persona_set_by_user: Option<bool>,
    #[prost(uint32, optional, tag = "6")]
    pub persona_state_flags: Option<u32>,
    #[prost(bool, optional, tag = "7")]
    pub need_persona_response: Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub is_client_idle: Option<bool>,
}

/// Add friend request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientAddFriend {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid_to_add: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub accountname_or_email_to_add: Option<String>,
}

/// Add friend response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientAddFriendResponse {
    #[prost(int32, optional, tag = "1")]
    pub eresult: Option<i32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id_added: Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub persona_name_added: Option<String>,
}

/// Remove friend request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRemoveFriend {
    #[prost(fixed64, optional, tag = "1")]
    pub friendid: Option<u64>,
}

/// Request friend data
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRequestFriendData {
    #[prost(uint32, optional, tag = "1")]
    pub persona_state_requested: Option<u32>,
    #[prost(fixed64, repeated, packed = "false", tag = "2")]
    pub friends: Vec<u64>,
}

/// Create friends group request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientCreateFriendsGroup {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub groupname: Option<String>,
    #[prost(fixed64, repeated, packed = "false", tag = "3")]
    pub steamid_friends: Vec<u64>,
}

/// Create friends group response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientCreateFriendsGroupResponse {
    #[prost(uint32, optional, tag = "1")]
    pub eresult: Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub groupid: Option<i32>,
}

/// Delete friends group request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientDeleteFriendsGroup {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(int32, optional, tag = "2")]
    pub groupid: Option<i32>,
}

/// Delete friends group response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientDeleteFriendsGroupResponse {
    #[prost(uint32, optional, tag = "1")]
    pub eresult: Option<u32>,
}

/// Rename friends group request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRenameFriendsGroup {
    #[prost(int32, optional, tag = "1")]
    pub groupid: Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub groupname: Option<String>,
}

/// Add friend to group request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientAddFriendToGroup {
    #[prost(int32, optional, tag = "1")]
    pub groupid: Option<i32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamiduser: Option<u64>,
}

/// Add friend to group response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientAddFriendToGroupResponse {
    #[prost(uint32, optional, tag = "1")]
    pub eresult: Option<u32>,
}

/// Remove friend from group request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRemoveFriendFromGroup {
    #[prost(int32, optional, tag = "1")]
    pub groupid: Option<i32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamiduser: Option<u64>,
}

/// Remove friend from group response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRemoveFriendFromGroupResponse {
    #[prost(uint32, optional, tag = "1")]
    pub eresult: Option<u32>,
}

/// Invite to game request
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientInviteToGame {
    #[prost(fixed64, optional, tag = "1")]
    pub steam_id_dest: Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id_src: Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub connect_string: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub remote_play: Option<String>,
}

/// Set player nickname request (EMsg = AMClientSetPlayerNickname)
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientSetPlayerNickname {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub nickname: Option<String>,
}

/// Set player nickname response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientSetPlayerNicknameResponse {
    #[prost(uint32, optional, tag = "1")]
    pub eresult: Option<u32>,
}

/// Get friends steam levels request (EMsg = ClientFSGetFriendsSteamLevels)
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientFSGetFriendsSteamLevels {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub accountids: Vec<u32>,
}

/// Get friends steam levels response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientFSGetFriendsSteamLevelsResponse {
    #[prost(message, repeated, tag = "1")]
    pub friends: Vec<cmsg_client_fs_get_friends_steam_levels_response::Friend>,
}

pub mod cmsg_client_fs_get_friends_steam_levels_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct Friend {
        #[prost(uint32, optional, tag = "1")]
        pub accountid: Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub level: Option<u32>,
    }
}

/// Get nickname list request (Player.GetNicknameList#1)
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetNicknameListRequest {}

/// Get nickname list response
#[derive(Clone, PartialEq, Message)]
pub struct CPlayerGetNicknameListResponse {
    #[prost(message, repeated, tag = "1")]
    pub nicknames: Vec<c_player_get_nickname_list_response::PlayerNickname>,
}

pub mod c_player_get_nickname_list_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct PlayerNickname {
        #[prost(uint32, optional, tag = "1")]
        pub accountid: Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub nickname: Option<String>,
    }
}

/// Get persona name history request (EMsg = ClientAMGetPersonaNameHistory)
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientAMGetPersonaNameHistory {
    #[prost(int32, optional, tag = "1")]
    pub id_count: Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub ids: Vec<cmsg_client_am_get_persona_name_history::IdInstance>,
}

pub mod cmsg_client_am_get_persona_name_history {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct IdInstance {
        #[prost(fixed64, optional, tag = "1")]
        pub steamid: Option<u64>,
    }
}

/// Get persona name history response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientAMGetPersonaNameHistoryResponse {
    #[prost(message, repeated, tag = "2")]
    pub responses: Vec<cmsg_client_am_get_persona_name_history_response::NameTableInstance>,
}

pub mod cmsg_client_am_get_persona_name_history_response {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct NameTableInstance {
        #[prost(int32, optional, tag = "1")]
        pub eresult: Option<i32>,
        #[prost(fixed64, optional, tag = "2")]
        pub steamid: Option<u64>,
        #[prost(message, repeated, tag = "3")]
        pub names: Vec<NameInstance>,
    }

    #[derive(Clone, PartialEq, Message)]
    pub struct NameInstance {
        #[prost(string, optional, tag = "1")]
        pub name: Option<String>,
        #[prost(uint32, optional, tag = "2")]
        pub name_since: Option<u32>,
    }
}

// ============================================================================
// Quick Invite Link Messages (UserAccount service)
// ============================================================================

/// Create friend invite token request (UserAccount.CreateFriendInviteToken#1)
#[derive(Clone, PartialEq, Message)]
pub struct CUserAccountCreateFriendInviteTokenRequest {
    /// Maximum number of times this invite can be used (0 = unlimited)
    #[prost(uint32, optional, tag = "1")]
    pub invite_limit: Option<u32>,
    /// Duration in seconds that the invite is valid (0 = no expiry)
    #[prost(uint32, optional, tag = "2")]
    pub invite_duration: Option<u32>,
    /// Optional note for this invite
    #[prost(string, optional, tag = "3")]
    pub invite_note: Option<String>,
}

/// Create friend invite token response
#[derive(Clone, PartialEq, Message)]
pub struct CUserAccountCreateFriendInviteTokenResponse {
    /// The invite token string
    #[prost(string, optional, tag = "1")]
    pub invite_token: Option<String>,
    /// Maximum uses for this invite
    #[prost(uint64, optional, tag = "2")]
    pub invite_limit: Option<u64>,
    /// Duration in seconds
    #[prost(uint64, optional, tag = "3")]
    pub invite_duration: Option<u64>,
    /// Unix timestamp when created
    #[prost(fixed32, optional, tag = "4")]
    pub time_created: Option<u32>,
    /// Whether the token is currently valid
    #[prost(bool, optional, tag = "5")]
    pub valid: Option<bool>,
}

/// Get friend invite tokens request (UserAccount.GetFriendInviteTokens#1)
#[derive(Clone, PartialEq, Message)]
pub struct CUserAccountGetFriendInviteTokensRequest {}

/// Get friend invite tokens response
#[derive(Clone, PartialEq, Message)]
pub struct CUserAccountGetFriendInviteTokensResponse {
    /// List of all invite tokens for this account
    #[prost(message, repeated, tag = "1")]
    pub tokens: Vec<CUserAccountCreateFriendInviteTokenResponse>,
}

/// View friend invite token request (UserAccount.ViewFriendInviteToken#1)
#[derive(Clone, PartialEq, Message)]
pub struct CUserAccountViewFriendInviteTokenRequest {
    /// SteamID of the invite link owner
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    /// The invite token to check
    #[prost(string, optional, tag = "2")]
    pub invite_token: Option<String>,
}

/// View friend invite token response
#[derive(Clone, PartialEq, Message)]
pub struct CUserAccountViewFriendInviteTokenResponse {
    /// Whether the invite is valid
    #[prost(bool, optional, tag = "1")]
    pub valid: Option<bool>,
    /// SteamID of the invite owner
    #[prost(uint64, optional, tag = "2")]
    pub steamid: Option<u64>,
    /// Duration in seconds
    #[prost(uint64, optional, tag = "3")]
    pub invite_duration: Option<u64>,
}

/// Redeem friend invite token request (UserAccount.RedeemFriendInviteToken#1)
#[derive(Clone, PartialEq, Message)]
pub struct CUserAccountRedeemFriendInviteTokenRequest {
    /// SteamID of the invite link owner
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    /// The invite token to redeem
    #[prost(string, optional, tag = "2")]
    pub invite_token: Option<String>,
}

/// Redeem friend invite token response
#[derive(Clone, PartialEq, Message)]
pub struct CUserAccountRedeemFriendInviteTokenResponse {}

/// Revoke friend invite token request (UserAccount.RevokeFriendInviteToken#1)
#[derive(Clone, PartialEq, Message)]
pub struct CUserAccountRevokeFriendInviteTokenRequest {
    /// The invite token to revoke
    #[prost(string, optional, tag = "1")]
    pub invite_token: Option<String>,
}

/// Revoke friend invite token response
#[derive(Clone, PartialEq, Message)]
pub struct CUserAccountRevokeFriendInviteTokenResponse {}

// ============================================================================
// Unified Friends List Messages (FriendsList service)
// ============================================================================

/// Get friends list request (FriendsList.GetFriendsList#1)
#[derive(Clone, PartialEq, Message)]
pub struct CFriendsListGetFriendsListRequest {
    #[prost(uint32, optional, tag = "1")]
    pub role_mask: Option<u32>,
}

/// Get friends list response
#[derive(Clone, PartialEq, Message)]
pub struct CFriendsListGetFriendsListResponse {
    #[prost(message, optional, tag = "1")]
    pub friendslist: Option<CMsgClientFriendsList>,
}
