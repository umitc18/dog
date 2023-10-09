use figment::{Figment, providers::{Format, Json, Env}};
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct WebhookChannel {
    pub webhook: String,
    pub channel: serenity::ChannelId,
}

impl WebhookChannel {
    pub async fn webhook(&self, http: impl AsRef<serenity::Http>) -> serenity::Result<serenity::Webhook> {
        serenity::Webhook::from_url(http, &self.webhook).await
    }
}

#[derive(Deserialize)]
pub struct LogChannels {
    pub system: serenity::ChannelId,
    pub member: serenity::ChannelId,
}

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub warns: Vec<serenity::RoleId>,
    pub boards: HashMap<String, WebhookChannel>,
    pub logs: LogChannels,
    pub admins: Vec<serenity::UserId>,
}

impl Config {
    pub fn load() -> figment::error::Result<Self> {
        Figment::new()
            .merge(Json::file("Config.json"))
            .merge(Env::prefixed("DOG_"))
            .extract()
    }
}
