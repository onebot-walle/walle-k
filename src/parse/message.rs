use std::sync::Arc;

use kook::{kmd_from_str, prelude::AssetUrl, KMDItem, Kook, MessageType};
use walle_core::{
    error::WalleResult,
    prelude::{PushToValueMap, ToMsgSegment},
    resp::{resp_error, RespError},
    segment::{Image, MsgSegment, Segments, Text},
    util::OneBotBytes,
    value_map, WalleError,
};

use crate::structs::WKSegment;

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
                "file_id": s,
                "url": s
            },
        }]),
        3 => Ok(vec![MsgSegment {
            ty: "video".to_owned(),
            data: value_map! {
                "file_id": s,
                "url": s
            },
        }]),
        4 => Ok(vec![MsgSegment {
            ty: "file".to_owned(),
            data: value_map! {
                "file_id": s,
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
    v.into_iter().map(kmd_to_segment).collect()
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
        KMDItem::Mention(user_id) if user_id.as_str() == "all" => MsgSegment {
            ty: "mention_all".to_owned(),
            data: value_map! {},
        },
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
        KMDItem::Code { ty: _, content } => MsgSegment {
            ty: "text".to_owned(),
            data: value_map! {
                "text": content,
                "style": "code"
                // "language": ty
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

fn segments_parse(segments: Segments) -> Result<Vec<WKSegment>, RespError> {
    segments
        .into_iter()
        .map(|seg| WKSegment::try_from(seg))
        .collect::<Result<Vec<WKSegment>, WalleError>>()
        .map_err(|e| resp_error::bad_segment_data(e))
}

pub async fn segments_to_str(
    kook: &Arc<Kook>,
    segments: Segments,
) -> Result<(String, MessageType), RespError> {
    let mut segments = segments_parse(segments)?;
    let (mut images, video, file) = segments_extrac(kook, &mut segments).await?;
    if images.len() == 1 {
        return Ok((images.remove(0), MessageType::Image));
    } else if !images.is_empty() {
        todo!()
    }
    if !video.is_empty() {
        return Ok((video, MessageType::Video));
    }
    if !file.is_empty() {
        return Ok((file, MessageType::File));
    }
    let kmds = segments_to_kmd(segments)?;
    Ok((
        kmds.into_iter().map(|kmd| kmd.to_string()).collect(),
        MessageType::KMarkdown,
    ))
}

pub fn segments_to_kmd(segments: Vec<WKSegment>) -> Result<Vec<KMDItem>, RespError> {
    let mut kmds = vec![];
    for seg in segments {
        match seg {
            WKSegment::Text {
                text,
                style,
                sub_segments,
                url,
            } => {
                if let Some(sub) = sub_segments {
                    match style.as_deref() {
                        Some("bold") => {
                            kmds.push(KMDItem::Blod(segments_to_kmd(segments_parse(sub)?)?))
                        }
                        Some("italic") => {
                            kmds.push(KMDItem::Italic(segments_to_kmd(segments_parse(sub)?)?))
                        }
                        Some("deleted") => {
                            kmds.push(KMDItem::Deleted(segments_to_kmd(segments_parse(sub)?)?))
                        }
                        Some("link") => {
                            if let Some(url) = url {
                                kmds.push(KMDItem::Link { text, url });
                                continue;
                            }
                        }
                        Some("divider") => kmds.push(KMDItem::Divider),
                        Some("ref") => {
                            kmds.push(KMDItem::Ref(segments_to_kmd(segments_parse(sub)?)?))
                        }
                        Some("ubderline") => {
                            kmds.push(KMDItem::Underline(segments_to_kmd(segments_parse(sub)?)?))
                        }
                        Some("spoiler") => {
                            kmds.push(KMDItem::Spoiler(segments_to_kmd(segments_parse(sub)?)?))
                        }
                        Some("inline_code") => kmds.push(KMDItem::InlineCode(text)),
                        Some("code") => kmds.push(KMDItem::Code {
                            ty: "".to_owned(),
                            content: text,
                        }),
                        Some("k_markdown") => kmds.extend(kmd_from_str(&text)),
                        _ => kmds.push(KMDItem::Text(text)),
                    }
                    continue;
                }
                kmds.push(KMDItem::Text(text));
            }
            WKSegment::Mention { user_id, ty } => match ty.as_deref() {
                Some("channel") => kmds.push(KMDItem::Channel(user_id)),
                Some("role") => kmds.push(KMDItem::Role(user_id)),
                _ => kmds.push(KMDItem::Mention(user_id)),
            },
            WKSegment::MentionAll => kmds.push(KMDItem::Mention("all".to_owned())),
            WKSegment::Face { id, name } => kmds.push(KMDItem::Emoji(id, name)),
            _ => {}
        }
    }
    Ok(kmds)
}

async fn segments_extrac(
    kook: &Arc<Kook>,
    segments: &mut Vec<WKSegment>,
) -> Result<(Vec<String>, String, String), RespError> {
    let mut index = 0;
    let mut vs = (vec![], String::default(), String::default());
    while index < segments.len() {
        macro_rules! if_chain {
            ($item: tt, $index: tt, $content: expr) => {
                if let Some(WKSegment::$item { .. }) = segments.get(index) {
                    let WKSegment::$item {
                                        file_id,
                                        url,
                                        bytes,
                                    } = segments.remove(index) else { unreachable!() };
                    if !file_id.is_empty() {
                        vs.$index = file_id;
                    } else if let Some(url) = url {
                        vs.$index = url;
                    } else if let Some(OneBotBytes(v)) = bytes {
                        let AssetUrl { url } = kook
                            .create_asset(v, $content, if file_id.is_empty() { "-" } else { &file_id })
                            .await
                            .map_err(|e| resp_error::bad_handler(e))?;
                        vs.$index = url;
                    }
                    continue;
                }
            };
        }
        if let Some(WKSegment::Image { .. }) = segments.get(index) {
            let WKSegment::Image {
                                file_id,
                                url,
                                bytes,
                            } = segments.remove(index) else { unreachable!() };
            if !file_id.is_empty() {
                vs.0.push(file_id)
            } else if let Some(url) = url {
                vs.0.push(url)
            } else if let Some(OneBotBytes(v)) = bytes {
                let AssetUrl { url } = kook
                    .create_asset(
                        v,
                        "image/png",
                        if file_id.is_empty() { "-" } else { &file_id },
                    )
                    .await
                    .map_err(|e| {
                        println!("{}", e);
                        resp_error::bad_handler(e)
                    })?;
                vs.0.push(url)
            }
            continue;
        }
        if_chain!(Video, 1, "video/mpeg4");
        if_chain!(File, 2, "application/octet-stream");
        index += 1
    }
    Ok(vs)
}
