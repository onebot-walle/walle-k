use kook::{kmd_from_str, KMDItem, MessageType};
use walle_core::{
    error::WalleResult,
    prelude::{PushToValueMap, ToMsgSegment},
    resp::RespError,
    segment::{Image, MessageExt, MsgSegment, Segments, Text},
    value_map,
};

#[derive(Debug, ToMsgSegment, PushToValueMap)]
pub enum KookSegment {
    Text(Text),
    Image(Image),
}

pub fn kook_message_to_segment(s: &str, ty: u8) -> WalleResult<Segments> {
    match ty {
        1 => Ok(vec![MsgSegment {
            ty: "text".to_owned(),
            data: value_map! {
                "text": s
            },
        }]),
        2 => Ok(vec![MsgSegment {
            ty: "image".to_owned(),
            data: value_map! {
                "file_id": "",
                "url": s
            },
        }]),
        3 => Ok(vec![MsgSegment {
            ty: "video".to_owned(),
            data: value_map! {
                "file_id": "",
                "url": s
            },
        }]),
        4 => Ok(vec![MsgSegment {
            ty: "file".to_owned(),
            data: value_map! {
                "file_id": "",
                "url": s
            },
        }]),
        8 => todo!(), // audio
        9 => Ok(kmds_to_segments(kmd_from_str(s))),
        10 => todo!(), //card
        _ => unreachable!(),
    }
}

pub fn kmds_to_segments(v: Vec<KMDItem>) -> Segments {
    v.into_iter().map(|i| kmd_to_segment(i)).collect()
}

fn kmds_to_plait_text(v: &Vec<KMDItem>) -> String {
    v.into_iter().map(|i| i.plain_text()).collect()
}

pub fn kmd_to_segment(item: KMDItem) -> MsgSegment {
    fn text(v: Vec<KMDItem>, style: &str) -> MsgSegment {
        MsgSegment {
            ty: "text".to_owned(),
            data: value_map! {
                "text": kmds_to_plait_text(&v),
                "style": style,
                "sub_segments": kmds_to_segments(v)
            },
        }
    }
    fn mention(s: String, ty: &str) -> MsgSegment {
        MsgSegment {
            ty: "mention".to_owned(),
            data: value_map! {
                "user_id": s,
                "type": ty
            },
        }
    }
    match item {
        KMDItem::Text(s) => MsgSegment {
            ty: "text".to_owned(),
            data: value_map! {
                "text": s
            },
        },
        KMDItem::Blod(v) => text(v, "blod"),
        KMDItem::Italic(v) => text(v, "italic"),
        KMDItem::Deleted(v) => text(v, "deleted"),
        KMDItem::Link { text, url } => MsgSegment {
            ty: "text".to_owned(),
            data: value_map! {
                "text": text,
                "style": "link",
                "url": url
            },
        },
        KMDItem::Divider => MsgSegment {
            ty: "text".to_owned(),
            data: value_map! {
                "text": "---",
                "style": "divider"
            },
        },
        KMDItem::Ref(v) => text(v, "ref"),
        KMDItem::Underline(v) => text(v, "underline"),
        KMDItem::Spoiler(v) => text(v, "spoiler"),
        KMDItem::Emoji(s, None) => MsgSegment {
            ty: "face".to_owned(),
            data: value_map! {
                "id": s
            },
        },
        KMDItem::Emoji(s, Some(id)) => MsgSegment {
            ty: "face".to_owned(),
            data: value_map! {
                "id": id,
                "name": s
            },
        },
        KMDItem::Channel(id) => mention(id, "channel"),
        KMDItem::Mention(user_id) => MsgSegment {
            ty: "mention".to_owned(),
            data: value_map! {
                "user_id": user_id
            },
        },
        KMDItem::Role(id) => mention(id, "role"),
        KMDItem::InlineCode(s) => MsgSegment {
            ty: "text".to_owned(),
            data: value_map! {
                "text": s,
                "style": "inline_code"
            },
        },
        KMDItem::Code { ty, content } => MsgSegment {
            ty: "text".to_owned(),
            data: value_map! {
                "text": content,
                "style": "code",
                "langage": ty
            },
        },
        KMDItem::NewLine => MsgSegment {
            ty: "text".to_owned(),
            data: value_map! {
                "text": "\n"
            },
        },
    }
}

pub fn segments_to_kmd(segments: Segments) -> Result<(String, MessageType), RespError> {
    // if segments.len() == 1 {
    //     match KookSegment::try_from(segments.pop().unwrap()) {
    //         Ok(segment) => match segment {
    //             KookSegment::Text(t) => return Ok((t.text, MessageType::Text)),
    //             KookSegment::Image(i) => return Ok((i.file_id, MessageType::Image)), //todo
    //         },
    //         Err(e) => return Err(resp_error::unsupported_segment(e.to_string())),
    //     }
    // }
    Ok((segments.extract_plain_text(), MessageType::Text))
}
