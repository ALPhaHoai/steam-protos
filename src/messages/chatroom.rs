//! Chat room protobuf messages

use prost::Message;

/// Create chat room group request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomCreateChatRoomGroupRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid_partner: Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamid_invited: Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub name: Option<String>,
    #[prost(fixed64, repeated, tag = "4")]
    pub steamid_invitees: Vec<u64>,
    #[prost(uint32, optional, tag = "6")]
    pub watching_broadcast_accountid: Option<u32>,
    #[prost(uint64, optional, tag = "7")]
    pub watching_broadcast_channel_id: Option<u64>,
}

/// Save chat room group request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomSaveChatRoomGroupRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
}

/// Get my chat room groups request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomGetMyChatRoomGroupsRequest {}

/// Set session active chat room groups request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomSetSessionActiveChatRoomGroupsRequest {
    #[prost(uint64, repeated, tag = "1")]
    pub chat_group_ids: Vec<u64>,
    #[prost(uint64, repeated, tag = "2")]
    pub chat_groups_data_requested: Vec<u64>,
    #[prost(int32, optional, tag = "3")]
    pub virtualize_members_threshold: Option<i32>,
}

/// Get invite link info request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomGetInviteLinkInfoRequest {
    #[prost(string, optional, tag = "1")]
    pub invite_code: Option<String>,
}

/// Join chat room group request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomJoinChatRoomGroupRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub invite_code: Option<String>,
    #[prost(uint64, optional, tag = "3")]
    pub chat_id: Option<u64>,
}

/// Leave chat room group request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomLeaveChatRoomGroupRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
}

/// Create invite link request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomCreateInviteLinkRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub seconds_valid: Option<u32>,
    #[prost(uint64, optional, tag = "3")]
    pub chat_id: Option<u64>,
}

/// Create invite link response
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomCreateInviteLinkResponse {
    #[prost(string, optional, tag = "1")]
    pub invite_code: Option<String>,
    #[prost(uint32, optional, tag = "2")]
    pub seconds_valid: Option<u32>,
}

/// Get invite links for group request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomGetInviteLinksForGroupRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
}

/// Get invite links for group response
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomGetInviteLinksForGroupResponse {
    #[prost(message, repeated, tag = "1")]
    pub invite_links: Vec<CChatRoomInviteLink>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomInviteLink {
    #[prost(string, optional, tag = "1")]
    pub invite_code: Option<String>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamid_creator: Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub time_expires: Option<u32>,
    #[prost(uint64, optional, tag = "4")]
    pub chat_id: Option<u64>,
}

/// Delete invite link request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomDeleteInviteLinkRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub invite_code: Option<String>,
}

/// Delete invite link response
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomDeleteInviteLinkResponse {}

/// Send chat message request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomSendChatMessageRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub chat_id: Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub message: Option<String>,
    #[prost(bool, optional, tag = "4")]
    pub echo_to_sender: Option<bool>,
}

/// Get message history request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomGetMessageHistoryRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub chat_id: Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub last_time: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub last_ordinal: Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub start_time: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub start_ordinal: Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub max_count: Option<u32>,
}

/// Create chat room request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomCreateChatRoomRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
    #[prost(bool, optional, tag = "3")]
    pub allow_voice: Option<bool>,
}

/// Create chat room response
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomCreateChatRoomResponse {
    #[prost(message, optional, tag = "1")]
    pub chat_room: Option<CChatRoomState>,
}

/// Rename chat room request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomRenameChatRoomRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub chat_id: Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub name: Option<String>,
}

/// Rename chat room response
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomRenameChatRoomResponse {}

/// Delete chat room request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomDeleteChatRoomRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub chat_id: Option<u64>,
}

/// Delete chat room response
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomDeleteChatRoomResponse {}

/// Get clan chat room info request
#[derive(Clone, PartialEq, Message)]
pub struct CClanChatRoomsGetClanChatRoomInfoRequest {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(bool, optional, tag = "2")]
    pub autocreate: Option<bool>,
}

/// Get clan chat room info response
#[derive(Clone, PartialEq, Message)]
pub struct CClanChatRoomsGetClanChatRoomInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub chat_group_summary: Option<CChatRoomGroupSummary>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomGroupSummary {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub chat_group_name: Option<String>,
    #[prost(uint32, optional, tag = "3")]
    pub active_member_count: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub active_voice_member_count: Option<u32>,
    #[prost(uint64, optional, tag = "5")]
    pub default_chat_id: Option<u64>,
    #[prost(message, repeated, tag = "6")]
    pub chat_rooms: Vec<CChatRoomState>,
    #[prost(fixed64, optional, tag = "7")]
    pub steamid_owner: Option<u64>,
    #[prost(uint32, repeated, tag = "8")]
    pub top_members: Vec<u32>,
    #[prost(string, optional, tag = "9")]
    pub chat_group_tagline: Option<String>,
    #[prost(uint32, optional, tag = "10")]
    pub appid: Option<u32>,
    #[prost(bytes = "vec", optional, tag = "11")]
    pub chat_group_avatar_sha: Option<Vec<u8>>,
    #[prost(uint64, optional, tag = "12")]
    pub watching_broadcast_channel_id: Option<u64>,
    #[prost(uint32, optional, tag = "13")]
    pub watching_broadcast_accountid: Option<u32>,
}

/// Kick user request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomKickUserRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamid: Option<u64>,
    #[prost(int32, optional, tag = "3")]
    pub expiration: Option<i32>,
}

/// Set user ban state request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomSetUserBanStateRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamid: Option<u64>,
    #[prost(bool, optional, tag = "3")]
    pub ban_state: Option<bool>,
}

