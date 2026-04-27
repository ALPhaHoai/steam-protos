//! CS:GO specific protobuf messages

use prost::Message;

/// k_EMsgGCCStrike15_v2_Party_Search = 9191
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGccStrike15V2PartySearch {
    #[prost(uint32, optional, tag = "1")]
    pub ver: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub apr: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub ark: Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub grps: Vec<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub launcher: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub game_type: Option<u32>,
}

/// k_EMsgGCCStrike15_v2_Party_SearchResults
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGccStrike15V2PartySearchResults {
    #[prost(message, repeated, tag = "1")]
    pub entries: Vec<c_msg_gcc_strike15_v2_party_search_results::Entry>,
}

pub mod c_msg_gcc_strike15_v2_party_search_results {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct Entry {
        #[prost(uint32, optional, tag = "1")]
        pub id: Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub grp: Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub game_type: Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub apr: Option<u32>,
        #[prost(uint32, optional, tag = "5")]
        pub ark: Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub loc: Option<u32>,
        #[prost(uint32, optional, tag = "7")]
        pub accountid: Option<u32>,
    }
}

/// k_EMsgGCCStrike15_v2_Party_Invite = 9192
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGccStrike15V2PartyInvite {
    #[prost(uint32, optional, tag = "1")]
    pub accountid: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub lobbyid: Option<u32>,
}

/// k_EMsgGCCStrike15_v2_Party_Register = 9189
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGccStrike15V2PartyRegister {
    #[prost(uint32, optional, tag = "1")]
    pub id: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub ver: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub apr: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub ark: Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub nby: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub grp: Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub slots: Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub launcher: Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub game_type: Option<u32>,
}

/// k_EMsgGCCStrike15_v2_ClientRequestJoinFriendData = 9163
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGccStrike15V2ClientRequestJoinFriendData {
    #[prost(uint32, optional, tag = "1")]
    pub version: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub account_id: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub join_token: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub join_ipp: Option<u32>,
    // Missing MatchmakingGC2ClientReserve definition here if I need it,
    // but maybe I can import or redefine if it's complex.
    // For now, skipping complex nested struct if not immediate priority.
    // Actually, `CMsgGccStrike15V2MatchmakingGc2ClientReserve` is likely defined in `gc.rs` or I need to define it.
    // I'll check `gc.rs` again.
    #[prost(string, optional, tag = "6")]
    pub errormsg: Option<String>,
}

// It was not in gc.rs visible lines. I'll check if needed.

/// k_EMsgGCCStrike15_v2_AcknowledgePenalty = 9171
#[derive(Clone, PartialEq, Message)]
pub struct CMsgGccStrike15V2AcknowledgePenalty {
    #[prost(int32, optional, tag = "1")]
    pub acknowledged: Option<i32>,
}
