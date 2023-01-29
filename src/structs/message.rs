use iso8601_timestamp::Timestamp;
use optional_struct::OptionalStruct;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn if_false(t: &bool) -> bool {
    !t
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reply {
    /// Message Id
    pub id: String,
    /// Whether this reply should mention the message's author
    pub mention: bool,
}

#[derive(Validate, Serialize, Deserialize, Clone, Debug)]
pub struct SendableEmbed {
    #[validate(length(min = 1, max = 128))]
    pub icon_url: Option<String>,
    pub url: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 2000))]
    pub description: Option<String>,
    pub media: Option<String>,
    #[validate(length(min = 1, max = 128))]
    pub colour: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SystemMessage {
    #[serde(rename = "text")]
    Text { content: String },
    #[serde(rename = "user_added")]
    UserAdded { id: String, by: String },
    #[serde(rename = "user_remove")]
    UserRemove { id: String, by: String },
    #[serde(rename = "user_joined")]
    UserJoined { id: String },
    #[serde(rename = "user_left")]
    UserLeft { id: String },
    #[serde(rename = "user_kicked")]
    UserKicked { id: String },
    #[serde(rename = "user_banned")]
    UserBanned { id: String },
    #[serde(rename = "channel_renamed")]
    ChannelRenamed { name: String, by: String },
    #[serde(rename = "channel_description_changed")]
    ChannelDescriptionChanged { by: String },
    #[serde(rename = "channel_icon_changed")]
    ChannelIconChanged { by: String },
    #[serde(rename = "channel_ownership_changed")]
    ChannelOwnershipChanged { from: String, to: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Masquerade {
    /// Replace the display name shown on this message
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 32))]
    pub name: Option<String>,
    /// Replace the avatar shown on this message (URL to image file)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 256))]
    pub avatar: Option<String>,
    /// Replace the display role colour shown on this message
    ///
    /// Must have `ManageRole` permission to use
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 128))]
    pub colour: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, OptionalStruct, Default)]
#[optional_derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Message {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Unique value generated by client sending this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// Id of the channel this message was sent in
    pub channel: String,
    /// Id of the user that sent this message
    pub author: String,

    /// Message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// System message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemMessage>,
    /// Time at which this message was last edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited: Option<Timestamp>,
    /// Array of user ids mentioned in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
    /// Array of message ids this message is replying to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<SendableEmbed>>,
    /// Name and / or avatar overrides for this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Masquerade>,
}
impl Message {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(String::from(content));
        self
    }
}

#[derive(Validate, Serialize, Deserialize, Default)]
pub struct DataMessageSend {
    /// Message content to send
    #[validate(length(min = 0, max = 2000))]
    pub content: Option<String>,
    /// Messages to reply to
    pub replies: Option<Vec<Reply>>,
    /// Embeds to include in message
    ///
    /// Text embed content contributes to the content length cap
    #[validate(length(min = 1, max = 10))]
    pub embeds: Option<Vec<SendableEmbed>>,
    /// Masquerade to apply to this message
    #[validate]
    pub masquerade: Option<Masquerade>,
}
impl DataMessageSend {
    pub fn new() -> Self {
        DataMessageSend::default()
    }
    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(String::from(content));
        self
    }
}