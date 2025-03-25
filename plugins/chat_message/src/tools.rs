use anyhow::anyhow;
use kovi::serde_json;
use kovi::{MsgEvent, RuntimeBot};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberInfo {
    pub group_id: i64,
    pub user_id: i64,
    pub nickname: String,
    pub card: String,
    pub sex: String, // "male", "female", or "unknown"
    pub age: i32,
    pub area: String,
    pub join_time: i32,
    pub last_sent_time: i32,
    pub level: String,
    pub role: String, // "owner", "admin", or "member"
    pub unfriendly: bool,
    pub title: String,
    pub title_expire_time: i32,
    pub card_changeable: bool,
}
pub async fn self_bot_info(
    bot: &RuntimeBot,
    event: &MsgEvent,
) -> Result<MemberInfo, Box<dyn Error>> {
    Ok(serde_json::from_value(
        bot.get_group_member_info(
            event
                .group_id
                .ok_or(anyhow::anyhow!("bot_info not found"))?,
            event.self_id,
            false,
        )
        .await
        .map_err(|e| anyhow!("{}", e.data.to_string()))?
        .data,
    )?)
}
