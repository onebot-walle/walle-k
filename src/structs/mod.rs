use walle_core::{
    prelude::{PushToValueMap, ToMsgSegment, TryFromMsgSegment},
    segment::MsgSegment,
    util::OneBotBytes,
};

#[derive(Debug, ToMsgSegment, PushToValueMap, TryFromMsgSegment)]
pub enum WKSegment {
    Text {
        text: String,
        style: Option<String>,
        sub_segments: Option<Vec<MsgSegment>>,
        url: Option<String>,
    },
    Mention {
        user_id: String,
        ty: Option<String>,
    },
    MentionAll,
    Face {
        id: String,
        name: Option<String>,
    },
    Image {
        file_id: String,
        url: Option<String>,
        bytes: Option<OneBotBytes>,
    },
    Video {
        file_id: String,
        url: Option<String>,
        bytes: Option<OneBotBytes>,
    },
    File {
        file_id: String,
        url: Option<String>,
        bytes: Option<OneBotBytes>,
    },
}
