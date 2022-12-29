use kook::prelude::{Event as _KookEvent, EventExtra, SystemExtra};
use walle_core::{event::Event, segment::MessageExt, structs::Selft, value_map};

mod action;
mod message;
pub use action::KookAction;
pub use message::*;

type KookEvent = _KookEvent<EventExtra>;

pub async fn event_parse(event: KookEvent, user_id: String) -> Option<Event> {
    let selft = Selft {
        user_id,
        platform: crate::WALLE_K.to_owned(),
    };
    match event.extra {
        EventExtra::GroupMessage(g) => {
            let segs = match kook_message_to_segment(&event.content, event.ty) {
                Ok(segs) => segs,
                Err(e) => {
                    tracing::warn!(target: crate::WALLE_K, "parse segs error:{}", e);
                    return None;
                }
            };
            Some(Event {
                id: event.msg_id.clone(),
                time: event.msg_timestamp as f64,
                ty: "message".to_owned(),
                detail_type: "channel".to_owned(),
                sub_type: "".to_owned(),
                extra: value_map! {
                    "message_id": event.msg_id,
                    "alt_message": segs.extract_plain_text(),
                    "message": segs,
                    "guild_id": g.guild_id,
                    "channel_id": event.target_id,
                    "user_id": g.author.id,
                    "self": selft
                },
            })
        }
        EventExtra::PersonMessage(p) => {
            let segs = match kook_message_to_segment(&event.content, event.ty) {
                Ok(segs) => segs,
                Err(e) => {
                    tracing::warn!(target: crate::WALLE_K, "parse segs error:{}", e);
                    return None;
                }
            };
            Some(Event {
                id: event.msg_id.clone(),
                time: event.msg_timestamp as f64,
                ty: "message".to_owned(),
                detail_type: "private".to_owned(),
                sub_type: "".to_owned(),
                extra: value_map! {
                    "message_id": event.msg_id,
                    "alt_message": segs.extract_plain_text(),
                    "message": segs,
                    "user_id": p.author.id,
                    "self": selft
                },
            })
        }
        EventExtra::System(SystemExtra::AddedReaction {
            channel_id,
            emoji,
            user_id,
            msg_id,
        }) => Some(Event {
            id: event.msg_id,
            time: event.msg_timestamp as f64,
            ty: "message".to_owned(),
            detail_type: "channel".to_owned(),
            sub_type: "add_reaction".to_owned(),
            extra: value_map! {
                "message_id": msg_id,
                "alt_message": emoji.name,
                "message": [{
                    "type": "face",
                    "data": {
                        "id": emoji.id,
                        "name": emoji.name
                    }
                }],
                "guild_id": event.target_id,
                "channel_id": channel_id,
                "user_id": user_id,
                "self": selft
            },
        }),
        EventExtra::System(SystemExtra::DeletedReaction {
            channel_id,
            emoji,
            user_id,
            msg_id,
        }) => Some(Event {
            id: event.msg_id,
            time: event.msg_timestamp as f64,
            ty: "message".to_owned(),
            detail_type: "channel".to_owned(),
            sub_type: "delete_reaction".to_owned(),
            extra: value_map! {
                "message_id": msg_id,
                "alt_message": emoji.name,
                "message": [{
                    "type": "face",
                    "data": {
                        "id": emoji.id,
                        "name": emoji.name
                    }
                }],
                "guild_id": event.target_id,
                "channel_id": channel_id,
                "user_id": user_id,
                "self": selft
            },
        }),
        _ => None,
    }
}
