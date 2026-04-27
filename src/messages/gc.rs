//! Game Coordinator protobuf messages

use prost::Message;

/// Game Coordinator client message
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGCClient {
    #[prost(uint32, optional, tag = "1")]
    pub appid: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub msgtype: Option<u32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub payload: Option<Vec<u8>>,
    #[prost(fixed64, optional, tag = "4")]
    pub steamid: Option<u64>,
    #[prost(string, optional, tag = "5")]
    pub gcname: Option<String>,
    #[prost(uint32, optional, tag = "6")]
    pub ip: Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub flags: Option<u32>,
}

pub mod cmsg_gc_client {
    /// GC Client flags
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, prost::Enumeration)]
    #[repr(i32)]
    pub enum EFlag {
        ValveDs = 1,
    }
}

/// GC Client Hello (k_EMsgGCClientHello)
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGCClientHello {
    #[prost(uint32, optional, tag = "1")]
    pub version: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub client_session_need: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub client_launcher: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub steam_launcher: Option<u32>,
}

/// GC Client Welcome message (k_EMsgGCClientWelcome)
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientWelcome {
    #[prost(uint32, optional, tag = "1")]
    pub version: Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub game_data: Option<Vec<u8>>,
    #[prost(message, repeated, tag = "3")]
    pub outofdate_subscribed_caches: Vec<CMsgClientWelcomeOutOfDateSubscribedCache>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientWelcomeOutOfDateSubscribedCache {
    #[prost(message, repeated, tag = "2")]
    pub objects: Vec<CMsgClientWelcomeOutOfDateSubscribedCacheObject>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientWelcomeOutOfDateSubscribedCacheObject {
    #[prost(uint32, optional, tag = "1")]
    pub type_id: Option<u32>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub object_data: Vec<Vec<u8>>,
}

/// CS:GO Econ Game Account Client
#[derive(Clone, PartialEq, Message)]
pub struct CSOEconGameAccountClient {
    #[prost(uint32, optional, tag = "13")]
    pub bonus_xp_usedflags: Option<u32>,
    #[prost(uint32, optional, tag = "14")]
    pub elevated_state: Option<u32>,
}

/// k_EMsgGCCStrike15_v2_MatchmakingClient2GCHello = 9109
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGccStrike15V2MatchmakingClient2GcHello {}

//=============================================================================
// CS:GO Client Hello Response (k_EMsgGCCStrike15_v2_MatchmakingGC2ClientHello =
// 9110)
//=============================================================================

/// Player ranking information
#[derive(Clone, PartialEq, Message)]
pub struct PlayerRankingInfo {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub rank_id: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub wins: Option<u32>,
    #[prost(float, optional, tag = "4")]
    pub rank_change: Option<f32>,
    #[prost(uint32, optional, tag = "6")]
    pub rank_type_id: Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub tv_control: Option<u32>,
}

/// Player commendation information
#[derive(Clone, PartialEq, Message)]
pub struct PlayerCommendationInfo {
    #[prost(uint32, optional, tag = "1")]
    pub cmd_friendly: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub cmd_teaching: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub cmd_leader: Option<u32>,
}

/// Player medals information
#[derive(Clone, PartialEq, Message)]
pub struct PlayerMedalsInfo {
    #[prost(uint32, repeated, tag = "7")]
    pub display_items_defidx: Vec<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub featured_display_item_defidx: Option<u32>,
}

/// Global CS:GO statistics
#[derive(Clone, PartialEq, Message)]
pub struct GlobalStatistics {
    #[prost(uint32, optional, tag = "1")]
    pub players_online: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub servers_online: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub players_searching: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub servers_available: Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub ongoing_matches: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub search_time_avg: Option<u32>,
    #[prost(string, optional, tag = "8")]
    pub main_post_url: Option<String>,
    #[prost(uint32, optional, tag = "9")]
    pub required_appid_version: Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub pricesheet_version: Option<u32>,
}

/// k_EMsgGCCStrike15_v2_MatchmakingGC2ClientHello = 9110
/// Response from CS:GO GC with player profile data
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGccStrike15V2MatchmakingGc2ClientHello {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: Option<u32>,
    #[prost(message, optional, tag = "3")]
    pub global_stats: Option<GlobalStatistics>,
    #[prost(uint32, optional, tag = "4")]
    pub penalty_seconds: Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub penalty_reason: Option<u32>,
    #[prost(int32, optional, tag = "6")]
    pub vac_banned: Option<i32>,
    #[prost(message, optional, tag = "7")]
    pub ranking: Option<PlayerRankingInfo>,
    #[prost(message, optional, tag = "8")]
    pub commendation: Option<PlayerCommendationInfo>,
    #[prost(message, optional, tag = "9")]
    pub medals: Option<PlayerMedalsInfo>,
    #[prost(uint32, optional, tag = "14")]
    pub survey_vote: Option<u32>,
    #[prost(int32, optional, tag = "17")]
    pub player_level: Option<i32>,
    #[prost(int32, optional, tag = "18")]
    pub player_cur_xp: Option<i32>,
    #[prost(int32, optional, tag = "19")]
    pub player_xp_bonus_flags: Option<i32>,
    #[prost(message, repeated, tag = "20")]
    pub rankings: Vec<PlayerRankingInfo>,
}

/// k_EMsgGCCStrike15_v2_ClientRequestPlayersProfile = 9127
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGccStrike15V2ClientRequestPlayersProfile {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub request_level: Option<u32>,
}

/// k_EMsgGCCStrike15_v2_PlayersProfile = 9128
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGccStrike15V2PlayersProfile {
    #[prost(message, repeated, tag = "1")]
    pub account_profiles: Vec<CMsgGccStrike15V2MatchmakingGc2ClientHello>,
}
