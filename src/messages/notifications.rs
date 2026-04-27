//! Notification-related protobuf messages

use prost::Message;

/// Request item announcements
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRequestItemAnnouncements {}

/// Item announcements response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientItemAnnouncements {
    #[prost(uint32, optional, tag = "1")]
    pub count_new_items: Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub unseen_items: Vec<cmsg_client_item_announcements::UnseenItem>,
}

pub mod cmsg_client_item_announcements {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct UnseenItem {
        #[prost(uint32, optional, tag = "1")]
        pub appid: Option<u32>,
        #[prost(uint64, optional, tag = "2")]
        pub context_id: Option<u64>,
        #[prost(uint64, optional, tag = "3")]
        pub asset_id: Option<u64>,
        #[prost(uint64, optional, tag = "4")]
        pub amount: Option<u64>,
        #[prost(fixed32, optional, tag = "5")]
        pub rtime32_gained: Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub source_appid: Option<u32>,
    }
}

/// Request comment notifications
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRequestCommentNotifications {}

/// Comment notifications response
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientCommentNotifications {
    #[prost(uint32, optional, tag = "1")]
    pub count_new_comments: Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub count_new_comments_owner: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub count_new_comments_subscriptions: Option<u32>,
}

/// Request offline message count (also known as
/// CMsgClientFSRequestOfflineMessageCount)
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientRequestOfflineMessageCount {}

/// Offline message notification
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientOfflineMessageNotification {
    #[prost(uint32, optional, tag = "1")]
    pub offline_messages: Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub friends_with_offline_messages: Vec<u32>,
}

/// User notifications
#[derive(Clone, PartialEq, Message)]
pub struct CMsgClientUserNotifications {
    #[prost(message, repeated, tag = "1")]
    pub notifications: Vec<cmsg_client_user_notifications::Notification>,
}

pub mod cmsg_client_user_notifications {
    use prost::Message;

    #[derive(Clone, PartialEq, Message)]
    pub struct Notification {
        #[prost(uint32, optional, tag = "1")]
        pub user_notification_type: Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub count: Option<u32>,
    }
}

/// Steam notification mark as read notification
///
/// From `service_steamnotification.proto` (webui):
/// - Used with `SteamNotification.MarkNotificationsRead#1` unified message
#[derive(Clone, PartialEq, Message)]
pub struct CSteamNotificationMarkNotificationsReadNotification {
    /// Timestamp for marking notifications read
    #[prost(uint32, optional, tag = "1")]
    pub timestamp: Option<u32>,
    /// Type of notification to mark as read (ESteamNotificationType)
    #[prost(int32, optional, tag = "2")]
    pub notification_type: Option<i32>,
    /// Specific notification IDs to mark as read
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub notification_ids: Vec<u64>,
    /// If true, mark all notifications as read
    #[prost(bool, optional, tag = "4")]
    pub mark_all_read: Option<bool>,
}

#[derive(Clone, PartialEq, Message)]
pub struct SteamNotificationData {
    #[prost(uint64, optional, tag = "1")]
    pub notification_id: Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub notification_targets: Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub notification_type: Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub body_data: Option<String>,
    #[prost(bool, optional, tag = "7")]
    pub read: Option<bool>,
    #[prost(uint32, optional, tag = "8")]
    pub timestamp: Option<u32>,
    #[prost(bool, optional, tag = "9")]
    pub hidden: Option<bool>,
    #[prost(uint32, optional, tag = "10")]
    pub expiry: Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub viewed: Option<u32>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CSteamNotificationNotificationsReceivedNotification {
    #[prost(message, repeated, tag = "1")]
    pub notifications: Vec<SteamNotificationData>,
    #[prost(uint32, optional, tag = "2")]
    pub pending_gift_count: Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub pending_friend_count: Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub pending_family_invite_count: Option<u32>,
}
