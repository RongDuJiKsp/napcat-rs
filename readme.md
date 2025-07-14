# NyaCat Bot

NyaCat Bot 是一个基于 `kovi` 和 `napcat` 的猫娘机器人，支持指令执行和智能聊天对话功能。

## 功能特性

* **指令执行**: 支持通过特定指令与机器人进行交互，执行预设操作。
* **智能聊天**: 集成智能聊天模型，能够进行富有上下文的对话。
* **JavaScript Shell**: 内置 JavaScript shell，允许超级用户执行 JS 代码。
* **可配置性**: 灵活的配置选项，包括群组权限、超级用户、API 设置等。
* **记忆管理**: 支持清除机器人记忆，使其恢复初始状态。

## 配置

机器人通过 `JSON` 文件进行配置。以下是主要的配置文件及其说明：

### `chat_config.json`

此文件配置与聊天模型相关的参数。

```json5
{
  "allow_groups": [],
  // 允许机器人响应的群组ID列表
  "model": {
    "key": "your_openai_api_key",
    // OpenAI API Key
    "endpoint": "[https://api.openai.com/v1/](https://api.openai.com/v1/)",
    // OpenAI API 地址
    "max_tokens": 2048,
    // 最大token限制
    "role_model": "gpt-3.5-turbo",
    // 角色扮演模型名称
    "role_prompt": "你是一只可爱的猫娘，回复时请带上“喵”字。",
    // 角色扮演的初始提示
    "role_context_expiration_time_second": 3600,
    // 角色扮演对话记忆过期时间（秒）
    "role_max_message": 10,
    // 角色扮演对话窗口大小（最大消息数量）
    "dot_wait_tag": "。",
    // 机器人大段话拆分时的分隔符
    "dot_wait_time_ms": 1000,
    // 机器人发送大段话时的停顿时间（毫秒）
    "dot_wait_pre_char_ms": null,
    // （可选）每段对话长度与此值综合计算停顿时间
    "smart_model": "gpt-4",
    // 聪明机器人模型名称
    "smart_prompt": "你是一个无所不知的AI助手。"
    // 聪明机器人的初始提示
  }
}
```

### `command_exec_config.json`

此文件配置指令执行的权限和超级用户设置。

```json5
{
  "allow_exec_groups": [],
  // 允许执行指令的群组ID或私聊上下文ID列表
  "allow_super_user": [],
  // 允许执行特权命令的用户ID列表
  "is_admin_super_user": false,
  // 群管理员是否为超级用户
  "is_all_user_admin": false
  // 是否所有用户都是超级用户
}
```
### `emoji_attack_config.json`

此文件配置自动表情回应的设置
```json5
{
  "allow_monkey_groups": [
    459613872
  ],
  //允许自动表情回应的群号
  "emoji": [
    "147"
  ],
  //自动贴的表情 这里是棒棒糖 id可以去相关文档找
  "wait_ms": 30,
  //防抖时间 默认30
}
```

## NyaCat Bot 支持以下指令：

### 通用指令

这些指令可以在允许执行的群组或私聊上下文中使用。

* `$smart [query]`: 启动智能聊天模式，机器人将回复您的查询。如果未提供 `[query]`，机器人会回复 "聪明猫娘在这里喵！".
* `$hi`: 机器人会回复 "你好喵！我是一只猫娘喵！前面忘了中间忘了，反正我是一只猫娘喵".

### 超级用户指令

这些指令只有被配置为超级用户或在特定条件下（如群管理员）的用户才能使用。

* `$shell help`: 显示 `$shell` 指令的帮助信息.
* `$shell new`: 创建一个新的 JavaScript shell 实例，并返回其编号.
* `$shell use [shell_id]`: 切换当前用户绑定的 shell 为指定编号的 shell.
* `$shell lock [shell_id]`: 将指定编号的 shell 锁定给当前用户，使其成为该用户的专属 shell.
* `$shell unlock [shell_id]`: 解锁指定编号的 shell，使其不再是当前用户的专属 shell.
* `$shell list`: 列出所有活跃的 shell 实例及其所有者.
* `$shell [javascript_code]`: 在当前绑定的 JavaScript shell 中执行 JavaScript 代码.
* `$restart`: 重新启动机器人服务，使其恢复响应.
* `$kill`: 停止机器人服务，使其不再响应指令和消息.
* `$mem_kill`: 清除猫娘的记忆.
* `$monkey add [qq_code]`: 在这个群内给qq号对应的人自动贴表情.
* `$monkey del [qq_code]`: 移除前面的自动设置.