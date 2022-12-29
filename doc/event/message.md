# 消息事件 message

## 私聊消息 message.private

| 字段名        | 数据类型 | 说明                         |
| ------------- | -------- | ---------------------------- |
| `message_id`  | String   | 消息唯一 ID                  |
| `message`     | message  | 消息内容                     |
| `alt_message` | String   | 消息内容的替代表示, 可以为空 |
| `user_id`     | String   | 用户 ID                      |

### * 私聊消息添加反应 message.private.reaction_increase

> 无额外增加字段

### * 私聊消息更新 message.private.update

> 无额外增加字段

## 频道消息 message.channel

| 字段名        | 数据类型 | 说明                         |
| ------------- | -------- | ---------------------------- |
| `message_id`  | String   | 消息唯一 ID                  |
| `message`     | message  | 消息内容                     |
| `alt_message` | String   | 消息内容的替代表示, 可以为空 |
| `guild_id`    | String   | 群组 ID                      |
| `channel_id`  | String   | 频道 ID                      |
| `user_id`     | String   | 用户 ID                      |

### * 频道消息添加反应 message.channel.reaction_increase

> 无额外增加字段

### * 私聊消息更新 message.channel.update

> 无额外增加字段