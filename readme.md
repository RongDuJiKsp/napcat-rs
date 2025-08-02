# NyaCat Bot

NyaCat Bot 是一个基于 `kovi` 和 `napcat` 的猫娘机器人，支持指令执行和智能聊天对话功能。

## 功能特性

- **指令执行**: 支持通过特定指令与机器人进行交互，执行预设操作。
- **智能聊天**: 集成智能聊天模型，能够进行富有上下文的对话。
- **JavaScript Shell**: 内置 JavaScript shell，允许超级用户执行 JS 代码。
- **可配置性**: 灵活的配置选项，包括群组权限、超级用户、API 设置等。
- **记忆管理**: 支持清除机器人记忆，使其恢复初始状态。
- **自动表情攻击**: 支持群聊自动贴表情、表情轰炸等娱乐功能。
- **群聊风控**: 支持违禁词自动禁言/踢人、群邀请风控等。

## 配置文件说明

机器人通过 `JSON` 文件进行配置。以下是主要的配置文件及其结构体说明：

### `chat_config.json` —— 聊天配置

对应结构体：`ChatConfig`

```json5
{
  "allow_groups": [123456789], // 允许机器人响应的群组ID列表
  "model": {
    "key": "your_openai_api_key", // OpenAI API Key
    "endpoint": "https://api.openai.com/v1/", // OpenAI API 地址
    "max_tokens": 2048, // 最大token限制
    "role_model": "gpt-3.5-turbo", // 角色扮演模型名称
    "role_prompt": "你是一只可爱的猫娘，回复时请带上‘喵’字。", // 角色扮演的初始提示
    "role_context_expiration_time_second": 3600, // 角色扮演对话记忆过期时间（秒）
    "role_max_message": 10, // 角色扮演对话窗口大小（最大消息数量）
    "dot_wait_tag": "。", // 机器人大段话拆分时的分隔符
    "dot_wait_time_ms": 1000, // 机器人发送大段话时的停顿时间（毫秒）
    "dot_wait_pre_char_ms": null, // （可选）每段对话长度与此值综合计算停顿时间
    "smart_model": "gpt-4", // 聪明机器人模型名称
    "smart_prompt": "你是一个无所不知的AI助手。", // 聪明机器人的初始提示
  },
}
```

### `command_exec_config.json` —— 指令权限配置

对应结构体：`CommandExecConfig`

```json5
{
  "allow_exec_context": [123456789], // 允许执行指令的群组ID或私聊上下文ID列表
  "allow_super_user": [987654321], // 允许执行特权命令的用户ID列表
  "is_admin_super_user": false, // 群管理员是否为超级用户
  "is_all_user_admin": false, // 是否所有用户都是超级用户
}
```

### `ban_config.json` —— 群聊风控配置

对应结构体：`BanConfig`

```json5
{
  "enable_group": [123456789], // 启用风控的群组ID列表
  "chat_regex_list": ["违禁词1", "违禁词2"], // 触发发言匹配的正则表达式列表
  "enable_chat_shut_up": 3, // 触发禁言的次数阈值（达到后自动禁言）
  "chat_shut_up_time": 3600, // 禁言时长（秒），最大值2332799秒
  "enable_chat_kick": 5, // 触发踢人的次数阈值（达到后自动踢人）
  "enable_invite_ban": { "min_level": 2, "min_activate": 3 }, // 群邀请风控配置
  "enable_invite_kick": 2, // 邀请风控踢人阈值
  "kick_can_request": true, // 被踢后能否再次加群
}
```

#### `InviteBanConfig` 说明

- `min_level`: 邀请者等级低于此值时触发 ban
- `min_activate`: 邀请者群活跃等级低于此值时触发 ban

### `ban_data.json` —— 风控数据

对应结构体：`BanData`

- `chat_action_times`: 每个群内每个人触发违禁词的次数
- `invite_action_times`: 每个群内每个人触发邀请风控的次数

### `emoji_attack_config.json` —— 自动表情攻击配置

对应结构体：`EmojiAttackConfig`

```json5
{
  "allow_monkey_groups": [123456789], // 允许自动表情回应的群号
  "emoji": ["147", "148"], // 自动贴的表情ID列表
  "wait_ms": 300, // 表情贴间隔时间（毫秒），默认300ms
}
```

### `emoji_attack_data.json` —— 自动表情数据

对应结构体：`EmojiAttackData`

- `group_users`: 每个群内自动贴表情的用户列表

## 指令说明

### 通用指令

这些指令可在允许的群组或私聊上下文中使用：

- `$smart [query]`
  - 参数：`query`（可选，字符串）
  - 功能：智能聊天，回复你的问题。若无参数则回复“聪明猫娘在这里喵！”
- `$hi`
  - 无参数，回复“你好喵！我是一只猫娘喵！前面忘了中间忘了，反正我是一只猫娘喵”

### 超级用户指令

这些指令仅超级用户或特定条件下（如群管理员）可用：

- `$shell help`
  - 显示 shell 指令帮助信息。
- `$shell new`
  - 创建新的 JavaScript shell 实例，返回编号。
- `$shell use [shell_id]`
  - 参数：`shell_id`（必填，数字）
  - 切换当前用户绑定的 shell。
- `$shell lock [shell_id]`
  - 参数：`shell_id`（必填，数字）
  - 锁定 shell 为当前用户专属。
- `$shell unlock [shell_id]`
  - 参数：`shell_id`（必填，数字）
  - 解锁 shell。
- `$shell list`
  - 列出所有活跃 shell 实例及所有者。
- `$shell [javascript_code]`
  - 参数：`javascript_code`（必填，字符串）
  - 在当前绑定 shell 执行 JS 代码。
- `$restart`
  - 重新启动机器人服务。
- `$kill`
  - 停止机器人服务。
- `$mem_kill`

  - 清除猫娘记忆。

- `$monkey add [qq_code...] [@at...]`
  - 参数：`qq_code`（可选，数字，支持多个），`@at`（可选，@用户，支持多个）
  - 给指定用户自动贴表情。
- `$monkey del [qq_code...] [@at...]`
  - 移除自动贴表情设置。
- `$monkey clean`
  - 清空自动贴表情设置。
- `$monkey atk [reply]`
  - 参数：`reply`（必填，回复消息 ID）
  - 对某条消息进行表情轰炸。
- `$monkey once [reply]`
  - 参数：`reply`（必填，回复消息 ID）
  - 对某条消息执行一次自动贴表情。

## 进阶说明

- 所有配置项均不支持热更新，修改配置文件后重启机器人即可生效。
- 正则表达式请参考 Rust 标准库语法。
- 表情 ID 可参考 QQ 表情文档或群聊表情包（这里 147，148 是棒棒糖和猴子）。
- 风控相关配置建议根据实际群聊情况调整，避免误伤。

---

如需更多帮助或定制功能，请联系项目维护者。
