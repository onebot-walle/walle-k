## 消息段

### 文本消息段 text

| 字段           | 类型              | 备注                                                                                                  |
| -------------- | ----------------- | ----------------------------------------------------------------------------------------------------- |
| text           | String            | 文本内容                                                                                              |
| * style        | String            | bold \| italic \| deleted \| ref \| underline \|<br>spoiler \| divider \| inline_code \| code \| link |
| * sub_segments | Vec\<MsgSegment\> | KMarkdown子字段                                                                                       |
| * url          | Option\<String\>  | link url                                                                                              |

### 图片消息段 image

| 字段     | 类型   | 备注            |
| -------- | ------ | --------------- |
| file_id  | String | 由图片 url 代替 |
| * url    | String | 同上            |
| ** bytes | Bytes  | 仅发送时支持    |

> 视频消息段 video | 文件消息段 file 与图片消息段雷同，不再赘述

### * 表情消息段 face

| 字段 | 类型             | 备注                            |
| ---- | ---------------- | ------------------------------- |
| id   | String           | emoji shortcode 或者频道表情 id |
| name | Option\<String\> | 频道表情名称                    |

### 提及消息段 mention

| 字段    | 类型             | 备注            |
| ------- | ---------------- | --------------- |
| user_id | String           | -               |
| * type  | Option\<String\> | role \| channel |