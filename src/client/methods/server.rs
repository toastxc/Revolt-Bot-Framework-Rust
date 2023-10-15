use reywen_http::{driver::Method, results::DeltaError};
use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    json,
    structures::{
        channels::{invite::Invite, Channel},
        media::attachment::File,
        server::{ban::ServerBan, Category, FieldsServer, Server, SystemMessageChannels},
        users::User,
    },
};

use super::opt_vec_add;

impl Client {
    pub async fn server_ack(&self, server: &str) -> Result<(), DeltaError> {
        self.http
            .request(Method::PUT, &format!("/servers/{server}/ack"), None)
            .await
    }
    pub async fn server_create(
        &self,
        data: &DataCreateServer,
    ) -> Result<CreateServerResponse, DeltaError> {
        self.http
            .request(Method::POST, "/servers/create", json!(data))
            .await
    }
    pub async fn server_delete(&self, server: &str) -> Result<(), DeltaError> {
        self.http
            .request(Method::DELETE, &format!("/servers/{server}"), None)
            .await
    }

    pub async fn server_edit(
        &self,
        server: &str,
        data: &DataEditServer,
    ) -> Result<Server, DeltaError> {
        self.http
            .request(Method::PATCH, &format!("/servers/{server}"), json!(data))
            .await
    }

    pub async fn server_fetch(&self, server: &str) -> Result<Server, DeltaError> {
        self.http
            .request(Method::GET, &format!("/servers/{server}"), None)
            .await
    }

    pub async fn ban_create(
        &self,
        server: &str,
        user: &str,
        reason: Option<&str>,
    ) -> Result<DataBan, DeltaError> {
        self.http
            .request(
                Method::PUT,
                &format!("/servers/{server}/bans/{user}"),
                json!(Into::<DataBanReason>::into(reason)),
            )
            .await
    }
    pub async fn ban_list(&self, server: &str) -> Result<DataBanList, DeltaError> {
        self.http
            .request(Method::GET, &format!("/servers/{server}/bans"), None)
            .await
    }

    pub async fn ban_remove(&self, server: &str, user: &str) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::DELETE,
                &format!("/servers/{server}/bans/{user}"),
                None,
            )
            .await
    }

    pub async fn member_kick(&self, server: &str, user: &str) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::DELETE,
                &format!("/servers/{server}/members/{user}"),
                None,
            )
            .await
    }

    pub async fn channel_create(
        &self,
        server: &str,
        data: &DataChannelCreate,
    ) -> Result<Channel, DeltaError> {
        self.http
            .request(
                Method::POST,
                &format!("/servers/{server}/channels"),
                json!(data),
            )
            .await
    }

    pub async fn invites_fetch(&self, server: &str) -> Result<Vec<Invite>, DeltaError> {
        self.http
            .request(Method::GET, &format!("/servers/{server}/invites"), None)
            .await
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataChannelCreate {
    #[serde(rename = "type")]
    pub r#type: Option<ChannelType>,
    pub name: String,
    pub description: Option<String>,
    pub nsfw: Option<bool>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ChannelType {
    #[default]
    Text,
    Voice,
}

impl DataChannelCreate {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }
    pub fn set_type(&mut self, r#type: ChannelType) -> Self {
        self.r#type = Some(r#type);
        self.to_owned()
    }
    pub fn set_name(&mut self, name: &str) -> Self {
        self.name = String::from(name);
        self.to_owned()
    }
    pub fn set_description(&mut self, description: &str) -> Self {
        self.description = Some(String::from(description));
        self.to_owned()
    }
    pub fn set_nsfw(&mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BannedUser {
    /// Id of the banned user
    #[serde(rename = "_id")]
    pub id: String,
    /// Username of the banned user
    pub username: String,
    /// Avatar of the banned user
    pub avatar: Option<File>,
}

/// # Ban List Result
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataBanList {
    /// Users objects
    users: Vec<BannedUser>,
    /// Ban objects
    bans: Vec<ServerBan>,
}

impl From<User> for BannedUser {
    fn from(user: User) -> Self {
        BannedUser {
            id: user.id,
            username: user.username,
            avatar: user.avatar,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataCreateServer {
    /// Server name
    name: String,
    /// Server description
    description: Option<String>,
    /// Whether this server is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
}

impl DataCreateServer {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }
    pub fn set_name(&mut self, name: &str) -> Self {
        self.name = String::from(name);
        self.to_owned()
    }
    pub fn set_description(&mut self, description: &str) -> Self {
        self.description = Some(String::from(description));
        self.to_owned()
    }
    pub fn set_nsfw(&mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self.to_owned()
    }
}

/// # Create Server Response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateServerResponse {
    /// Server object
    server: Server,
    /// Default channels
    channels: Vec<Channel>,
}

/// # Server Data
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataEditServer {
    /// Server name
    pub name: Option<String>,
    /// Server description
    pub description: Option<String>,
    /// Attachment Id for icon
    pub icon: Option<String>,
    /// Attachment Id for banner
    pub banner: Option<String>,
    /// Category structure for server
    pub categories: Option<Vec<Category>>,
    /// System message configuration
    pub system_messages: Option<SystemMessageChannels>,

    /// Bitfield of server flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,

    // Whether this server is age-restricted
    // nsfw: Option<bool>,
    /// Whether this server is public and should show up on [Revolt Discover](https://rvlt.gg)
    discoverable: Option<bool>,
    /// Whether analytics should be collected for this server
    ///
    /// Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg).
    analytics: Option<bool>,

    /// Fields to remove from server object
    remove: Option<Vec<FieldsServer>>,
}

impl DataEditServer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_name(&mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self.to_owned()
    }
    pub fn set_description(&mut self, description: &str) -> Self {
        self.name = Some(String::from(description));
        self.to_owned()
    }
    pub fn set_icon(&mut self, icon: &str) -> Self {
        self.icon = Some(String::from(icon));
        self.to_owned()
    }
    pub fn set_banner(&mut self, banner: &str) -> Self {
        self.banner = Some(String::from(banner));
        self.to_owned()
    }
    pub fn add_category(&mut self, category: Category) {
        self.categories = opt_vec_add(&self.categories, &category)
    }
    pub fn set_categories(&mut self, categories: Vec<Category>) -> Self {
        self.categories = Some(categories);
        self.to_owned()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataBanReason {
    pub reason: Option<String>,
}

impl From<Option<&str>> for DataBanReason {
    fn from(value: Option<&str>) -> Self {
        Self {
            reason: value.map(|reason| reason.to_string()),
        }
    }
}

impl DataBanReason {
    pub fn new(reason: &str) -> Self {
        Self {
            reason: Some(String::from(reason)),
        }
    }
    pub fn none() -> Self {
        Self { reason: None }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataBan {
    pub _id: BanId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BanId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    pub user: Option<String>,
}