/// Ack chat message notification
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomAckChatMessageNotification {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub chat_id: Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub timestamp: Option<u32>,
}

/// Delete chat messages request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomDeleteChatMessagesRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub chat_id: Option<u64>,
    #[prost(message, repeated, tag = "3")]
    pub messages: Vec<CChatRoomMessage>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomMessage {
    #[prost(uint32, optional, tag = "1")]
    pub server_timestamp: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub ordinal: Option<u32>,
}

/// Delete chat messages response
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomDeleteChatMessagesResponse {}

/// Get ban list request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomGetBanListRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
}

/// Get ban list response
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomGetBanListResponse {
    #[prost(message, repeated, tag = "1")]
    pub bans: Vec<CChatRoomBan>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomBan {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamid_actor: Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub time_banned: Option<u32>,
    #[prost(string, optional, tag = "4")]
    pub ban_reason: Option<String>,
}

/// Add role to user request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomAddRoleToUserRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub role_id: Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub steamid: Option<u64>,
}

/// Add role to user response
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomAddRoleToUserResponse {}

/// Delete role from user request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomDeleteRoleFromUserRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub role_id: Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub steamid: Option<u64>,
}

/// Delete role from user response
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomDeleteRoleFromUserResponse {}

/// Invite friend to chat room group request
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomInviteFriendToChatRoomGroupRequest {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamid: Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub chat_id: Option<u64>,
    #[prost(bool, optional, tag = "4")]
    pub skip_friendsui_check: Option<bool>,
}

/// Incoming chat message notification
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomIncomingChatMessageNotification {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub chat_id: Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub steamid_sender: Option<u64>,
    #[prost(string, optional, tag = "4")]
    pub message: Option<String>,
    #[prost(uint32, optional, tag = "5")]
    pub timestamp: Option<u32>,
    #[prost(message, optional, tag = "6")]
    pub mentions: Option<CChatMentions>,
    #[prost(uint32, optional, tag = "7")]
    pub ordinal: Option<u32>,
    #[prost(message, optional, tag = "8")]
    pub server_message: Option<ServerMessage>,
    #[prost(string, optional, tag = "9")]
    pub message_no_bbcode: Option<String>,
    #[prost(string, optional, tag = "10")]
    pub chat_name: Option<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CChatMentions {
    #[prost(bool, optional, tag = "1")]
    pub mention_all: Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub mention_here: Option<bool>,
    #[prost(uint32, repeated, tag = "3")]
    pub mention_accountids: Vec<u32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ServerMessage {
    #[prost(int32, optional, tag = "1")]
    pub message: Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub string_param: Option<String>,
    #[prost(uint32, optional, tag = "3")]
    pub accountid_param: Option<u32>,
}

/// Member state change notification
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomMemberStateChangeNotification {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(message, optional, tag = "2")]
    pub member: Option<CChatRoomMember>,
    #[prost(int32, optional, tag = "3")]
    pub change: Option<i32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomMember {
    #[prost(uint32, optional, tag = "1")]
    pub accountid: Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub state: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub rank: Option<i32>,
    #[prost(uint32, optional, tag = "6")]
    pub time_kick_expire: Option<u32>,
    #[prost(uint64, repeated, tag = "7")]
    pub role_ids: Vec<u64>,
}

/// Chat room group rooms change notification
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomChatRoomGroupRoomsChangeNotification {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub default_chat_id: Option<u64>,
    #[prost(message, repeated, tag = "3")]
    pub chat_rooms: Vec<CChatRoomState>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomState {
    #[prost(uint64, optional, tag = "1")]
    pub chat_id: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub chat_name: Option<String>,
    #[prost(bool, optional, tag = "3")]
    pub voice_allowed: Option<bool>,
    #[prost(uint32, repeated, tag = "4")]
    pub members_in_voice: Vec<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub time_last_message: Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub sort_order: Option<u32>,
    #[prost(string, optional, tag = "7")]
    pub last_message: Option<String>,
    #[prost(uint32, optional, tag = "8")]
    pub accountid_last_message: Option<u32>,
}

/// Chat message modified notification
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomChatMessageModifiedNotification {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub chat_id: Option<u64>,
    #[prost(message, repeated, tag = "3")]
    pub messages: Vec<c_chat_room_chat_message_modified_notification::ChatMessage>,
}

pub mod c_chat_room_chat_message_modified_notification {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct ChatMessage {
        #[prost(uint32, optional, tag = "1")]
        pub server_timestamp: Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub ordinal: Option<u32>,
        #[prost(bool, optional, tag = "3")]
        pub deleted: Option<bool>,
    }
}

/// Chat room header state notification
#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomChatRoomHeaderStateNotification {
    #[prost(message, optional, tag = "1")]
    pub header_state: Option<CChatRoomGroupHeaderState>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CChatRoomGroupHeaderState {
    #[prost(uint64, optional, tag = "1")]
    pub chat_group_id: Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub chat_name: Option<String>,
    #[prost(fixed64, optional, tag = "3")]
    pub steamid_owner: Option<u64>,
    #[prost(uint32, optional, tag = "4")]
    pub appid: Option<u32>,
    #[prost(fixed64, optional, tag = "5")]
    pub steamid_clan: Option<u64>,
    #[prost(uint32, optional, tag = "6")]
    pub chat_group_tagline: Option<u32>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub avatar_sha: Option<Vec<u8>>,
    #[prost(uint32, optional, tag = "8")]
    pub default_chat_id: Option<u32>,
}
