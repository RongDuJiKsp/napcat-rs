use crate::config::BanConfig;
use crate::data::BanData;
use kovi::event::GroupMsgEvent;
use kovi::log::info;
use kovi::{serde_json, RequestEvent, RuntimeBot};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupRequestEvent {
    pub time: i64,    // 事件发生时间戳
    pub self_id: i64, // 接收事件的机器人 QQ 号

    pub post_type: String,    // 固定为 "request"
    pub request_type: String, // 固定为 "group"
    pub sub_type: String,     // "add" 或 "invite"

    pub group_id: i64, // 群号
    pub user_id: i64,  // 发送请求的 QQ 号

    pub comment: String, // 验证信息
    pub flag: String,    // 请求 flag，处理请求时需传入
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupMemberInfo {
    pub group_id: i64,
    pub user_id: i64,
    pub nickname: String,
    pub card: String,
    pub sex: String, // male / female / unknown
    pub age: i32,
    pub area: String,
    pub join_time: i32,
    pub last_sent_time: i32,
    pub level: String,
    pub role: String, // owner / admin / member
    pub unfriendly: bool,
    pub title: String,
    pub title_expire_time: i32,
    pub card_changeable: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    /// 年龄
    pub age: i64,
    /// 头像
    pub avatar: String,
    #[serde(rename = "Business")]
    pub business: Vec<Business>,
    /// 等级
    pub level: i64,
    /// 昵称
    pub nickname: String,
    /// QID
    pub q_id: Option<String>,
    /// 注册时间
    #[serde(rename = "RegisterTime")]
    pub register_time: String,
    /// 性别
    pub sex: String,
    /// 个性签名
    pub sign: String,
    /// 当前状态信息
    pub status: StatusClass,
    /// 用户 Uin
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Business {
    pub icon: Option<String>,
    pub ispro: i64,
    pub isyear: i64,
    pub level: i64,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub business_type: i64,
}

/// 当前状态信息
#[derive(Serialize, Deserialize, Debug)]
pub struct StatusClass {
    /// 表情 ID
    pub face_id: Option<i64>,
    /// 信息
    pub message: Option<String>,
    /// 状态 ID
    pub status_id: i64,
}
pub async fn on_chat(e: Arc<GroupMsgEvent>, bot: Arc<RuntimeBot>) -> Result<(), anyhow::Error> {
    //不是我喜欢的群，直接屏蔽
    if !BanConfig::get().enable_group.contains(&e.group_id) {
        return Ok(());
    }
    //放行
    if !BanConfig::get()
        .chat_regex_list
        .iter()
        .map(|reg| reg.is_match(&e.human_text))
        .fold(false, |p, c| p || c)
    {
        return Ok(());
    }
    let ban_data = BanData::get();
    let mut ban_lock = ban_data.write().await;
    let cnt = ban_lock.chat_action_times.entry(e.group_id).or_default();
    let times = cnt.entry(e.user_id).or_default();
    *times += 1;
    if BanConfig::get()
        .enable_chat_kick
        .map(|val| *times >= val)
        .unwrap_or(false)
    {
        bot.set_group_kick(
            e.group_id,
            e.user_id,
            BanConfig::get().kick_can_request_or_default(),
        );
        e.reply_and_quote(format!(
            "用户{} 因为触发违禁词达到次数{} 已被踢出！！如需申诉请联系管理员",
            e.user_id, *times
        ));
        //身死债消
        cnt.remove(&e.user_id);
    } else if BanConfig::get()
        .enable_chat_shut_up
        .map(|val| *times >= val)
        .unwrap_or(false)
    {
        bot.set_group_ban(
            e.group_id,
            e.user_id,
            BanConfig::get().chat_shut_up_duration().as_secs() as usize,
        );
        e.reply_and_quote(format!(
            "用户{} 因为触发违禁词达到次数{} 已被封禁{}s！如需申诉请联系管理员",
            e.user_id,
            *times,
            BanConfig::get().chat_shut_up_duration().as_secs()
        ));
    } else if BanConfig::get().enable_chat_kick.is_some()
        || BanConfig::get().enable_chat_shut_up.is_some()
    {
        e.reply_and_quote("爆了")
    }

    Ok(())
}
pub async fn on_request(e: Arc<RequestEvent>, bot: Arc<RuntimeBot>) -> Result<(), anyhow::Error> {
    //这里只处理群内邀请请求，即request_type=group,sub_type=invite
    if e.request_type != "group" {
        return Ok(());
    }
    let group_request = serde_json::from_value::<GroupRequestEvent>(e.original_json.clone())
        .map_err(|e| anyhow::anyhow!("Fail to serde json on GroupRequestEvent:{:?}", e))?;
    if group_request.sub_type != "invite" {
        return Ok(());
    }
    //不是我喜欢的群，直接屏蔽
    if !BanConfig::get()
        .enable_group
        .contains(&group_request.group_id)
    {
        return Ok(());
    }
    //先查member data 防止退群了
    let member_data = serde_json::from_value::<GroupMemberInfo>(
        bot.get_group_member_info(group_request.group_id, group_request.user_id, false)
            .await
            .map_err(|e| anyhow::anyhow!("Fail to get group member:{:?}", e))?
            .data,
    )
        .map_err(|e| anyhow::anyhow!("Fail to de_serde group member:{:?}", e))?;
    info!("{:?}", member_data);
    if BanConfig::get()
        .enable_invite_ban
        .as_ref()
        .and_then(|c| c.min_activate)
        .and_then(|e| {
            member_data
                .level
                .parse::<i32>()
                .ok()
                .map(|level| level >= e)
        })
        .unwrap_or(true)
    {
        return Ok(());
    }
    let user_data = serde_json::from_value::<UserInfo>(
        bot.get_stranger_info(group_request.user_id, false)
            .await
            .map_err(|e| anyhow::anyhow!("Fail to get user member:{:?}", e))?
            .data,
    )
        .map_err(|e| anyhow::anyhow!("Fail to de_serde user member:{:?}", e))?;
    info!("{:?}", user_data);
    if BanConfig::get()
        .enable_invite_ban
        .as_ref()
        .and_then(|c| c.min_level)
        .map(|e| user_data.level >= e as i64)
        .unwrap_or(true)
    {
        return Ok(());
    }
    let ban_data = BanData::get();
    let mut ban_lock = ban_data.write().await;
    let cnt = ban_lock
        .invite_action_times
        .entry(group_request.group_id)
        .or_default();
    let times = cnt.entry(group_request.user_id).or_default();
    *times += 1;
    if BanConfig::get()
        .enable_invite_kick
        .map(|e| *times >= e)
        .unwrap_or(false)
    {
        bot.set_group_kick(
            group_request.group_id,
            group_request.user_id,
            BanConfig::get().kick_can_request_or_default(),
        );
        bot.send_group_msg(
            group_request.group_id,
            format!(
                "用户{} 因为邀请群成员达到次数{} 已被踢出！如需申诉请联系管理员",
                group_request.user_id, *times,
            ),
        );
    } else if BanConfig::get().enable_invite_kick.is_some() {
        bot.send_group_msg(
            group_request.group_id,
            format!("用户{} 邀请群成员达到次数{}", group_request.user_id, *times, ),
        );
    }
    Ok(())
}
