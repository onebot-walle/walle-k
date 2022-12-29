## 通知事件 notice

### * 私聊消息删除反应 notice.reaction_decrease.private

| 字段名       | 数据类型 | 说明                |
| ------------ | -------- | ------------------- |
| `message_id` | String   | 消息唯一 ID         |
| `emoji_id`   | String   | reaction emoji id   |
| `emoji_name` | String   | reaction emoji name |
| `user_id`    | String   | 用户 ID             |

### * 频道消息删除反应 notice.reaction_decrease.channel

| 字段名       | 数据类型 | 说明                |
| ------------ | -------- | ------------------- |
| `message_id` | String   | 消息唯一 ID         |
| `emoji_id`   | String   | reaction emoji id   |
| `emoji_name` | String   | reaction emoji name |
| `guild_id`   | String   | 群组 ID             |
| `channel_id` | String   | 频道 ID             |
| `user_id`    | String   | 用户 ID             |